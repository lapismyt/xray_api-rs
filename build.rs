use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let proto_dir = PathBuf::from("proto/Xray-core");

    // 1. Clone Xray-core if it doesn't exist
    if !proto_dir.exists() {
        println!("cargo:warning=Cloning Xray-core repository...");
        let status = Command::new("git")
            .args([
                "clone",
                "--depth",
                "1",
                "https://github.com/XTLS/Xray-core.git",
                proto_dir.to_str().unwrap(),
            ])
            .status()?;

        if !status.success() {
            return Err(format!("Failed to clone Xray-core: exit code {:?}", status.code()).into());
        }
    }

    // 2. Define which command.proto files we want to compile
    // These are the main gRPC service definitions in Xray-core
    let proto_files = [
        "app/proxyman/command/command.proto",
        "app/stats/command/command.proto",
        "app/router/command/command.proto",
        "app/dns/command/command.proto",
        "app/log/command/config.proto",
        "app/observatory/command/command.proto",
        "proxy/vmess/inbound/config.proto",
        "proxy/vless/inbound/config.proto",
        "proxy/shadowsocks/config.proto",
        "proxy/trojan/config.proto",
    ];

    let proto_paths: Vec<PathBuf> = proto_files
        .iter()
        .map(|f| proto_dir.join(f))
        .filter(|p| p.exists())
        .collect();

    if proto_paths.is_empty() {
        return Err("No command.proto files found in Xray-core directory".into());
    }

    // 3. Compile with tonic-prost-build
    tonic_prost_build::configure()
        .build_server(false) // Usually we only need the client for API
        .build_client(true)
        .out_dir(&out_dir)
        .compile_protos(&proto_paths, &[proto_dir])?;

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=proto/Xray-core");

    Ok(())
}
