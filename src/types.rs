use crate::xray::app::stats::command::{
    Stat as gRPCStat, SysStatsResponse as gRPCSysStats, GetStatsOnlineIpListResponse as gRPCOnlineIpList
};
use crate::xray::core::{InboundHandlerConfig as gRPCInboundConfig, OutboundHandlerConfig as gRPCOutboundConfig};
use crate::xray::common::protocol::User as gRPCUser;
use crate::xray::app::router::command::{
    BalancerMsg as gRPCBalancerMsg, ListRuleItem as gRPCListRuleItem
};
pub use crate::xray::common::serial::TypedMessage;

impl TypedMessage {
    /// Create a new TypedMessage from a message that implements prost::Message.
    pub fn from_message<M: prost::Message>(type_name: impl Into<String>, message: &M) -> Self {
        Self {
            r#type: type_name.into(),
            value: message.encode_to_vec(),
        }
    }
}

/// Represents a single statistic counter.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Stat {
    /// The name of the statistic (e.g., "user>>>email>>>traffic>>>downlink").
    pub name: String,
    /// The current value of the statistic.
    pub value: i64,
}

impl From<gRPCStat> for Stat {
    fn from(s: gRPCStat) -> Self {
        Self {
            name: s.name,
            value: s.value,
        }
    }
}

/// Represents system-level performance statistics from the Xray instance.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SysStats {
    /// Current number of Go routines.
    pub num_goroutine: u32,
    /// Total number of completed GC cycles.
    pub num_gc: u32,
    /// Bytes of allocated heap objects.
    pub alloc: u64,
    /// Cumulative bytes allocated for heap objects.
    pub total_alloc: u64,
    /// Total bytes of memory obtained from the OS.
    pub sys: u64,
    /// Cumulative count of heap objects allocated.
    pub mallocs: u64,
    /// Cumulative count of heap objects freed.
    pub frees: u64,
    /// Number of live heap objects.
    pub live_objects: u64,
    /// Total GC pause time in nanoseconds.
    pub pause_total_ns: u64,
    /// Time elapsed since the process started, in seconds.
    pub uptime: u32,
}

impl From<gRPCSysStats> for SysStats {
    fn from(s: gRPCSysStats) -> Self {
        Self {
            num_goroutine: s.num_goroutine,
            num_gc: s.num_gc,
            alloc: s.alloc,
            total_alloc: s.total_alloc,
            sys: s.sys,
            mallocs: s.mallocs,
            frees: s.frees,
            live_objects: s.live_objects,
            pause_total_ns: s.pause_total_ns,
            uptime: s.uptime,
        }
    }
}

/// Represents a list of online IP addresses associated with a user or counter.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OnlineIpList {
    /// The name of the statistic or user.
    pub name: String,
    /// A map of IP addresses to their last access timestamps.
    pub ips: std::collections::HashMap<String, i64>,
}

impl From<gRPCOnlineIpList> for OnlineIpList {
    fn from(r: gRPCOnlineIpList) -> Self {
        Self {
            name: r.name,
            ips: r.ips,
        }
    }
}

/// Configuration summary for an inbound handler.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InboundConfig {
    /// The unique tag of the inbound handler.
    pub tag: String,
}

impl From<gRPCInboundConfig> for InboundConfig {
    fn from(c: gRPCInboundConfig) -> Self {
        Self {
            tag: c.tag,
        }
    }
}

/// Configuration summary for an outbound handler.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OutboundConfig {
    /// The unique tag of the outbound handler.
    pub tag: String,
}

impl From<gRPCOutboundConfig> for OutboundConfig {
    fn from(c: gRPCOutboundConfig) -> Self {
        Self {
            tag: c.tag,
        }
    }
}

/// Represents a user within an Xray inbound.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    /// User level (privilege level).
    pub level: u32,
    /// Unique email address for identification.
    pub email: String,
}

impl From<gRPCUser> for User {
    fn from(u: gRPCUser) -> Self {
        Self {
            level: u.level,
            email: u.email,
        }
    }
}

impl From<User> for gRPCUser {
    fn from(u: User) -> gRPCUser {
        gRPCUser {
            level: u.level,
            email: u.email,
            account: None,
        }
    }
}

/// Information about a load balancer's state.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BalancerInfo {
    /// The current target tag set via override.
    pub override_target: String,
    /// The list of target tags available to the balancer.
    pub principle_target: Vec<String>,
}

impl From<gRPCBalancerMsg> for BalancerInfo {
    fn from(m: gRPCBalancerMsg) -> Self {
        Self {
            override_target: m.r#override.map(|o| o.target).unwrap_or_default(),
            principle_target: m.principle_target.map(|p| p.tag).unwrap_or_default(),
        }
    }
}

/// Represents an item in the routing rules list.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RoutingRuleItem {
    /// The tag of the routing handler this rule belongs to.
    pub tag: String,
    /// The identifier for the specific rule.
    pub rule_tag: String,
}

impl From<gRPCListRuleItem> for RoutingRuleItem {
    fn from(i: gRPCListRuleItem) -> Self {
        Self {
            tag: i.tag,
            rule_tag: i.rule_tag,
        }
    }
}

/// Represents a routing rule (placeholder for complex configuration).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RoutingRule {
    /// The tag for the rule.
    pub tag: String,
}
