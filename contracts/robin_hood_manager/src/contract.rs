use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, LAST_REGISTERED_ADDR, OBJECTS, POOR, RICH};
use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Reply,
    Response, StdError, SubMsg,
};
use neutron_sdk::{
    bindings::{
        msg::{MsgRegisterInterchainQueryResponse, NeutronMsg},
        query::NeutronQuery,
    },
    interchain_queries::v045::new_register_balance_query_msg,
    interchain_queries::v045::queries::query_balance,
    sudo::msg::SudoMsg,
    NeutronError, NeutronResult,
};

/// Reply ID used to tell this kind of reply call apart.
pub const REGISTER_BALANCES_ICQ_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> NeutronResult<Response> {
    CONFIG.save(
        deps.storage,
        &Config {
            owner: info.sender,
            rich_line: msg.rich_line,
            asset_denom: msg.asset_denom,
            frequency: msg.frequency,
            connection_id: msg.connection_id,
        },
    )?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<NeutronQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> NeutronResult<Response<NeutronMsg>> {
    match msg {
        ExecuteMsg::KeepAnEyeOn { addr } => keep_an_eye_on(deps, addr),
    }
}

/// Registers a balance ICQ for a given address.
pub fn keep_an_eye_on(
    deps: DepsMut<NeutronQuery>,
    addr: String,
) -> NeutronResult<Response<NeutronMsg>> {
    // Save a given address to LAST_REGISTERED_ADDR to handle it later in Reply handler
    LAST_REGISTERED_ADDR.save(deps.storage, &Addr::unchecked(addr.clone()))?;

    // Construct an ICQ registration message based on contract's config and passed arguments
    let conf: Config = CONFIG.load(deps.storage)?;
    let msg =
        new_register_balance_query_msg(conf.connection_id, addr, conf.asset_denom, conf.frequency)?;

    // Send the ICQ registration message as a submessage to receive a reply callback
    Ok(Response::new().add_submessage(SubMsg {
        id: REGISTER_BALANCES_ICQ_REPLY_ID,
        msg: CosmosMsg::Custom(msg),
        gas_limit: None,
        reply_on: cosmwasm_std::ReplyOn::Success,
    }))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<NeutronQuery>, _env: Env, msg: QueryMsg) -> NeutronResult<Binary> {
    match msg {
        QueryMsg::Objects {} => query_objects(deps),
        QueryMsg::Rich {} => query_rich(deps),
        QueryMsg::Poor {} => query_poor(deps),
        QueryMsg::Config {} => query_config(deps),
    }
}

/// Returns encoded objects stored in the contract's state.
fn query_objects(deps: Deps<NeutronQuery>) -> NeutronResult<Binary> {
    let objects = OBJECTS
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|v| Ok(v?.1))
        .collect::<Result<Vec<Addr>, StdError>>()?;
    Ok(to_binary(&objects)?)
}

/// Returns encoded rich objects stored in the contract's state.
fn query_rich(deps: Deps<NeutronQuery>) -> NeutronResult<Binary> {
    let rich = RICH
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .collect::<Result<Vec<(Addr, u64)>, StdError>>()?;
    Ok(to_binary(&rich)?)
}

/// Returns encoded poor objects stored in the contract's state.
fn query_poor(deps: Deps<NeutronQuery>) -> NeutronResult<Binary> {
    let poor = POOR
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .collect::<Result<Vec<(Addr, u64)>, StdError>>()?;
    Ok(to_binary(&poor)?)
}

/// Returns encoded current contract's configuration.
pub fn query_config(deps: Deps<NeutronQuery>) -> NeutronResult<Binary> {
    let config = CONFIG.load(deps.storage)?;
    Ok(to_binary(&config)?)
}

#[entry_point]
pub fn sudo(deps: DepsMut<NeutronQuery>, env: Env, msg: SudoMsg) -> NeutronResult<Response> {
    match msg {
        SudoMsg::KVQueryResult { query_id } => sudo_kv_query_result(deps, env, query_id),
        _ => Ok(Response::default()),
    }
}

/// Contract's callback for KV query results. Handles a balance ICQ result by declaring the
/// respective address' owner a rich or a poor.
pub fn sudo_kv_query_result(
    deps: DepsMut<NeutronQuery>,
    env: Env,
    query_id: u64,
) -> NeutronResult<Response> {
    // Get the last submitted ICQ result from the Neutron ICQ module storage
    let balance_resp = query_balance(deps.as_ref(), env.clone(), query_id)?;

    // Get the remote chain address used for the ICQ with a given query ID
    let addr_behind_query = OBJECTS.load(deps.storage, query_id)?;

    let conf: Config = CONFIG.load(deps.storage)?;

    // Since balance_resp.balances.coins is an array of coins, we need to find the coin with
    // the conf.asset_denom as denom.
    let balance = balance_resp
        .balances
        .coins
        .iter()
        .find(|b| b.denom == conf.asset_denom);

    if let Some(balance) = balance {
        if balance.amount.ge(&conf.rich_line) {
            RICH.save(deps.storage, addr_behind_query, &env.block.height)?;
        } else {
            POOR.save(deps.storage, addr_behind_query, &env.block.height)?;
        }
    }

    Ok(Response::default())
}

#[entry_point]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> NeutronResult<Response> {
    match msg.id {
        // Bind a given ICQ ID with the last registered address.
        REGISTER_BALANCES_ICQ_REPLY_ID => {
            // decode the reply msg payload as MsgRegisterInterchainQueryResponse
            let resp: MsgRegisterInterchainQueryResponse = serde_json_wasm::from_slice(
                msg.result
                    .into_result()
                    .map_err(StdError::generic_err)?
                    .data
                    .ok_or_else(|| StdError::generic_err("no result"))?
                    .as_slice(),
            )
            .map_err(|e| StdError::generic_err(format!("failed to parse response: {:?}", e)))?;

            // load the pre-set address used in the ICQ we just registered
            let last_registered_addr = LAST_REGISTERED_ADDR.load(deps.storage)?;
            OBJECTS.save(deps.storage, resp.id, &last_registered_addr)?;

            Ok(Response::new())
        }

        _ => Err(NeutronError::Std(StdError::generic_err(format!(
            "unsupported reply message id {}",
            msg.id
        )))),
    }
}
