#[macro_export]
macro_rules! auth_user_session {
    ($session:ident) => {
        match $session.get::<i32>("uid") {
            Ok(Some(_)) => {},
            _ => return HttpResponse::Unauthorized().finish()
        }
    };
}

#[macro_export]
macro_rules! auth_user_session_with_id {
    ($varname:ident, $session:ident) => {
        let $varname = match $session.get::<i32>("uid") {
            Ok(Some($varname)) => $varname,
            _ => return HttpResponse::Unauthorized().finish()
        };
    };
}

#[macro_export]
macro_rules! auth_admin_session {
    ($varname:ident, $session:ident, $pool:ident) => {
        $crate::auth_user_session_with_id!($varname, $session);

        if !$crate::models::user::User::validate_is_admin(&$pool, $varname).await {
            return HttpResponse::Forbidden().finish();
        }
    };
}

#[macro_export]
macro_rules! auth_not_admin_session {
    ($varname:ident, $session:ident, $pool:ident) => {
        $crate::auth_user_session_with_id!($varname, $session);

        if $crate::models::user::User::validate_is_admin(&$pool, $varname).await {
            return HttpResponse::Forbidden().finish();
        }
    };
}