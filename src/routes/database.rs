use async_trait::async_trait;
use tiberius::{error::Error, AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

pub struct TiberiusConnectionManager {
    config: Config,
}

impl TiberiusConnectionManager {
    /// Create a new `TiberiusConnectionManager`.
    pub fn new(config: Config) -> tiberius::Result<TiberiusConnectionManager> {
        Ok(TiberiusConnectionManager { config })
    }
}



#[async_trait]
impl bb8::ManageConnection for TiberiusConnectionManager {
    type Connection = Client<Compat<TcpStream>>;
    type Error = Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let tcp = TcpStream::connect(&self.config.get_addr()).await?;
        tcp.set_nodelay(true)?;

        Client::connect(self.config.clone(), tcp.compat_write()).await
    }

    async fn is_valid(&self, conn: &mut Client<Compat<TcpStream>>) -> Result<(), Self::Error> {
        conn.simple_query("").await?.into_row().await?;
        Ok(())
    }

    fn has_broken(&self, _: &mut Client<Compat<TcpStream>>) -> bool {
        false
    }
}

// Example of creating the connection pool
pub async fn create_connection(
    host: &str,
    database: &str,
) -> anyhow::Result<bb8::Pool<TiberiusConnectionManager>> {
    let mut config = Config::new();
    config.host(host);
    config.database(database);
    config.authentication(AuthMethod::Integrated);
    config.trust_cert();
    let manager = TiberiusConnectionManager::new(config).unwrap();
    let pool = bb8::Pool::builder().build(manager).await?;
    Ok(pool)
}
