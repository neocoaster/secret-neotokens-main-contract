use cosmwasm_std::{
    debug_print, to_binary, Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse,
    Querier, StdError, StdResult, Storage, Uint128,
};

use crate::msg::{
    ClaimCreditsResponse, CreditsResponse, HandleMsg, InitMsg, OracleCreditsResponse,
    OracleQueryMsg, QueryMsg,
};
use crate::state::{
    config, config_read, credits_history_storage, credits_history_storage_read, State,
};
use secret_toolkit::utils::Query;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        owner: deps.api.canonical_address(&env.message.sender)?,
        oracle_contract: msg.oracle_contract,
        token_contract: msg.token_contract,
    };

    config(&mut deps.storage).save(&state)?;

    debug_print!("Contract was initialized by {}", env.message.sender);

    Ok(InitResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAvailableCredits { address } => {
            to_binary(&try_get_available_credits(deps, address)?)
        }
    }
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Claim {} => try_claim_credits(deps, env),
    }
}

pub fn try_claim_credits<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    let state = config_read(&deps.storage).load()?;
    let available_credits = try_get_available_credits(deps, env.message.sender.clone())?;
    let to_claim_credits = available_credits.available_credits;

    if to_claim_credits > 0 {
        if let Some(new_total_claimed) =
            (to_claim_credits).checked_add(available_credits.already_claimed_credits)
        {
            let address_canonical = deps.api.canonical_address(&env.message.sender)?;
            let _credits = credits_history_storage(&mut deps.storage)
                .save(address_canonical.as_slice(), &new_total_claimed);
        } else {
            return Err(StdError::generic_err("Historical claim would exceed limit"));
        }

        let available_credits: Uint128 = Uint128::from(to_claim_credits);

        Ok(HandleResponse {
            messages: vec![state
                .token_contract
                .mint_msg(env.message.sender, available_credits)?],
            log: vec![],
            data: Some(to_binary(&ClaimCreditsResponse {
                total_claimed: available_credits,
            })?),
        })
    } else {
        let zero: u64 = 0;
        Ok(HandleResponse {
            messages: vec![],
            log: vec![],
            data: Some(to_binary(&ClaimCreditsResponse {
                total_claimed: Uint128::from(zero),
            })?),
        })
    }
}

fn try_get_available_credits<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    address: HumanAddr,
) -> StdResult<CreditsResponse> {
    let address_canonical = deps.api.canonical_address(&address)?;
    let state = config_read(&deps.storage).load()?;
    let user_historical_claim =
        match credits_history_storage_read(&deps.storage).load(address_canonical.as_slice()) {
            Ok(historical_claim) => historical_claim,
            Err(_error) => 0,
        };

    let get_oracle_credits = OracleQueryMsg::GetCredits { address };
    let oracle_response: OracleCreditsResponse = get_oracle_credits.query(
        &deps.querier,
        state.oracle_contract.code_hash,
        state.oracle_contract.address,
    )?;

    let available_credits = if oracle_response.owned_credits <= user_historical_claim {
        0
    } else {
        oracle_response.owned_credits - user_historical_claim
    };

    Ok(CreditsResponse {
        available_credits,
        already_claimed_credits: user_historical_claim,
    })
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use cosmwasm_std::testing::{mock_dependencies, mock_env};
    // use cosmwasm_std::{coins, from_binary};

    // TODO WRITE SPECS
}
