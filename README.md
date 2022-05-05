# COMPILE AND OPTIMIZE RUST CODE:
`cargo wasm`

`docker run --rm -v "$(pwd)":/contract --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry enigmampc/secret-contract-optimizer`

# START BLOCKCHAIN:

`docker run -d -it -p 9091:9091 -p 26657:26657 -p 13170:1317 -p 5000:5000 -v $(pwd):/root/code --name localsecret ghcr.io/scrtlabs/localsecret`

# CONNECT TO NODE:

`docker exec -it localsecret /bin/bash`

# INSIDE NODE:

`cd code`

`secretd tx compute store contract.wasm.gz --from a --gas 1000000 -y --keyring-backend test`

`INIT='{"oracle_contract": {"address":"<address>", "code_hash":"<code_hash>"}, "token_contract": {"address":"<address>", "code_hash":"<code_hash>"}}'`

`CODE_ID=<code_id>`

`secretd tx compute instantiate $CODE_ID "$INIT" --from a --label "neotokens-main-contract" -y --keyring-backend test`

`secretd query compute list-contract-by-code <code_id>`

# RUN TRANSACTIONS

``secretd query compute query $MAIN_CONTRACT '{"get_available_credits": { "address": "secret1fc3fzy78ttp0lwuujw7e52rhspxn8uj52zfyne" }}'``

`secretd tx compute execute $MAIN_CONTRACT '{"claim": {}}' --from a --keyring-backend test`



