use clap::{Parser, Subcommand};
use xray_api::{Client, User};

#[derive(Parser)]
#[command(author, version, about = "Xray API CLI Tool", long_about = None)]
struct Cli {
    /// gRPC API address (e.g., http://127.0.0.1:10085)
    #[arg(short, long, default_value = "http://127.0.0.1:10085")]
    addr: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Restart the logger
    Restartlogger,
    /// Retrieve statistics
    Stats {
        name: String,
        #[arg(short, long)]
        reset: bool,
    },
    /// Query statistics
    Statsquery {
        pattern: String,
        #[arg(short, long)]
        reset: bool,
    },
    /// Retrieve system statistics
    Statssys,
    /// Retrieve balancer information
    Bi { tag: String },
    /// Override balancer
    Bo {
        balancer_tag: String,
        target: String,
    },
    /// Add inbounds
    Adi {
        tag: String,
        protocol: String,
        settings_json: String,
    },
    /// Add outbounds
    Ado {
        tag: String,
        protocol: String,
        settings_json: String,
    },
    /// Remove inbounds
    Rmi { tag: String },
    /// Remove outbounds
    Rmo { tag: String },
    /// List inbounds
    Lsi {
        #[arg(short, long)]
        only_tags: bool,
    },
    /// List outbounds
    Lso,
    /// Add users to inbounds
    Adu {
        tag: String,
        email: String,
        level: u32,
    },
    /// Remove users from inbounds
    Rmu { tag: String, email: String },
    /// Retrieve inbound user(s)
    Inbounduser { tag: String, email: String },
    /// Retrieve inbound user count
    Inboundusercount { tag: String, email: String },
    /// Add routing rules (Note: expects JSON config for now)
    Adrules {
        config_json: String,
        #[arg(short, long)]
        append: bool,
    },
    /// Remove routing rules by ruleTag
    Rmrules { rule_tag: String },
    /// List routing rules
    Lsrules,
    /// Block connections by source IP (implemented via routing rule)
    Sib { ip: String },
    /// Retrieve the online session count for a user
    Statsonline {
        name: String,
        #[arg(short, long)]
        reset: bool,
    },
    /// Retrieve a user's online IP addresses and access times
    Statsonlineiplist {
        name: String,
        #[arg(short, long)]
        reset: bool,
    },
    /// Retrieve array of all online users
    Statsgetallonlineusers,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let client = Client::connect(cli.addr).await?;

    match cli.command {
        Commands::Restartlogger => {
            client.logger().restart_logger().await?;
            println!("Logger restarted.");
        }
        Commands::Stats { name, reset } => {
            let stat = client.stats().get_stats(name, reset).await?;
            println!("{}: {}", stat.name, stat.value);
        }
        Commands::Statsquery { pattern, reset } => {
            let stats = client.stats().query_stats(pattern, reset).await?;
            for s in stats {
                println!("{}: {}", s.name, s.value);
            }
        }
        Commands::Statssys => {
            let sys = client.stats().get_sys_stats().await?;
            println!("{:#?}", sys);
        }
        Commands::Bi { tag } => {
            let info = client.routing().get_balancer_info(tag).await?;
            println!("{:#?}", info);
        }
        Commands::Bo {
            balancer_tag,
            target,
        } => {
            client
                .routing()
                .override_balancer_target(balancer_tag, target)
                .await?;
            println!("Balancer target overridden.");
        }
        Commands::Adi {
            tag,
            protocol,
            settings_json,
        } => {
            // In a real CLI, we would parse JSON to bytes. For simplicity:
            client
                .proxyman()
                .add_inbound(tag, protocol, settings_json.into_bytes())
                .await?;
            println!("Inbound added.");
        }
        Commands::Ado {
            tag,
            protocol,
            settings_json,
        } => {
            client
                .proxyman()
                .add_outbound(tag, protocol, settings_json.into_bytes())
                .await?;
            println!("Outbound added.");
        }
        Commands::Rmi { tag } => {
            client.proxyman().remove_inbound(tag).await?;
            println!("Inbound removed.");
        }
        Commands::Rmo { tag } => {
            client.proxyman().remove_outbound(tag).await?;
            println!("Outbound removed.");
        }
        Commands::Lsi { only_tags } => {
            let list = client.proxyman().list_inbounds(only_tags).await?;
            for i in list {
                println!("{}", i.tag);
            }
        }
        Commands::Lso => {
            let list = client.proxyman().list_outbounds().await?;
            for o in list {
                println!("{}", o.tag);
            }
        }
        Commands::Adu { tag, email, level } => {
            client
                .proxyman()
                .add_user(tag, User { email, level })
                .await?;
            println!("User added.");
        }
        Commands::Rmu { tag, email } => {
            client.proxyman().remove_user(tag, email).await?;
            println!("User removed.");
        }
        Commands::Inbounduser { tag, email } => {
            let users = client.proxyman().get_inbound_users(tag, email).await?;
            println!("{:#?}", users);
        }
        Commands::Inboundusercount { tag, email } => {
            let count = client
                .proxyman()
                .get_inbound_users_count(tag, email)
                .await?;
            println!("Count: {}", count);
        }
        Commands::Adrules {
            config_json,
            append,
        } => {
            // Placeholder for TypedMessage construction from JSON
            println!(
                "Add rules with JSON not fully implemented in this example (requires TypedMessage mapping)."
            );
        }
        Commands::Rmrules { rule_tag } => {
            client.routing().remove_rule(rule_tag).await?;
            println!("Rule removed.");
        }
        Commands::Lsrules => {
            let rules = client.routing().list_rules().await?;
            for r in rules {
                println!("{} -> {}", r.tag, r.rule_tag);
            }
        }
        Commands::Sib { ip } => {
            println!(
                "Block IP not fully implemented (requires constructing specific RoutingRule TypedMessage)."
            );
        }
        Commands::Statsonline { name, reset } => {
            let stat = client.stats().get_stats_online(name, reset).await?;
            println!("{}: {}", stat.name, stat.value);
        }
        Commands::Statsonlineiplist { name, reset } => {
            let list = client.stats().get_stats_online_ip_list(name, reset).await?;
            println!("{:#?}", list);
        }
        Commands::Statsgetallonlineusers => {
            let users = client.stats().get_all_online_users().await?;
            for u in users {
                println!("{}", u);
            }
        }
    }

    Ok(())
}
