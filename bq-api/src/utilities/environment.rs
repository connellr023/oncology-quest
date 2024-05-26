/// Represents the environment configuration for the application.
#[derive(Clone)]
pub struct Environment {
    host_ip: String,
    host_port: String,
    database_url: String
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
        let database_url = std::env::var("DATABASE_URL")?;

        Ok(Self {
            host_ip,
            host_port,
            database_url
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

    /// Returns the database URL.
    pub fn database_url(&self) -> &str {
        &self.database_url
    }
}
        