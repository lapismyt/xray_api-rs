use tonic::transport::Channel;
use crate::error::Result;
use crate::xray::app::proxyman::command::handler_service_client::HandlerServiceClient as gRPCHandlerClient;
use crate::xray::app::proxyman::command::{
    AddInboundRequest, RemoveInboundRequest, AddOutboundRequest, RemoveOutboundRequest,
    ListInboundsRequest, ListOutboundsRequest, AlterInboundRequest,
    AddUserOperation, RemoveUserOperation, GetInboundUserRequest
};
use crate::xray::core::InboundHandlerConfig as gRPCInboundConfig;
use crate::xray::core::OutboundHandlerConfig as gRPCOutboundConfig;
use crate::types::{InboundConfig, OutboundConfig, User};
use crate::xray::common::serial::TypedMessage;

/// Client for the Proxyman service.
///
/// This service allows you to manage inbound and outbound handlers dynamically.
#[derive(Clone, Debug)]
pub struct ProxymanClient {
    inner: gRPCHandlerClient<Channel>,
}

impl ProxymanClient {
    pub(crate) fn new(channel: Channel) -> Self {
        Self {
            inner: gRPCHandlerClient::new(channel),
        }
    }

    /// Adds a new inbound handler to Xray.
    ///
    /// * `tag`: Unique identifier for the inbound.
    /// * `protocol`: Protocol name (e.g., "vmess", "vless", "shadowsocks").
    /// * `settings`: Serialized protocol settings (usually JSON bytes).
    pub async fn add_inbound(&mut self, tag: impl Into<String>, protocol: impl Into<String>, settings: Vec<u8>) -> Result<()> {
        let request = AddInboundRequest {
            inbound: Some(gRPCInboundConfig {
                tag: tag.into(),
                proxy_settings: Some(TypedMessage {
                    r#type: protocol.into(),
                    value: settings,
                }),
                ..Default::default()
            }),
        };
        self.inner.add_inbound(request).await?;
        Ok(())
    }

    /// Removes an existing inbound handler by its tag.
    pub async fn remove_inbound(&mut self, tag: impl Into<String>) -> Result<()> {
        let request = RemoveInboundRequest {
            tag: tag.into(),
        };
        self.inner.remove_inbound(request).await?;
        Ok(())
    }

    /// Lists current inbound handlers.
    ///
    /// * `only_tags`: If true, only the tags of the inbounds are returned.
    pub async fn list_inbounds(&mut self, only_tags: bool) -> Result<Vec<InboundConfig>> {
        let request = ListInboundsRequest {
            is_only_tags: only_tags,
        };
        let response = self.inner.list_inbounds(request).await?.into_inner();
        Ok(response.inbounds.into_iter().map(Into::into).collect())
    }

    /// Adds a user to an existing inbound handler.
    ///
    /// * `tag`: Tag of the inbound handler.
    /// * `user`: User configuration including email and level.
    pub async fn add_user(&mut self, tag: impl Into<String>, user: User) -> Result<()> {
        let operation = AddUserOperation {
            user: Some(user.into()),
        };
        let request = AlterInboundRequest {
            tag: tag.into(),
            operation: Some(TypedMessage::from_message("xray.app.proxyman.command.AddUserOperation", &operation)),
        };
        self.inner.alter_inbound(request).await?;
        Ok(())
    }

    /// Removes a user from an existing inbound handler by email.
    pub async fn remove_user(&mut self, tag: impl Into<String>, email: impl Into<String>) -> Result<()> {
        let operation = RemoveUserOperation {
            email: email.into(),
        };
        let request = AlterInboundRequest {
            tag: tag.into(),
            operation: Some(TypedMessage::from_message("xray.app.proxyman.command.RemoveUserOperation", &operation)),
        };
        self.inner.alter_inbound(request).await?;
        Ok(())
    }

    /// Retrieves users associated with an inbound handler.
    pub async fn get_inbound_users(&mut self, tag: impl Into<String>, email: impl Into<String>) -> Result<Vec<User>> {
        let request = GetInboundUserRequest {
            tag: tag.into(),
            email: email.into(),
        };
        let response = self.inner.get_inbound_users(request).await?.into_inner();
        Ok(response.users.into_iter().map(Into::into).collect())
    }

    /// Retrieves the count of users associated with an inbound handler.
    pub async fn get_inbound_users_count(&mut self, tag: impl Into<String>, email: impl Into<String>) -> Result<i64> {
        let request = GetInboundUserRequest {
            tag: tag.into(),
            email: email.into(),
        };
        let response = self.inner.get_inbound_users_count(request).await?.into_inner();
        Ok(response.count)
    }

    /// Adds a new outbound handler to Xray.
    ///
    /// * `tag`: Unique identifier for the outbound.
    /// * `protocol`: Protocol name.
    /// * `settings`: Serialized protocol settings.
    pub async fn add_outbound(&mut self, tag: impl Into<String>, protocol: impl Into<String>, settings: Vec<u8>) -> Result<()> {
        let request = AddOutboundRequest {
            outbound: Some(gRPCOutboundConfig {
                tag: tag.into(),
                proxy_settings: Some(TypedMessage {
                    r#type: protocol.into(),
                    value: settings,
                }),
                ..Default::default()
            }),
        };
        self.inner.add_outbound(request).await?;
        Ok(())
    }

    /// Removes an existing outbound handler by its tag.
    pub async fn remove_outbound(&mut self, tag: impl Into<String>) -> Result<()> {
        let request = RemoveOutboundRequest {
            tag: tag.into(),
        };
        self.inner.remove_outbound(request).await?;
        Ok(())
    }

    /// Lists current outbound handlers.
    pub async fn list_outbounds(&mut self) -> Result<Vec<OutboundConfig>> {
        let request = ListOutboundsRequest {};
        let response = self.inner.list_outbounds(request).await?.into_inner();
        Ok(response.outbounds.into_iter().map(Into::into).collect())
    }
}
