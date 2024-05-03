/// Represents the environment configuration for the application.
#[derive(Clone)]
pub struct Environment {
    host_ip: String,
    host_port: String,
    redis_user: String,
    redis_password: String,
    redis_endpoint: String,
    redis_port: String,
    redis_db: String
}

impl Environment {
    /// Creates a new instance of the `Environment` struct by reading the necessary environment variables.
    ///
    /// # Errors
    ///
    /// This function will return an `anyhow::Error` if any of the required environment variables are missing or invalid.
    pub fn new() -> anyhow::Result<Self> {
        let host_ip = std::env::var("HOST_IP")?;
        let host_port = std::env::var("HOST_PORT")?;
        let redis_user = std::env::var("REDIS_USER")?;
        let redis_password = std::env::var("REDIS_PASSWORD")?;
        let redis_endpoint = std::env::var("REDIS_ENDPOINT")?;
        let redis_port = std::env::var("REDIS_PORT")?;
        let redis_db = std::env::var("REDIS_DB")?;

        Ok(Self {
            host_ip,
            host_port,
            redis_user,
            redis_password,
            redis_endpoint,
            redis_port,
            redis_db
        })
    }
    
    /// Returns the host IP address.
    pub fn host_ip(&self) -> &str {
        &self.host_ip
    }

    /// Returns the host port.
    pub fn host_port(&self) -> &str {
        &self.host_port
    }

    /// Returns the Redis username.
    pub fn redis_user(&self) -> &str {
        &self.redis_user
    }

    /// Returns the Redis password.
    pub fn redis_password(&self) -> &str {
        &self.redis_password
    }

    /// Returns the Redis endpoint.
    pub fn redis_endpoint(&self) -> &str {
        &self.redis_endpoint
    }

    /// Returns the Redis port.
    pub fn redis_port(&self) -> &str {
        &self.redis_port
    }

    /// Returns the Redis database number.
    pub fn redis_db(&self) -> &str {
        &self.redis_db
    }
}
        