use tonic::transport::Channel;
use crate::error::Result;
use crate::xray::core::app::observatory::command::observatory_service_client::ObservatoryServiceClient as gRPCObservatoryClient;
use crate::xray::core::app::observatory::command::GetOutboundStatusRequest;

/// Client for the Observatory service.
///
/// This service provides information about outbound handler status and performance.
#[derive(Clone, Debug)]
pub struct ObservatoryClient {
    inner: gRPCObservatoryClient<Channel>,
}

impl ObservatoryClient {
    pub(crate) fn new(channel: Channel) -> Self {
        Self {
            inner: gRPCObservatoryClient::new(channel),
        }
    }

    /// Retrieves the current status of all monitored outbound handlers.
    pub async fn get_outbound_status(&mut self) -> Result<()> {
        let request = GetOutboundStatusRequest {};
        self.inner.get_outbound_status(request).await?;
        Ok(())
    }
}
