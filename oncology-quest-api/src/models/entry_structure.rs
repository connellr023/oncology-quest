use crate::query_many;
use crate::utilities::parsable::EntryTitle;
use std::collections::HashMap;
use sqlx::{FromRow, PgPool};
use serde::Serialize;

#[derive(Debug, FromRow, Clone, Serialize)]
pub struct Supertask {
    id: i32,
    title: EntryTitle,
    rotation_id: i32,
}

#[derive(Debug, FromRow, Clone, Serialize)]
pub struct Task {
    id: i32,
    supertask_id: i32,
    title: EntryTitle,
    rotation_id: i32
}

#[derive(Debug, FromRow, Clone, Serialize)]
pub struct Subtask {
    id: i32,
    task_id: i32,
    title: EntryTitle,
    rotation_id: i32
}

/// Represents a level of entries, with an entry and its children.
/// 
/// # Type Parameters
/// 
/// * `T` - The type of the entry.
/// * `U` - The type of the children.
#[derive(Serialize, Debug)]
pub struct EntryLevel<T: Sized, U: Sized> {
    entry: T,
    children: Box<[U]>
}

type EntryHierarchy = EntryLevel<Supertask, EntryLevel<Task, Subtask>>;

/// Represents the hierarchal structure of entries, with supertasks at the top level, tasks at the second level, and subtasks at the third level.
#[derive(Serialize, Debug)]
pub struct EntryStructure(Vec<EntryHierarchy>);

macro_rules! fetch_all {
    ($struct_name:ident, $table_name:literal) => {
        pub async fn fetch_all(pool: &PgPool, rotation_id: i32) -> anyhow::Result<Box<[Self]>> {
            let records = sqlx::query_as!(
                $struct_name,
                "SELECT * FROM " + $table_name + " WHERE rotation_id = $1 ORDER BY id;",
                rotation_id
            )
            .fetch_all(pool)
            .await?;

            Ok(records.into_boxed_slice())
        }
    };
}

macro_rules! update_title {
    ($struct_name:ident, $table_name:literal) => {
        pub async fn update_title(pool: &PgPool, id: i32, title: &str) -> anyhow::Result<()> {
            let mut transaction = pool.begin().await?;
            
            sqlx::query!(
                "UPDATE " + $table_name + " SET title = $1 WHERE id = $2;",
                title,
                id
            )
            .execute(&mut *transaction)
            .await?;

            sqlx::query!(
                "UPDATE rotations SET last_updated = NOW() WHERE id = (SELECT rotation_id FROM " + $table_name + " WHERE id = $1);",
                id
            )
            .execute(&mut *transaction)
            .await?;

            transaction.commit().await?;

            Ok(())
        }
    };
}

impl Supertask {
    fetch_all!(Supertask, "supertasks");
    update_title!(Supertask, "supertasks");

    pub async fn insert_from(pool: &PgPool, title: &str, rotation_id: i32) -> anyhow::Result<i32> {
        let mut transaction = pool.begin().await?;

        sqlx::query!(
            r#"
            UPDATE rotations SET last_updated = NOW() WHERE id = $1;
            "#,
            rotation_id
        )
        .execute(&mut *transaction)
        .await?;

        let row = sqlx::query!(
            r#"
            INSERT INTO supertasks (title, rotation_id) VALUES ($1, $2) RETURNING id;
            "#,
            title,
            rotation_id
        )
        .fetch_one(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(row.id)
    }

    pub async fn delete(pool: &PgPool, id: i32) -> anyhow::Result<()> {
        let mut transaction = pool.begin().await?;

        query_many!(&mut *transaction, id,
            "DELETE FROM subtasks WHERE task_id IN (SELECT id FROM tasks WHERE supertask_id = $1)",
            "DELETE FROM tasks WHERE supertask_id = $1",
            "DELETE FROM supertasks WHERE id = $1",
            "UPDATE rotations SET last_updated = NOW() WHERE id = (SELECT rotation_id FROM supertasks WHERE id = $1);"
        );

        transaction.commit().await?;

        Ok(())
    }
}

impl Task {
    fetch_all!(Task, "tasks");
    update_title!(Task, "tasks");

    pub async fn insert_from(pool: &PgPool, title: &str, rotation_id: i32, supertask_id: i32) -> anyhow::Result<i32> {
        let mut transaction = pool.begin().await?;

        sqlx::query!(
            r#"
            UPDATE rotations SET last_updated = NOW()
            WHERE id = $1;
            "#,
            rotation_id
        )
        .execute(&mut *transaction)
        .await?;

        let row = sqlx::query!(
            r#"
            INSERT INTO tasks (title, rotation_id, supertask_id) VALUES ($1, $2, $3)
            RETURNING id;
            "#,
            title,
            rotation_id,
            supertask_id
        )
        .fetch_one(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(row.id)
    }

    pub async fn delete(pool: &PgPool, id: i32) -> anyhow::Result<()> {
        let mut transaction = pool.begin().await?;

        query_many!(&mut *transaction, id,
            "DELETE FROM subtasks WHERE task_id = $1",
            "DELETE FROM tasks WHERE id = $1",
            "UPDATE rotations SET last_updated = NOW() WHERE id = (SELECT rotation_id FROM tasks WHERE id = $1);"
        );

        transaction.commit().await?;

        Ok(())
    }
}

impl Subtask {
    fetch_all!(Subtask, "subtasks");
    update_title!(Subtask, "subtasks");

    pub async fn insert_from(pool: &PgPool, title: &str, rotation_id: i32, task_id: i32) -> anyhow::Result<i32> {
        let mut transaction = pool.begin().await?;

        sqlx::query!(
            r#"
            UPDATE rotations SET last_updated = NOW()
            WHERE id = $1;
            "#,
            rotation_id
        )
        .execute(&mut *transaction)
        .await?;

        let row = sqlx::query!(
            r#"
            INSERT INTO subtasks (title, rotation_id, task_id) VALUES ($1, $2, $3)
            RETURNING id;
            "#,
            title,
            rotation_id,
            task_id
        )
        .fetch_one(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(row.id)
    }

    pub async fn delete(pool: &PgPool, id: i32) -> anyhow::Result<()> {
        let mut transaction = pool.begin().await?;

        query_many!(&mut *transaction, id,
            "DELETE FROM subtasks WHERE id = $1",
            "UPDATE rotations SET last_updated = NOW() WHERE id = (SELECT rotation_id FROM subtasks WHERE id = $1);"
        );

        transaction.commit().await?;

        Ok(())
    }
}

impl EntryStructure {
    /// Builds an entry structure from an unordered collection of supertasks, tasks, and subtasks.
    /// 
    /// # Parameters
    /// 
    /// * `supertasks` - The supertasks.
    /// * `tasks` - The tasks.
    /// * `subtasks` - The subtasks.
    /// 
    /// # Returns
    /// 
    /// The entry structure.
    pub fn build(supertasks: &[Supertask], tasks: &[Task], subtasks: &[Subtask]) -> anyhow::Result<Self> {
        let mut task_map: HashMap<i32, Vec<Subtask>> = HashMap::new();
        for subtask in subtasks.iter() {
            task_map
                .entry(subtask.task_id)
                .or_default()
                .push(subtask.to_owned());
        }

        let mut supertask_map: HashMap<i32, Vec<EntryLevel<Task, Subtask>>> = HashMap::new();
        for task in tasks.iter() {
            let subtasks = task_map
                .remove(&task.id)
                .unwrap_or_default();

            let entry_level = EntryLevel {
                entry: task.to_owned(),
                children: subtasks.into_boxed_slice()
            };

            supertask_map
                .entry(task.supertask_id)
                .or_default()
                .push(entry_level);
        }

        let mut entry_structure = Vec::with_capacity(supertasks.len());
        for supertask in supertasks.iter() {
            let entry_levels = supertask_map
                .remove(&supertask.id)
                .unwrap_or_default();

            let entry_hierarchy = EntryHierarchy {
                entry: supertask.to_owned(),
                children: entry_levels.into_boxed_slice()
            };

            entry_structure.push(entry_hierarchy);
        }

        Ok(Self(entry_structure))
    }

    pub async fn fetch(pool: &PgPool, rotation_id: i32) -> anyhow::Result<Self> {
        let supertasks = Supertask::fetch_all(pool, rotation_id).await?;
        let tasks = Task::fetch_all(pool, rotation_id).await?;
        let subtasks = Subtask::fetch_all(pool, rotation_id).await?;

        let structure = Self::build(&supertasks, &tasks, &subtasks)?;
        Ok(structure)
    }
}