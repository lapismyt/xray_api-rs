use tonic::transport::{Channel, Endpoint};
use crate::error::Result;
use crate::services::proxyman::ProxymanClient;
use crate::services::stats::StatsClient;
use crate::services::routing::RoutingClient;
use crate::services::logger::LoggerClient;
use crate::services::observatory::ObservatoryClient;

/// The primary entry point for interacting with the Xray API.
///
/// Use `Client::connect` to establish a gRPC connection to an Xray-core instance.
/// Once connected, you can access specific services via the provided methods.
#[derive(Clone, Debug)]
pub struct Client {
    channel: Channel,
}

impl Client {
    /// Connects to an Xray gRPC API endpoint.
    ///
    /// # Example
    /// ```no_run
    /// use xray_api::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::connect("http://127.0.0.1:10085").await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn connect<D>(dst: D) -> Result<Self>
    where
        D: TryInto<Endpoint>,
        D::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        let endpoint = Endpoint::new(dst).map_err(crate::Error::from)?;
        let channel = endpoint.connect().await?;
        Ok(Self { channel })
    }

    /// Accesses the Proxyman (Handler) service for managing inbounds and outbounds.
    pub fn proxyman(&self) -> ProxymanClient {
        ProxymanClient::new(self.channel.clone())
    }

    /// Accesses the Stats service for retrieving traffic and system statistics.
    pub fn stats(&self) -> StatsClient {
        StatsClient::new(self.channel.clone())
    }

    /// Accesses the Routing service for rule and balancer management.
    pub fn routing(&self) -> RoutingClient {
        RoutingClient::new(self.channel.clone())
    }

    /// Accesses the Logger service for restarting the internal logger.
    pub fn logger(&self) -> LoggerClient {
        LoggerClient::new(self.channel.clone())
    }

    /// Accesses the Observatory service for outbound performance monitoring.
    pub fn observatory(&self) -> ObservatoryClient {
        ObservatoryClient::new(self.channel.clone())
    }
}
