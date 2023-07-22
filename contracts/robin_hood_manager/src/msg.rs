use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// InstantiateMsg contains initial configuration parameters for a new contract instance.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {
    /// A line which separates the rich and the poor. If one possesses more or equal assets than is
    /// defined by the line, they're rich.
    pub rich_line: Uint128,
    /// The asset which is taken into account when deciding whether one is rich or not.
    pub asset_denom: String,
    /// How often the manager evaluates one's richness.
    pub frequency: u64,
    /// Connection ID to identify a query associated IBC light client which will be used in
    /// crypto-proving the query results.
    pub connection_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Keep an eye on someone's pockets.
    KeepAnEyeOn { addr: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Get a list of people the manager looks after.
    Objects {},
    /// Get a list of rich people among all observed objects.
    Rich {},
    /// Get a list of poor people among all observed objects.
    Poor {},
    /// Returns the current contract's configuration.
    Config {},
}
