use std::{env::var, fmt::{self, Display, Formatter}};

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
        let host_ip = var("HOST_IP")?;
        let host_port = var("HOST_PORT")?;
        let database_url = var("DATABASE_URL")?;

        Ok(Self {
            host_ip,
            host_port,
            database_url
        })
    }
    
    /// Returns the host IP address.
    #[inline(always)]
    pub fn host_ip(&self) -> &str {
        &self.host_ip
    }

    /// Returns the host port.
    #[inline(always)]
    pub fn host_port(&self) -> &str {
        &self.host_port
    }

    /// Returns the database URL.
    #[inline(always)]
    pub fn database_url(&self) -> &str {
        &self.database_url
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Server running with configuration:")?;
        writeln!(f, "Host IP: {}", self.host_ip())?;
        writeln!(f, "Host Port: {}", self.host_port())?;
        writeln!(f, "Database URL: {}", self.database_url())?;
        writeln!(f, "Production Mode: {}", cfg!(feature = "production"))?;
        writeln!(f, "Monolith Mode: {}", cfg!(feature = "monolith"))?;

        Ok(())
    }
}