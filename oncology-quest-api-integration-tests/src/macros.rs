#[macro_export]
macro_rules! endpoint {
    ($route:literal) => {
        //format!("http://api:8000{}", $route)
        format!("http://127.0.0.1:8080{}", $route)
    }
}