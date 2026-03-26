//! High-level Xray API client for Rust.
//!
//! This library provides an ergonomic, gRPC-abstracted interface to interact with Xray-core.
//! It encapsulates all gRPC logic, providing clean Rust types and methods for managing
//! inbounds, outbounds, statistics, routing, and more.

/// Low-level generated gRPC modules.
///
/// These modules are generated from Xray-core proto files and are primarily used internally
/// by the high-level clients. Use them only if you need low-level gRPC access.
pub mod xray {
    pub mod common {
        pub mod net {
            include!(concat!(env!("OUT_DIR"), "/xray.common.net.rs"));
        }
        pub mod protocol {
            include!(concat!(env!("OUT_DIR"), "/xray.common.protocol.rs"));
        }
        pub mod serial {
            include!(concat!(env!("OUT_DIR"), "/xray.common.serial.rs"));

            impl TypedMessage {
                /// Create a TypedMessage from a message and its type name.
                pub fn new<M: prost::Message>(type_name: impl Into<String>, message: &M) -> Self {
                    Self {
                        r#type: type_name.into(),
                        value: message.encode_to_vec(),
                    }
                }
            }
        }
    }

    pub mod core {
        include!(concat!(env!("OUT_DIR"), "/xray.core.rs"));

        pub mod app {
            pub mod observatory {
                include!(concat!(env!("OUT_DIR"), "/xray.core.app.observatory.rs"));
                pub mod command {
                    include!(concat!(
                        env!("OUT_DIR"),
                        "/xray.core.app.observatory.command.rs"
                    ));
                }
            }
        }
    }

    pub mod app {
        pub mod proxyman {
            pub mod command {
                include!(concat!(env!("OUT_DIR"), "/xray.app.proxyman.command.rs"));
            }
        }
        pub mod stats {
            pub mod command {
                include!(concat!(env!("OUT_DIR"), "/xray.app.stats.command.rs"));
            }
        }
        pub mod router {
            pub mod command {
                include!(concat!(env!("OUT_DIR"), "/xray.app.router.command.rs"));
            }
        }
        pub mod log {
            pub mod command {
                include!(concat!(env!("OUT_DIR"), "/xray.app.log.command.rs"));
            }
        }
    }

    pub mod proxy {
        pub mod vmess {
            pub mod inbound {
                include!(concat!(env!("OUT_DIR"), "/xray.proxy.vmess.inbound.rs"));
            }
        }
        pub mod vless {
            pub mod inbound {
                include!(concat!(env!("OUT_DIR"), "/xray.proxy.vless.inbound.rs"));
            }
        }
        pub mod shadowsocks {
            include!(concat!(env!("OUT_DIR"), "/xray.proxy.shadowsocks.rs"));
        }
        pub mod trojan {
            include!(concat!(env!("OUT_DIR"), "/xray.proxy.trojan.rs"));
        }
    }
}

pub mod error;
pub mod client;
pub mod services;
pub mod types;

pub use error::{Error, Result};
pub use client::Client;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_structure() {
        // This test only checks that we can call the methods, not that they connect successfully.
        // We use a dummy URL.
        let client_result = Client::connect("http://127.0.0.1:10085").await;
        // It should fail to connect but the structure should be correct.
        assert!(client_result.is_err());
    }

    #[test]
    fn test_stat_type() {
        use crate::xray::app::stats::command::Stat as gRPCStat;
        let grpc_stat = gRPCStat {
            name: "test".to_string(),
            value: 123,
        };
        let stat: Stat = grpc_stat.into();
        assert_eq!(stat.name, "test");
        assert_eq!(stat.value, 123);
    }
}
