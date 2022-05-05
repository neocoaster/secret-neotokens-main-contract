use cosmwasm_std::{CosmosMsg, HumanAddr, StdResult, Uint128};
use schemars::JsonSchema;
use secret_toolkit::snip20::mint_msg;
use secret_toolkit::utils::Query;
use serde::{Deserialize, Serialize};

const BLOCK_SIZE: usize = 256;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub oracle_contract: ContractInfo,
    pub token_contract: Token,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    Claim {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetAvailableCredits { address: HumanAddr },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CreditsResponse {
    pub available_credits: u64,
    pub already_claimed_credits: u64,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ClaimCreditsResponse {
    pub total_claimed: Uint128,
}

// NEEDED FOR ORACLE COMMUNICATION

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractInfo {
    /// contract's code hash string
    pub code_hash: String,
    /// contract's address
    pub address: HumanAddr,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OracleCreditsResponse {
    pub owned_credits: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OracleQueryMsg {
    GetCredits { address: HumanAddr },
}

impl Query for OracleQueryMsg {
    const BLOCK_SIZE: usize = 256;
}

// NEEDED FOR TOKEN COMMUNICATION

/// token's contract address and TokenInfo response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Token {
    /// contract address of token
    pub address: HumanAddr,

    // code has of token contract
    pub code_hash: String,
}

impl Token {
    /// Returns a StdResult<CosmosMsg> used to execute Mint
    ///
    /// # Arguments
    ///
    /// * `recipient` - address tokens are to be minted to
    /// * `amount` - Uint128 amount of tokens to send
    pub fn mint_msg(&self, recipient: HumanAddr, amount: Uint128) -> StdResult<CosmosMsg> {
        mint_msg(
            recipient,
            amount,
            None,
            None,
            BLOCK_SIZE,
            self.code_hash.clone(),
            self.address.clone(),
        )
    }
}
