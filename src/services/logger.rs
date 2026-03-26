use tonic::transport::Channel;
use crate::error::Result;
use crate::xray::app::log::command::logger_service_client::LoggerServiceClient as gRPCLoggerClient;
use crate::xray::app::log::command::RestartLoggerRequest;

/// Client for the Logger service.
///
/// This service allows you to control Xray's internal logger.
#[derive(Clone, Debug)]
pub struct LoggerClient {
    inner: gRPCLoggerClient<Channel>,
}

impl LoggerClient {
    pub(crate) fn new(channel: Channel) -> Self {
        Self {
            inner: gRPCLoggerClient::new(channel),
        }
    }

    /// Restarts the internal logger, effectively reloading its configuration and reopening log files.
    pub async fn restart_logger(&mut self) -> Result<()> {
        let request = RestartLoggerRequest {};
        self.inner.restart_logger(request).await?;
        Ok(())
    }
}
