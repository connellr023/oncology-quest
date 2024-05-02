use super::validatable::Validatable;
use crate::utilities::regex_patterns::*;
use crate::models::{model::Model, user::User};
use actix_web::{web::{Json, Data}, HttpResponse, Responder};
use serde::Deserialize;
use regex::Regex;
use redis::Client;

#[derive(Deserialize)]
struct CreateUser {
    pub username: String,
    pub name: String,
    pub email: String,
    pub password: String
}

impl Validatable for CreateUser {
    fn validate(&self) -> bool {
        let username_pattern = Regex::new(USERNAME_REGEX).unwrap();
        let name_pattern = Regex::new(NAME_REGEX).unwrap();
        let email_pattern = Regex::new(EMAIL_REGEX).unwrap();
        let password_pattern = Regex::new(PASSWORD_REGEX).unwrap();

        username_pattern.is_match(&self.username) &&
        name_pattern.is_match(&self.name) &&
        email_pattern.is_match(&self.email) &&
        password_pattern.is_match(&self.password)
    }
}

#[actix_web::post("/api/user/create")]
pub(super) async fn create(redis: Data<Client>, create_user: Json<CreateUser>) -> impl Responder {
    if !create_user.validate() {
        return HttpResponse::BadRequest().finish();
    }

    let create_user = create_user.into_inner();
    let user = match User::new(
        create_user.username,
        create_user.name,
        create_user.email,
        create_user.password,
        false
    ) {
        Ok(user) => user,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let mut connection = match redis.get_connection() {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    if user.store(&mut connection) {
        return HttpResponse::Created().finish();
    }
    
    HttpResponse::InternalServerError().finish()
}

#[cfg(test)]
mod tests {
    use super::{CreateUser, Validatable};

    #[test]
    fn test_validate_create_user() {
        let create_user = CreateUser {
            username: "test_user".to_string(),
            name: "Test User".to_string(),
            email: "test@test.com".to_string(),
            password: "Password1".to_string(),
        };

        assert_eq!(create_user.validate(), true);
    }

    #[test]
    fn test_invalid_username() {
        let create_user = CreateUser {
            username: "test_user!".to_string(),
            name: "Test User".to_string(),
            email: "test@test.com".to_string(),
            password: "Password1".to_string(),
        };

        assert_eq!(create_user.validate(), false);
    }

    #[test]
    fn test_invalid_name() {
        let create_user = CreateUser {
            username: "test_user".to_string(),
            name: "Test User123".to_string(),
            email: "test@test.com".to_string(),
            password: "Password1".to_string(),
        };

        assert_eq!(create_user.validate(), false);
    }

    #[test]
    fn test_invalid_email() {
        let create_user = CreateUser {
            username: "test_user".to_string(),
            name: "Test User".to_string(),
            email: "test@test".to_string(),
            password: "Password1".to_string(),
        };

        assert_eq!(create_user.validate(), false);
    }

    #[test]
    fn test_invalid_password() {
        let create_user = CreateUser {
            username: "test_user".to_string(),
            name: "Test User".to_string(),
            email: "test@test.com".to_string(),
            password: "poop".to_string(),
        };

        assert_eq!(create_user.validate(), false);
    }
}