use tonic::transport::Channel;
use crate::error::Result;
use crate::xray::app::router::command::routing_service_client::RoutingServiceClient as gRPCRoutingClient;
use crate::xray::app::router::command::{
    GetBalancerInfoRequest, OverrideBalancerTargetRequest,
    AddRuleRequest, RemoveRuleRequest, ListRuleRequest
};
use crate::xray::common::serial::TypedMessage;
use crate::types::{BalancerInfo, RoutingRuleItem};

/// Client for the Routing service.
///
/// This service allows you to query balancer information and dynamically manage routing rules.
#[derive(Clone, Debug)]
pub struct RoutingClient {
    inner: gRPCRoutingClient<Channel>,
}

impl RoutingClient {
    pub(crate) fn new(channel: Channel) -> Self {
        Self {
            inner: gRPCRoutingClient::new(channel),
        }
    }

    /// Retrieves information about a specific balancer by its tag.
    pub async fn get_balancer_info(&mut self, tag: impl Into<String>) -> Result<BalancerInfo> {
        let request = GetBalancerInfoRequest {
            tag: tag.into(),
        };
        let response = self.inner.get_balancer_info(request).await?.into_inner();
        Ok(response.balancer.map(Into::into).ok_or_else(|| crate::Error::Unknown("Empty response".into()))?)
    }

    /// Overrides the target of a specific balancer.
    ///
    /// * `balancer_tag`: Tag of the balancer to modify.
    /// * `target`: Tag of the outbound handler to set as the new target.
    pub async fn override_balancer_target(&mut self, balancer_tag: impl Into<String>, target: impl Into<String>) -> Result<()> {
        let request = OverrideBalancerTargetRequest {
            balancer_tag: balancer_tag.into(),
            target: target.into(),
        };
        self.inner.override_balancer_target(request).await?;
        Ok(())
    }

    /// Adds a new routing rule.
    ///
    /// * `config`: The rule configuration wrapped in a `TypedMessage`.
    /// * `should_append`: If true, the rule is added to the end of the existing ruleset.
    pub async fn add_rule(&mut self, config: TypedMessage, should_append: bool) -> Result<()> {
        let request = AddRuleRequest {
            config: Some(config),
            should_append,
        };
        self.inner.add_rule(request).await?;
        Ok(())
    }

    /// Removes a routing rule by its tag.
    pub async fn remove_rule(&mut self, rule_tag: impl Into<String>) -> Result<()> {
        let request = RemoveRuleRequest {
            rule_tag: rule_tag.into(),
        };
        self.inner.remove_rule(request).await?;
        Ok(())
    }

    /// Lists all current routing rules.
    pub async fn list_rules(&mut self) -> Result<Vec<RoutingRuleItem>> {
        let request = ListRuleRequest {};
        let response = self.inner.list_rule(request).await?.into_inner();
        Ok(response.rules.into_iter().map(Into::into).collect())
    }
}
