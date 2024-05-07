use crate::utilities::{USERNAME_REGEX, NAME_REGEX, EMAIL_REGEX, PASSWORD_REGEX};
use crate::models::{model::Model, user::User};
use actix_web::{web::{Json, Data}, HttpResponse, Responder};
use super::validatable::Validatable;
use serde::Deserialize;
use regex::Regex;
use redis::Client;

#[derive(Deserialize)]
struct RegisterUser {
    pub username: String,
    pub name: String,
    pub email: String,
    pub password: String
}

impl Validatable for RegisterUser {
    fn is_valid(&self) -> bool {
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

#[actix_web::post("/api/user/register")]
pub(super) async fn register(redis: Data<Client>, create_user: Json<RegisterUser>) -> impl Responder {
    if !create_user.is_valid() {
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
        Some(user) => user,
        None => return HttpResponse::InternalServerError().finish()
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
    use super::{RegisterUser, Validatable};

    #[test]
    fn test_validate_create_user() {
        let create_user = RegisterUser {
            username: "test_user".to_string(),
            name: "Test User".to_string(),
            email: "test@test.com".to_string(),
            password: "Password1".to_string(),
        };

        assert_eq!(create_user.is_valid(), true);
    }

    #[test]
    fn test_invalid_username() {
        let create_user = RegisterUser {
            username: "test_user!".to_string(),
            name: "Test User".to_string(),
            email: "test@test.com".to_string(),
            password: "Password1".to_string(),
        };

        assert_eq!(create_user.is_valid(), false);
    }

    #[test]
    fn test_invalid_name() {
        let create_user = RegisterUser {
            username: "test_user".to_string(),
            name: "Test User123".to_string(),
            email: "test@test.com".to_string(),
            password: "Password1".to_string(),
        };

        assert_eq!(create_user.is_valid(), false);
    }

    #[test]
    fn test_invalid_email() {
        let create_user = RegisterUser {
            username: "test_user".to_string(),
            name: "Test User".to_string(),
            email: "test@test".to_string(),
            password: "Password1".to_string(),
        };

        assert_eq!(create_user.is_valid(), false);
    }

    #[test]
    fn test_invalid_password() {
        let create_user = RegisterUser {
            username: "test_user".to_string(),
            name: "Test User".to_string(),
            email: "test@test.com".to_string(),
            password: "poop".to_string(),
        };

        assert_eq!(create_user.is_valid(), false);
    }
}