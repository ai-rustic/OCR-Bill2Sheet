use std::env;
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: IpAddr,
    pub port: u16,
}

#[derive(Debug, thiserror::Error)]
pub enum ServerConfigError {
    #[error("Environment variable error: {0}")]
    Environment(String),

    #[error("Parse error: {0}")]
    Parse(String),
}

impl ServerConfig {
    /// Create ServerConfig from environment variables
    pub fn from_env() -> Result<Self, ServerConfigError> {
        let host_str = env::var("SERVER_HOST")
            .unwrap_or_else(|_| "127.0.0.1".to_string());

        let port_str = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string());

        let host: IpAddr = host_str.parse()
            .map_err(|e| ServerConfigError::Parse(format!("Invalid SERVER_HOST '{}': {}", host_str, e)))?;

        let port: u16 = port_str.parse()
            .map_err(|e| ServerConfigError::Parse(format!("Invalid SERVER_PORT '{}': {}", port_str, e)))?;

        Ok(Self { host, port })
    }

    /// Get the socket address for binding
    pub fn socket_addr(&self) -> std::net::SocketAddr {
        std::net::SocketAddr::new(self.host, self.port)
    }

    /// Display config info (safe for logging)
    pub fn display_config(&self) -> String {
        format!("host={}:{}", self.host, self.port)
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 3000,
        }
    }
}