use super::*;
use cosmwasm_std::{from_binary, Uint128, Addr};
use cosmwasm_std::testing::{mock_env, mock_info, MOCK_CONTRACT_ADDR};

use crate::contract::{execute, instantiate};
use crate::query::{query};
use crate::msg::{QueryMsg, ExecuteMsg, InstantiateMsg, PoolInfo, UserInfo};

use crate::mock_querier::mock_dependencies;

#[test]
fn workflow(){
    let mut deps = mock_dependencies(&[]);
    
    let msg = InstantiateMsg{
        TOMB: "tomb".to_string(),
        SHIBA: "shiba".to_string(),
        POOLSTARTTIME: Uint128::from(mock_env().block.time.seconds() + 1000)
    };
//instantiate
    let info = mock_info("admin", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

// 
    let msg = ExecuteMsg::BurnFrom{
        account: Addr::unchecked("".to_string()),
        amount: Uint128::zero()
    };

    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    println!("Burn{:?}", res);
// 
    let msg = ExecuteMsg::Mint{
        recipient: Addr::unchecked("".to_string()),
        amount: Uint128::zero()
    };

    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    println!("Mint{:?}", res);
}

