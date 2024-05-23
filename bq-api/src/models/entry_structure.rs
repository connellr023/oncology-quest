use super::domain::Domain;
use sqlx::{FromRow, Pool, Postgres};

#[derive(Debug, FromRow)]
pub struct Supertask {
    id: i32,
    title: String,
    domain_id: i32,
}

#[derive(Debug, FromRow)]
pub struct Task {
    id: i32,
    supertask_id: Option<i32>,
    title: String,
    domain_id: i32,
}

#[derive(Debug, FromRow)]
pub struct Subtask {
    id: i32,
    task_id: Option<i32>,
    title: String,
    domain_id: i32,
}

#[derive(Debug, FromRow)]
pub struct EntryStructure {
    domain: Domain,
    supertasks: Box<[Supertask]>,
    tasks: Box<[Task]>,
    subtasks: Box<[Subtask]>,
}

macro_rules! task_model {
    ($struct_name:ident, $table_name:literal) => {
        impl $struct_name {
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
        }
    };
}

task_model!(Supertask, "supertasks");
task_model!(Task, "tasks");
task_model!(Subtask, "subtasks");

impl Supertask {
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
    pub async fn fetch(pool: Pool<Postgres>, domain: Domain) -> anyhow::Result<Self> {
        let domain_id = domain.id();

        Ok(Self {
            domain,
            supertasks: Supertask::fetch_all(&pool, domain_id).await?,
            tasks: Task::fetch_all(&pool, domain_id).await?,
            subtasks: Subtask::fetch_all(&pool, domain_id).await?,
        })
    }
}