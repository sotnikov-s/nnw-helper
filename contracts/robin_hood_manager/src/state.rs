use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Config of the contract.
pub const CONFIG: Item<Config> = Item::new("config");

/// Contains addresses of all observed objects mapped to respective Interchain query ID.
pub const OBJECTS: Map<u64, Addr> = Map::new("objects");

/// Contains addresses of the rich observed objects as key and the evaluation height as value.
pub const RICH: Map<Addr, u64> = Map::new("rich");

/// Contains addresses of the poor observed objects as key and the evaluation height as value.
pub const POOR: Map<Addr, u64> = Map::new("poor");

/// The last address passed to the balances ICQ registration message. Used in the reply handler.
pub const LAST_REGISTERED_ADDR: Item<Addr> = Item::new("last_registered_addr");

/// Contains contract's configurable parameters.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct Config {
    /// Owner is capable of declaring new observed objects.
    pub owner: Addr,
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
