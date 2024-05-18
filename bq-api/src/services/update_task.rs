use crate::models::{model::Model, user::User, tasks::{SubTask, UserTaskEntries}};
use actix_web::{web::{Json, Data}, HttpResponse, Responder};
use actix_session::Session;
use serde::Deserialize;
use redis::Client;
use std::collections::HashMap;

#[derive(Deserialize)]
struct UpdateTaskQuery {
    pub index: (usize, usize, usize),
    pub task: SubTask
}

/// Updates the task at the given index.
/// 
/// # Arguments
/// 
/// * `entries` - A mutable reference to the user task entries.
/// * `index` - A tuple containing the indices of the task to update.
/// * `task` - The new task to replace the old one.
/// 
/// # Remarks
/// 
/// * If the task at the given index does not exist, it will be created.
/// * If the subtask at the given index does not exist, it will be created.
fn update_task(entries: &mut UserTaskEntries, index: (usize, usize, usize), task: SubTask) {
    if let Some(subtasks) = entries.get_mut(&index.0) {
        if let Some(tasks) = subtasks.get_mut(&index.1) {
            tasks.insert(index.2, task);
        }
        else {
            let mut tasks = HashMap::new();

            tasks.insert(index.2, task);
            subtasks.insert(index.1, tasks);
        }
    }
    else {
        let mut tasks = HashMap::new();
        let mut subtasks = HashMap::new();

        tasks.insert(index.2, task);
        subtasks.insert(index.1, tasks);
        entries.insert(index.0, subtasks);
    }
}

#[actix_web::patch("/api/tasks/update")]
pub(super) async fn update(session: Session, redis: Data<Client>, task_update: Json<UpdateTaskQuery>) -> impl Responder {
    let username = match session.get::<String>("username") {
        Ok(Some(username)) => username,
        _ => return HttpResponse::Unauthorized().finish()
    };
    
    let mut connection = match redis.get_connection() {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let mut user = match User::fetch(&mut connection, username.as_str()) {
        Ok(user) => user,
        Err(_) => return HttpResponse::NotFound().finish()
    };

    // Admins do not have tasks.
    if user.is_admin {
        return HttpResponse::Forbidden().finish();
    };

    update_task(user.tasks_mut(), task_update.index, task_update.task.clone());

    if user.update(&mut connection).is_err() {
        return HttpResponse::InternalServerError().finish();
    };
    
    HttpResponse::Ok().finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::parsables::{Parsable, Comment};
    
    #[test]
    fn test_update_task_new() {
        let mut entries = HashMap::new();
        let comment = Comment::parse(String::from("test comment")).unwrap();
        let task = SubTask {
            completed: false,
            comment: comment
        };

        update_task(&mut entries, (0, 0, 0), task.clone());
        assert_eq!(entries.len(), 1);
        assert_eq!(entries.get(&0).unwrap().len(), 1);
        assert_eq!(entries.get(&0).unwrap().get(&0).unwrap().len(), 1);
        assert_eq!(entries.get(&0).unwrap().get(&0).unwrap().get(&0).unwrap(), &task);
    }

    #[test]
    fn test_update_task_existing_subtask() {
        let mut entries = HashMap::new();
        let comment = Comment::parse(String::from("test comment")).unwrap();
        let task = SubTask {
            completed: false,
            comment: comment
        };

        update_task(&mut entries, (0, 0, 0), task.clone());
        update_task(&mut entries, (0, 0, 1), task.clone());
        assert_eq!(entries.len(), 1);
        assert_eq!(entries.get(&0).unwrap().len(), 1);
        assert_eq!(entries.get(&0).unwrap().get(&0).unwrap().len(), 2);
        assert_eq!(entries.get(&0).unwrap().get(&0).unwrap().get(&0).unwrap(), &task);
        assert_eq!(entries.get(&0).unwrap().get(&0).unwrap().get(&1).unwrap(), &task);
    }
}