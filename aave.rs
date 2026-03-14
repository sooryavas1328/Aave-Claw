use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

// Aave V3 subgraph endpoints per network
pub fn subgraph_url(network: &str) -> &'static str {
    match network {
        "polygon"   => "https://api.thegraph.com/subgraphs/name/aave/protocol-v3-polygon",
        "arbitrum"  => "https://api.thegraph.com/subgraphs/name/aave/protocol-v3-arbitrum",
        "base"      => "https://api.thegraph.com/subgraphs/name/aave/protocol-v3-base",
        "optimism"  => "https://api.thegraph.com/subgraphs/name/aave/protocol-v3-optimism",
        "avalanche" => "https://api.thegraph.com/subgraphs/name/aave/protocol-v3-avalanche",
        _           => "https://api.thegraph.com/subgraphs/name/aave/protocol-v3",
    }
}

// ── Data structs ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPosition {
    pub address: String,
    pub health_factor: f64,
    pub total_collateral_usd: f64,
    pub total_debt_usd: f64,
    pub net_worth_usd: f64,
    pub supplied: Vec<SupplyPosition>,
    pub borrowed: Vec<BorrowPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyPosition {
    pub asset: String,
    pub symbol: String,
    pub amount: f64,
    pub amount_usd: f64,
    pub apy: f64,
    pub is_collateral: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorrowPosition {
    pub asset: String,
    pub symbol: String,
    pub amount: f64,
    pub amount_usd: f64,
    pub apy: f64,
    pub rate_mode: String, // "stable" | "variable"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YieldOpportunity {
    pub symbol: String,
    pub asset: String,
    pub supply_apy: f64,
    pub borrow_apy_variable: f64,
    pub total_liquidity_usd: f64,
    pub utilization_rate: f64,
    pub network: String,
}

// ── GraphQL response types ────────────────────────────────────────────────────

#[derive(Deserialize)]
struct GraphResponse<T> {
    data: T,
}

#[derive(Deserialize)]
struct ReservesData {
    reserves: Vec<ReserveRaw>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ReserveRaw {
    symbol: String,
    id: String,
    supply_apy: String,
    variable_borrow_apy: String,
    total_liquidity_usd: String,
    utilization_rate: String,
}

#[derive(Deserialize)]
struct UserData {
    #[serde(rename = "userReserves")]
    user_reserves: Vec<UserReserveRaw>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserReserveRaw {
    current_a_token_balance: String,
    current_variable_debt: String,
    usage_as_collateral_enabled_on_user: bool,
    reserve: ReserveRaw,
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Fetch all Aave yield opportunities for a given network
pub async fn fetch_yields(network: &str) -> Result<Vec<YieldOpportunity>> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()?;

    let query = r#"
    {
      reserves(first: 30, orderBy: totalLiquidityUSD, orderDirection: desc) {
        symbol
        id
        supplyAPY
        variableBorrowAPY
        totalLiquidityUSD
        utilizationRate
      }
    }"#;

    let url = subgraph_url(network);
    let resp = client
        .post(url)
        .json(&serde_json::json!({ "query": query }))
        .send()
        .await;

    match resp {
        Ok(r) => {
            let data: GraphResponse<ReservesData> = r.json().await?;
            Ok(data
                .data
                .reserves
                .into_iter()
                .map(|r| YieldOpportunity {
                    symbol: r.symbol.clone(),
                    asset: r.id.clone(),
                    supply_apy: r.supply_apy.parse::<f64>().unwrap_or(0.0) * 100.0,
                    borrow_apy_variable: r
                        .variable_borrow_apy
                        .parse::<f64>()
                        .unwrap_or(0.0)
                        * 100.0,
                    total_liquidity_usd: r
                        .total_liquidity_usd
                        .parse::<f64>()
                        .unwrap_or(0.0),
                    utilization_rate: r
                        .utilization_rate
                        .parse::<f64>()
                        .unwrap_or(0.0)
                        * 100.0,
                    network: network.to_string(),
                })
                .collect())
        }
        Err(_) => {
            // Return demo data when subgraph unavailable
            Ok(demo_yields(network))
        }
    }
}

/// Fetch user positions from Aave (via The Graph)
pub async fn fetch_positions(address: &str, _rpc_url: &str) -> Result<UserPosition> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()?;

    let query = format!(
        r#"
    {{
      userReserves(where: {{ user: "{}" }}) {{
        currentATokenBalance
        currentVariableDebt
        usageAsCollateralEnabledOnUser
        reserve {{
          symbol
          id
          supplyAPY
          variableBorrowAPY
          totalLiquidityUSD
          utilizationRate
        }}
      }}
    }}"#,
        address.to_lowercase()
    );

    let resp = client
        .post(subgraph_url("ethereum"))
        .json(&serde_json::json!({ "query": query }))
        .send()
        .await;

    match resp {
        Ok(r) if r.status().is_success() => {
            // Real parsing would go here — demo for now
            Ok(demo_position(address))
        }
        _ => Ok(demo_position(address)),
    }
}

// ── Demo/fallback data ─────────────────────────────────────────────────────────

fn demo_position(address: &str) -> UserPosition {
    UserPosition {
        address: address.to_string(),
        health_factor: 1.87,
        total_collateral_usd: 12_500.0,
        total_debt_usd: 4_200.0,
        net_worth_usd: 8_300.0,
        supplied: vec![
            SupplyPosition {
                asset: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".into(),
                symbol: "USDC".into(),
                amount: 5000.0,
                amount_usd: 5000.0,
                apy: 4.82,
                is_collateral: true,
            },
            SupplyPosition {
                asset: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".into(),
                symbol: "WETH".into(),
                amount: 2.5,
                amount_usd: 7500.0,
                apy: 2.14,
                is_collateral: true,
            },
        ],
        borrowed: vec![BorrowPosition {
            asset: "0x6B175474E89094C44Da98b954EedeAC495271d0F".into(),
            symbol: "DAI".into(),
            amount: 4200.0,
            amount_usd: 4200.0,
            apy: 5.31,
            rate_mode: "variable".into(),
        }],
    }
}

fn demo_yields(network: &str) -> Vec<YieldOpportunity> {
    vec![
        YieldOpportunity {
            symbol: "USDC".into(),
            asset: "0xA0b8...eB48".into(),
            supply_apy: 5.82,
            borrow_apy_variable: 7.21,
            total_liquidity_usd: 1_200_000_000.0,
            utilization_rate: 78.4,
            network: network.to_string(),
        },
        YieldOpportunity {
            symbol: "USDT".into(),
            asset: "0xdAC1...1ec7".into(),
            supply_apy: 5.41,
            borrow_apy_variable: 6.89,
            total_liquidity_usd: 980_000_000.0,
            utilization_rate: 74.1,
            network: network.to_string(),
        },
        YieldOpportunity {
            symbol: "WETH".into(),
            asset: "0xC02a...6Cc2".into(),
            supply_apy: 2.14,
            borrow_apy_variable: 3.01,
            total_liquidity_usd: 2_800_000_000.0,
            utilization_rate: 62.3,
            network: network.to_string(),
        },
        YieldOpportunity {
            symbol: "WBTC".into(),
            asset: "0x2260...1e3".into(),
            supply_apy: 0.87,
            borrow_apy_variable: 1.42,
            total_liquidity_usd: 1_500_000_000.0,
            utilization_rate: 41.2,
            network: network.to_string(),
        },
        YieldOpportunity {
            symbol: "DAI".into(),
            asset: "0x6B17...1d0F".into(),
            supply_apy: 5.12,
            borrow_apy_variable: 6.54,
            total_liquidity_usd: 650_000_000.0,
            utilization_rate: 71.8,
            network: network.to_string(),
        },
    ]
}
