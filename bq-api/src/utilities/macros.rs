#[macro_export]
macro_rules! auth_admin {
    ($session:expr, $pool:expr) => {
        let user_id = match $session.get::<i32>("uid") {
            Ok(Some(user_id)) => user_id,
            _ => return HttpResponse::Unauthorized().finish()
        };

        if !User::validate_is_admin(&$pool, user_id).await {
            return HttpResponse::Forbidden().finish();
        }
    };
}