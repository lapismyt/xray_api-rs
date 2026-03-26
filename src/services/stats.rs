use tonic::transport::Channel;
use crate::error::Result;
use crate::xray::app::stats::command::stats_service_client::StatsServiceClient as gRPCStatsClient;
use crate::xray::app::stats::command::{
    GetStatsRequest, QueryStatsRequest, SysStatsRequest, GetAllOnlineUsersRequest
};

/// Client for the Stats service.
///
/// This service provides traffic counters and system resource usage information.
#[derive(Clone, Debug)]
pub struct StatsClient {
    inner: gRPCStatsClient<Channel>,
}

impl StatsClient {
    pub(crate) fn new(channel: Channel) -> Self {
        Self {
            inner: gRPCStatsClient::new(channel),
        }
    }

    /// Retrieves a specific statistic counter by name.
    ///
    /// * `name`: The full name of the counter (e.g., "user>>>email>>>traffic>>>downlink").
    /// * `reset`: If true, the counter is reset to zero after reading.
    pub async fn get_stats(&mut self, name: impl Into<String>, reset: bool) -> Result<crate::types::Stat> {
        let request = GetStatsRequest {
            name: name.into(),
            reset,
        };
        let response = self.inner.get_stats(request).await?.into_inner();
        Ok(response.stat.map(Into::into).ok_or_else(|| crate::Error::Unknown("Empty response".into()))?)
    }

    /// Retrieves the online session count for a specific user.
    pub async fn get_stats_online(&mut self, name: impl Into<String>, reset: bool) -> Result<crate::types::Stat> {
        let request = GetStatsRequest {
            name: name.into(),
            reset,
        };
        let response = self.inner.get_stats_online(request).await?.into_inner();
        Ok(response.stat.map(Into::into).ok_or_else(|| crate::Error::Unknown("Empty response".into()))?)
    }

    /// Queries multiple statistics using a pattern.
    ///
    /// * `pattern`: A string pattern to match counter names (e.g., "user>>>").
    /// * `reset`: If true, all matched counters are reset.
    pub async fn query_stats(&mut self, pattern: impl Into<String>, reset: bool) -> Result<Vec<crate::types::Stat>> {
        let request = QueryStatsRequest {
            pattern: pattern.into(),
            reset,
        };
        let response = self.inner.query_stats(request).await?.into_inner();
        Ok(response.stat.into_iter().map(Into::into).collect())
    }

    /// Retrieves system-level resource usage statistics.
    pub async fn get_sys_stats(&mut self) -> Result<crate::types::SysStats> {
        let request = SysStatsRequest {};
        let response = self.inner.get_sys_stats(request).await?.into_inner();
        Ok(response.into())
    }

    /// Retrieves a list of online IP addresses and their access times for a user.
    pub async fn get_stats_online_ip_list(&mut self, name: impl Into<String>, reset: bool) -> Result<crate::types::OnlineIpList> {
        let request = GetStatsRequest {
            name: name.into(),
            reset,
        };
        let response = self.inner.get_stats_online_ip_list(request).await?.into_inner();
        Ok(response.into())
    }

    /// Retrieves a list of emails for all users currently considered "online".
    pub async fn get_all_online_users(&mut self) -> Result<Vec<String>> {
        let request = GetAllOnlineUsersRequest {};
        let response = self.inner.get_all_online_users(request).await?.into_inner();
        Ok(response.users)
    }
}
