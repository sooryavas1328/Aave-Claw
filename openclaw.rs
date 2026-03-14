//! OpenClaw Skill Server
//!
//! Exposes aave-claw as an HTTP skill that OpenClaw can call.
//! Spec: https://openclaw.dev/skills

use anyhow::Result;
use colored::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::net::TcpListener;

use crate::aave;
use crate::config::Config;

// ── OpenClaw skill manifest ───────────────────────────────────────────────────

#[derive(Serialize)]
struct SkillManifest {
    name: String,
    version: String,
    description: String,
    author: String,
    repository: String,
    tools: Vec<SkillTool>,
}

#[derive(Serialize)]
struct SkillTool {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

fn build_manifest() -> SkillManifest {
    SkillManifest {
        name: "aave-claw".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        description: "Monitor Aave DeFi positions, health factors, and yield opportunities across multiple networks.".into(),
        author: "susanudgzf".into(),
        repository: "https://github.com/susanudgzf/Aave-Claw".into(),
        tools: vec![
            SkillTool {
                name: "get_positions".into(),
                description: "Get all Aave supply/borrow positions and health factor for an Ethereum address.".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "required": ["address"],
                    "properties": {
                        "address": {
                            "type": "string",
                            "description": "Ethereum wallet address (0x...)"
                        }
                    }
                }),
            },
            SkillTool {
                name: "get_yields".into(),
                description: "Get current Aave yield (APY) opportunities for a network.".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "network": {
                            "type": "string",
                            "enum": ["ethereum", "polygon", "arbitrum", "base", "optimism", "avalanche"],
                            "default": "ethereum"
                        }
                    }
                }),
            },
            SkillTool {
                name: "check_health".into(),
                description: "Check if a position's health factor is below a threshold.".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "required": ["address"],
                    "properties": {
                        "address": { "type": "string" },
                        "threshold": { "type": "number", "default": 1.2 }
                    }
                }),
            },
        ],
    }
}

// ── Request/response types ────────────────────────────────────────────────────

#[derive(Deserialize)]
struct ToolRequest {
    tool: String,
    parameters: HashMap<String, serde_json::Value>,
}

#[derive(Serialize)]
struct ToolResponse {
    success: bool,
    data: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

// ── Minimal HTTP server (no framework dep) ───────────────────────────────────

pub async fn run_skill_server(port: u16) -> Result<()> {
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr).await?;

    println!(
        "{} OpenClaw skill running at {}",
        "⚡".yellow(),
        format!("http://{}", addr).bright_purple().underline()
    );
    println!(
        "  Manifest : {}",
        format!("http://{}/.well-known/skill.json", addr).dimmed()
    );
    println!(
        "  Execute  : {}\n",
        format!("http://{}/execute", addr).dimmed()
    );

    loop {
        match listener.accept().await {
            Ok((stream, _peer)) => {
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(stream).await {
                        tracing::warn!("Connection error: {}", e);
                    }
                });
            }
            Err(e) => tracing::warn!("Accept error: {}", e),
        }
    }
}

async fn handle_connection(mut stream: tokio::net::TcpStream) -> Result<()> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let mut buf = vec![0u8; 8192];
    let n = stream.read(&mut buf).await?;
    let req_str = String::from_utf8_lossy(&buf[..n]);

    // Parse request line
    let first_line = req_str.lines().next().unwrap_or("");
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    if parts.len() < 2 {
        return Ok(());
    }
    let method = parts[0];
    let path = parts[1];

    let (status, body) = match (method, path) {
        ("GET", "/.well-known/skill.json") | ("GET", "/skill.json") => {
            let manifest = build_manifest();
            (200, serde_json::to_string_pretty(&manifest)?)
        }
        ("GET", "/health") => (200, r#"{"status":"ok"}"#.to_string()),
        ("POST", "/execute") => {
            // Extract JSON body (after \r\n\r\n)
            let body_start = req_str.find("\r\n\r\n").map(|i| i + 4).unwrap_or(0);
            let body_str = &req_str[body_start..];

            match serde_json::from_str::<ToolRequest>(body_str) {
                Ok(req) => {
                    let result = dispatch_tool(&req).await;
                    (200, serde_json::to_string(&result)?)
                }
                Err(e) => {
                    let resp = ToolResponse {
                        success: false,
                        data: serde_json::Value::Null,
                        error: Some(format!("Invalid request: {}", e)),
                    };
                    (400, serde_json::to_string(&resp)?)
                }
            }
        }
        _ => (404, r#"{"error":"not found"}"#.to_string()),
    };

    let response = format!(
        "HTTP/1.1 {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\n\r\n{}",
        status,
        body.len(),
        body
    );

    stream.write_all(response.as_bytes()).await?;
    Ok(())
}

async fn dispatch_tool(req: &ToolRequest) -> ToolResponse {
    let cfg = Config::load().unwrap_or_default();

    match req.tool.as_str() {
        "get_positions" => {
            let address = match req.parameters.get("address").and_then(|v| v.as_str()) {
                Some(a) => a.to_string(),
                None => {
                    return ToolResponse {
                        success: false,
                        data: serde_json::Value::Null,
                        error: Some("Missing required parameter: address".into()),
                    }
                }
            };
            match aave::fetch_positions(&address, &cfg.rpc_url).await {
                Ok(pos) => ToolResponse {
                    success: true,
                    data: serde_json::to_value(&pos).unwrap_or_default(),
                    error: None,
                },
                Err(e) => ToolResponse {
                    success: false,
                    data: serde_json::Value::Null,
                    error: Some(e.to_string()),
                },
            }
        }
        "get_yields" => {
            let network = req
                .parameters
                .get("network")
                .and_then(|v| v.as_str())
                .unwrap_or("ethereum");
            match aave::fetch_yields(network).await {
                Ok(yields) => ToolResponse {
                    success: true,
                    data: serde_json::to_value(&yields).unwrap_or_default(),
                    error: None,
                },
                Err(e) => ToolResponse {
                    success: false,
                    data: serde_json::Value::Null,
                    error: Some(e.to_string()),
                },
            }
        }
        "check_health" => {
            let address = match req.parameters.get("address").and_then(|v| v.as_str()) {
                Some(a) => a.to_string(),
                None => {
                    return ToolResponse {
                        success: false,
                        data: serde_json::Value::Null,
                        error: Some("Missing required parameter: address".into()),
                    }
                }
            };
            let threshold = req
                .parameters
                .get("threshold")
                .and_then(|v| v.as_f64())
                .unwrap_or(1.2);

            match aave::fetch_positions(&address, &cfg.rpc_url).await {
                Ok(pos) => {
                    let at_risk = pos.health_factor < threshold;
                    ToolResponse {
                        success: true,
                        data: serde_json::json!({
                            "address": address,
                            "health_factor": pos.health_factor,
                            "threshold": threshold,
                            "at_risk": at_risk,
                            "status": if at_risk { "DANGER" } else { "SAFE" }
                        }),
                        error: None,
                    }
                }
                Err(e) => ToolResponse {
                    success: false,
                    data: serde_json::Value::Null,
                    error: Some(e.to_string()),
                },
            }
        }
        unknown => ToolResponse {
            success: false,
            data: serde_json::Value::Null,
            error: Some(format!("Unknown tool: {}", unknown)),
        },
    }
}
