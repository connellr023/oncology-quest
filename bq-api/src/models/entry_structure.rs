use chrono::{DateTime, Utc};
use sqlx::{FromRow, Pool, Postgres};
use serde::Serialize;

#[derive(Debug, FromRow, Serialize)]
pub struct Supertask {
    id: i32,
    title: String,
    domain_id: i32,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Task {
    id: i32,
    supertask_id: Option<i32>,
    title: String,
    domain_id: i32
}

#[derive(Debug, FromRow, Serialize)]
pub struct Subtask {
    id: i32,
    task_id: Option<i32>,
    title: String,
    domain_id: i32
}

#[derive(Debug, FromRow, Serialize)]
pub struct EntryStructure {
    supertasks: Box<[Supertask]>,
    tasks: Box<[Task]>,
    subtasks: Box<[Subtask]>
}

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
                    UPDATE domain SET last_updated = NOW() WHERE id = (SELECT domain_id FROM {} WHERE id = $2);
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
            UPDATE domain SET last_updated = NOW() WHERE id = (SELECT domain_id FROM supertasks WHERE id = $1);
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
            UPDATE domain SET last_updated = NOW() WHERE id = $2;
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
            UPDATE domain SET last_updated = NOW() WHERE id = (SELECT domain_id FROM tasks WHERE id = $1);
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
            UPDATE domain SET last_updated = NOW() WHERE id = $2;
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
            UPDATE domain SET last_updated = NOW() WHERE id = (SELECT domain_id FROM subtasks WHERE id = $1);
            "#,
        )
        .bind(id)
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }
}

impl EntryStructure {
    pub async fn fetch(pool: &Pool<Postgres>, domain_id: i32) -> anyhow::Result<Self> {
        Ok(Self {
            supertasks: Supertask::fetch_all(&pool, domain_id).await?,
            tasks: Task::fetch_all(&pool, domain_id).await?,
            subtasks: Subtask::fetch_all(&pool, domain_id).await?,
        })
    }
}