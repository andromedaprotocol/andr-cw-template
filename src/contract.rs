#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{% raw %}{{% endraw %}{% unless minimal %}to_binary, {% endunless %}Binary, Deps, DepsMut, Env, MessageInfo, Response, ensure};
use andromeda_std::{
    ado_base::InstantiateMsg as BaseInstantiateMsg,
    ado_contract::{
        permissioning::{is_context_permissioned},
        ADOContract,
    },
    common::context::ExecuteContext,
    error::ContractError,
};
{% if minimal %}// {% endif %}use cw2::set_contract_version;

use crate::msg::{ExecuteMsg, {% unless minimal %}GetCountResponse, {% endunless %}InstantiateMsg, QueryMsg};
{% unless minimal %}use crate::state::{State, STATE};
{% endunless %}
{% if minimal %}/*
{% endif %}// version info for migration info
const CONTRACT_NAME: &str = "crates.io:{{project-name}}";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
{% if minimal %}*/
{% endif %}
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    let contract = ADOContract::default();

    let resp = contract.instantiate(
        deps.storage,
        env,
        deps.api,
        info.clone(),
        BaseInstantiateMsg {
            ado_type: "{{project-name}}".to_string(),
            ado_version: CONTRACT_VERSION.to_string(),
            operators: None,
            kernel_address: msg.kernel_address,
            owner: msg.owner,
        },
    )?;

    Ok(resp
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result<Response, ContractError> {
    let ctx = ExecuteContext::new(deps, info, env);
    if let ExecuteMsg::AMPReceive(pkt) = msg {
        ADOContract::default().execute_amp_receive(
            ctx,
            pkt,
            handle_execute,
        )
    } else {
        handle_execute(ctx, msg)
    }
}

pub fn handle_execute(
    ctx: ExecuteContext,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    {% if permissioned %}
    ensure!(
        is_context_permissioned(
            ctx.deps.storage,
            &ctx.info,
            &ctx.env,
            &ctx.amp_ctx,
            msg.as_ref()
        )?,
        ContractError::Unauthorized {}
    );
    {% endif %}
    match msg {
        {% if !minimal %}ExecuteMsg::Increment {} => execute::increment(ctx),
        ExecuteMsg::Reset { count } => execute::reset(ctx, count),{% endif %}
        _ => ADOContract::default().execute(ctx, msg)
    }
}{% unless minimal %}

pub mod execute {
    use super::*;

    pub fn increment(ctx: ExecuteContext) -> Result<Response, ContractError> {
        let ExecuteContext { deps, .. } = ctx;
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            state.count += 1;
            Ok(state)
        })?;

        Ok(Response::new().add_attribute("action", "increment"))
    }

    pub fn reset(ctx: ExecuteContext, count: i32) -> Result<Response, ContractError> {
        let ExecuteContext { deps, info, .. } = ctx;
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {});
            }
            state.count = count;
            Ok(state)
        })?;
        Ok(Response::new().add_attribute("action", "reset"))
    }
}{% endunless %}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        {% if !minimal %}QueryMsg::GetCount {} => Ok(to_binary(&query::count(deps)?)?),{% endif %}
        _ => ADOContract::default().query(deps, env, msg),
    }
}{% unless minimal %}

pub mod query {
    use super::*;

    pub fn count(deps: Deps) -> Result<GetCountResponse, ContractError> {
        let state = STATE.load(deps.storage)?;
        Ok(GetCountResponse { count: state.count })
    }
}{% endunless %}

#[cfg(test)]
mod tests {% raw %}{{% endraw %}{% unless minimal %}
    use super::*;
    use andromeda_std::testing::mock_querier::MOCK_KERNEL_CONTRACT;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            count: 17,
            kernel_address: MOCK_KERNEL_CONTRACT.to_string(),
            owner: None,
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            count: 17,
            kernel_address: MOCK_KERNEL_CONTRACT.to_string(),
            owner: None,
        };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            count: 17,
            kernel_address: MOCK_KERNEL_CONTRACT.to_string(),
            owner: None,
        };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
}
{% endunless %}}
