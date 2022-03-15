use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map, U128Key};
use crate::msg::{Masonseat, MasonrySnapshot};

//---ShareWrapper----------------------------------
pub const SHARE: Item<Addr> = Item::new("share");
pub const TOTALSUPPLY: Item<Uint128> = Item::new("total supply");

//---Masonry-------------------------------------
pub const OPERATOR: Item<Addr> = Item::new("operator");
pub const INITIALIZED: Item<bool> = Item::new("initialized");
pub const TOMB: Item<Addr> = Item::new("tomb");
pub const TREASURY: Item<Addr> = Item::new("treasury");

pub const MASONS: Map<Addr, Masonseat> = Map::new("masons");
pub const MASONRY_HISTORY: Item<Vec<MasonrySnapshot>> = Item::new("masonry_history");
pub const WITHDRAW_LOCKUP_EPOCHS: Item<Uint128> = Item::new("withdraw_lockup_epochs");
pub const REWARD_LOCKUP_EPOCHS: Item<Uint128> = Item::new("reward_lockup_epochs");