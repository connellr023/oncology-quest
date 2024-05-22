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
    supertasks: Vec<Supertask>,
    tasks: Vec<Task>,
    subtasks: Vec<Subtask>,
}

macro_rules! generate_fetch_all {
    ($struct_name:ident, $table_name:literal) => {
        impl $struct_name {
            pub async fn fetch_all(pool: &Pool<Postgres>, domain_id: i32) -> anyhow::Result<Vec<Self>> {
                let records = sqlx::query_as!(
                    $struct_name,
                    "SELECT * FROM " + $table_name + " WHERE domain_id = $1;",
                    domain_id
                )
                .fetch_all(pool)
                .await?;

                Ok(records)
            }
        }
    };
}

generate_fetch_all!(Supertask, "supertasks");
generate_fetch_all!(Task, "tasks");
generate_fetch_all!(Subtask, "subtasks");

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