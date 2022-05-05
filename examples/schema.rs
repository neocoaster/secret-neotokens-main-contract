use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use neotokens_main_contract::msg::{
    ClaimCreditsResponse, ContractInfo, CreditsResponse, HandleMsg, InitMsg, OracleCreditsResponse,
    OracleQueryMsg, QueryMsg, Token,
};
use neotokens_main_contract::state::State;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InitMsg), &out_dir);
    export_schema(&schema_for!(HandleMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(State), &out_dir);
    export_schema(&schema_for!(CreditsResponse), &out_dir);
    export_schema(&schema_for!(ClaimCreditsResponse), &out_dir);
    export_schema(&schema_for!(ContractInfo), &out_dir);
    export_schema(&schema_for!(OracleCreditsResponse), &out_dir);
    export_schema(&schema_for!(OracleQueryMsg), &out_dir);
    export_schema(&schema_for!(Token), &out_dir);
}
