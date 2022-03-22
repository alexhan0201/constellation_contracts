use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map, U128Key};
use crate::msg::{UserInfo, PoolInfo};

pub const OPERATOR: Item<Addr> = Item::new("OPERATOR");
pub const BALANCES: Map<Addr, Uint128> = Map::new("balances");

pub const NAME: String = Item::new("NAME");
pub const SYMBOL: String = Item::new("SYMBOL");
pub const TOTALSUPPLY: Uint128 = Item::new("TOTALSUPPLY");
pub const ALLOWANCES: Map<Addr, Map<Addr,Uint128>> = Item:new("ALLOWANCES");