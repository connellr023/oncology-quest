use sqlx::{FromRow, Pool, Postgres};
use std::collections::HashMap;
use serde::Serialize;
use anyhow::anyhow;

#[derive(Debug, FromRow, Serialize)]
pub struct Supertask {
    id: i32,
    title: String,
    domain_id: i32,
}

#[derive(Debug, FromRow, Clone, Serialize)]
pub struct Task {
    id: i32,
    supertask_id: i32,
    title: String,
    domain_id: i32
}

#[derive(Debug, FromRow, Serialize)]
pub struct Subtask {
    id: i32,
    task_id: i32,
    title: String,
    domain_id: i32
}

/// Represents a level of entries, with an entry and its children.
/// 
/// # Type Parameters
/// 
/// * `T` - The type of the entry.
/// * `U` - The type of the children.
#[derive(Serialize)]
pub struct EntryLevel<T: Sized, U: Sized> {
    entry: T,
    children: Vec<U>
}

type EntryHierarchy = EntryLevel<Supertask, EntryLevel<Task, Subtask>>;

/// Represents the hierarchal structure of entries, with supertasks at the top level, tasks at the second level, and subtasks at the third level.
#[derive(Serialize)]
pub struct EntryStructure(Vec<EntryHierarchy>);

macro_rules! fetch_all {
    ($struct_name:ident, $table_name:literal) => {
        pub async fn fetch_all(pool: &Pool<Postgres>, domain_id: i32) -> anyhow::Result<Box<[Self]>> {
            let records = sqlx::query_as!(
                $struct_name,
                "SELECT * FROM " + $table_name + " WHERE domain_id = $1;",
                domain_id
            )
            .fetch_all(pool)
            .await?;

            Ok(records.into_boxed_slice())
        }
    };
}

macro_rules! update_title {
    ($struct_name:ident, $table_name:literal) => {
        pub async fn update_title(pool: &Pool<Postgres>, id: i32, title: &str) -> anyhow::Result<()> {
            let mut transaction = pool.begin().await?;
            
            sqlx::query(
                format!(
                    r#"
                    UPDATE {} SET title = $1 WHERE id = $2;
                    UPDATE domain SET last_updated = NOW()
                    WHERE id = (SELECT domain_id FROM {} WHERE id = $2);
                    "#,
                    $table_name, $table_name
                ).as_str(),
            )
            .bind(title)
            .bind(id)
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

    pub async fn insert(pool: &Pool<Postgres>, title: &str, domain_id: i32) -> anyhow::Result<()> {
        let mut transaction = pool.begin().await?;

        sqlx::query(
            r#"
            INSERT INTO supertasks (title, domain_id) VALUES ($1, $2);
            UPDATE domain SET last_updated = NOW() WHERE id = $2;
            "#,
        )
        .bind(title)
        .bind(domain_id)
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }

    pub async fn delete(pool: &Pool<Postgres>, id: i32) -> anyhow::Result<()> {
        let mut transaction = pool.begin().await?;

        sqlx::query(
            r#"
            DELETE FROM supertasks WHERE id = $1;
            DELETE FROM subtasks WHERE task_id IN (SELECT id FROM tasks WHERE supertask_id = $1);
            DELETE FROM tasks WHERE supertask_id = $1;
            UPDATE domain SET last_updated = NOW()
            WHERE id = (SELECT domain_id FROM supertasks WHERE id = $1);
            "#,
        )
        .bind(id)
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }
}

impl Task {
    fetch_all!(Task, "tasks");
    update_title!(Task, "tasks");

    pub async fn insert(pool: &Pool<Postgres>, title: &str, domain_id: i32, supertask_id: i32) -> anyhow::Result<()> {
        let mut transaction = pool.begin().await?;

        sqlx::query(
            r#"
            INSERT INTO tasks (title, domain_id, supertask_id) VALUES ($1, $2, $3);
            UPDATE domain SET last_updated = NOW()
            WHERE id = $2;
            "#,
        )
        .bind(title)
        .bind(domain_id)
        .bind(supertask_id)
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }

    pub async fn delete(pool: &Pool<Postgres>, id: i32) -> anyhow::Result<()> {
        let mut transaction = pool.begin().await?;

        sqlx::query(
            r#"
            DELETE FROM tasks WHERE id = $1;
            DELETE FROM subtasks WHERE task_id = $1;
            UPDATE domain SET last_updated = NOW()
            WHERE id = (SELECT domain_id FROM tasks WHERE id = $1);
            "#,
        )
        .bind(id)
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }
}

impl Subtask {
    fetch_all!(Subtask, "subtasks");
    update_title!(Subtask, "subtasks");

    pub async fn insert(pool: &Pool<Postgres>, title: &str, domain_id: i32, task_id: i32) -> anyhow::Result<()> {
        let mut transaction = pool.begin().await?;

        sqlx::query(
            r#"
            INSERT INTO subtasks (title, domain_id, task_id) VALUES ($1, $2, $3);
            UPDATE domain SET last_updated = NOW()
            WHERE id = $2;
            "#,
        )
        .bind(title)
        .bind(domain_id)
        .bind(task_id)
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }

    pub async fn delete(pool: &Pool<Postgres>, id: i32) -> anyhow::Result<()> {
        let mut transaction = pool.begin().await?;

        sqlx::query(
            r#"
            DELETE FROM subtasks WHERE id = $1;
            UPDATE domain SET last_updated = NOW()
            WHERE id = (SELECT domain_id FROM subtasks WHERE id = $1);
            "#,
        )
        .bind(id)
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }
}

impl<T: Sized, U: Sized> EntryLevel<T, U> {
    pub fn new(entry: T) -> Self {
        EntryLevel {
            entry,
            children: vec![]
        }
    }

    pub fn children_mut(&mut self) -> &mut Vec<U> {
        &mut self.children
    }
}

trait Mappable: Sized {
    fn id(&self) -> i32;

    fn build_lookup_map(collection: &[Self]) -> HashMap<i32, usize> {
        let mut lookup_map = HashMap::with_capacity(collection.len());

        for i in 0..collection.len() {
            lookup_map.insert(collection[i].id(), i);
        }

        lookup_map
    }
}

impl Mappable for Supertask {
    fn id(&self) -> i32 {
        self.id
    }
}

impl Mappable for Task {
    fn id(&self) -> i32 {
        self.id
    }
}

impl EntryStructure {
    pub fn new(supertask_count: usize) -> Self {
        EntryStructure(Vec::with_capacity(supertask_count))
    }

    pub fn supertasks_mut(&mut self) -> &mut Vec<EntryHierarchy> {
        &mut self.0
    }

    pub fn tasks_mut(&mut self, supertask_index: usize) -> &mut Vec<EntryLevel<Task, Subtask>> {
        let supertask = &mut self.0[supertask_index];
        supertask.children_mut()
    }

    pub fn subtasks_mut(&mut self, supertask_index: usize, task_index: usize) -> &mut Vec<Subtask> {
        let tasks = self.tasks_mut(supertask_index);
        tasks[task_index].children_mut()
    }

    /// Builds an entry structure from an unordered collection of supertasks, tasks, and subtasks.
    /// Takes ownership of the collections.
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
    pub fn build(supertasks: Box<[Supertask]>, tasks: Box<[Task]>, subtasks: Box<[Subtask]>) -> anyhow::Result<Self> {
        let supertask_lookup_map = Supertask::build_lookup_map(&supertasks);
        let task_lookup_map = Task::build_lookup_map(&tasks);

        let mut entry_structure = EntryStructure::new(supertasks.len());

        for supertask in supertasks.into_vec() {
            entry_structure
                .supertasks_mut()
                .push(EntryLevel::new(supertask));
        }

        for task in tasks.iter() {
            let supertask_index = *supertask_lookup_map
                .get(&task.supertask_id)
                .ok_or(anyhow!("Task's supertask not found"))?;

            let tasks = entry_structure
                .tasks_mut(supertask_index);

            tasks.push(EntryLevel::new(task.to_owned()));
        }

        for subtask in subtasks.into_vec() {
            let task_index = *task_lookup_map
                .get(&subtask.task_id)
                .ok_or(anyhow!("Subtask's task not found"))?;

            let supertask_index = *supertask_lookup_map
                .get(&tasks[task_index].supertask_id)
                .ok_or(anyhow!("Task's supertask not found"))?;

            let subtasks = entry_structure
                .subtasks_mut(supertask_index, task_index);

            subtasks.push(subtask);
        }

        Ok(entry_structure)
    }

    pub async fn fetch(pool: &Pool<Postgres>, domain_id: i32) -> anyhow::Result<Self> {
        let supertasks = Supertask::fetch_all(pool, domain_id).await?;
        let tasks = Task::fetch_all(pool, domain_id).await?;
        let subtasks = Subtask::fetch_all(pool, domain_id).await?;

        let structure = Self::build(supertasks, tasks, subtasks)?;
        Ok(structure)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        let supertasks = Box::new([Supertask {
            id: 1,
            title: "Supertask 1".to_string(),
            domain_id: 1
        }]);

        let tasks = Box::new([Task {
            id: 1,
            supertask_id: 1,
            title: "Task 1".to_string(),
            domain_id: 1
        }]);

        let subtasks = Box::new([Subtask {
            id: 1,
            task_id: 1,
            title: "Subtask 1".to_string(),
            domain_id: 1
        }]);

        let result = EntryStructure::build(supertasks, tasks, subtasks);
        assert!(result.is_ok());

        let mut entry_structure = result.unwrap();
        assert_eq!(entry_structure.supertasks_mut().len(), 1);
        assert_eq!(entry_structure.tasks_mut(0).len(), 1);
        assert_eq!(entry_structure.subtasks_mut(0, 0).len(), 1);
    }
}