# nnw-helper

Contains commands, files and links made to ease communication during Neutron Nebular workshop.

## Golang installation

Follow the link to the official golang website: https://go.dev/doc/install

## Neutron query relayer

Clone the repo from here: https://github.com/neutron-org/neutron-query-relayer

## Neutron core

1. Clone the repo from here: https://github.com/neutron-org/neutron
2. cd to it and run `make install`
3. make sure neutrond binary is available by running `neutrond version`

## Jq

Jq is a command-line JSON processor. Follow the link to install it: https://jqlang.github.io/jq/download/

## Configure default values for neutron core

1. `neutrond config chain-id pion-1`
2. `neutrond config keyring-backend test`
3. `neutrond config output json`
4. `neutrond config node https://rpc-palvus.pion-1.ntrn.tech:443`
5. `neutrond config broadcast-mode block`

#### Create a key pair

```sh
neutrond keys add nnw-wallet
```

#### Instantiate a contract's instance

Contact's code has already been stored under ID = 1220.

```sh
neutrond tx wasm instantiate 1220 '{
  "rich_line": "1000000",
  "asset_denom": "uatom",
  "frequency": 60,
  "connection_id": "connection-53"
}' --from nnw-wallet --admin nnw-wallet --label rhm --gas 5000000 --gas-prices 0.025untrn | jq .
```

## Relayer configuration

1. copy the `.env` file from here to the relayer app root folder:

```sh
cp .env ../neutron-query-relayer
```

2. put your contract's address to the RELAYER_REGISTRY_ADDRESSES var:

```
RELAYER_REGISTRY_ADDRESSES=neutron1...
```

## Faucet

Use the previously generated `nnw-wallet` address instead of the `<addr>` placeholder.

#### Discord:

1. visit https://discord.neutron.org
2. find `nnw-faucet` channel in the `NEBULAR` section
3. type: `$request <addr>`

#### Telegram

1. join channel https://t.me/+PzE7GmHOVWxhNTU6
2. type: `/request <addr>`

## Register ICQs

We'll look after two addresses and check their wealth:

1. poor Alice — `cosmos1p624fu7ywzxaty6w9cl5j3cj0u6rzzlk8qymc8`
2. rich Bob — `cosmos1deqmwmwfwkgwulu8yyqcwdhs5mtmy2gfzhyyak`

Use you contract's address instead of the `<addr>` placeholder

```sh
neutrond tx wasm execute <addr> '{"keep_an_eye_on":{"addr": "cosmos1p624fu7ywzxaty6w9cl5j3cj0u6rzzlk8qymc8"}}' --from nnw-wallet --gas 5000000 --gas-prices 0.025untrn | jq .
```

1. Look after Bob

```sh
neutrond tx wasm execute <addr> '{"keep_an_eye_on":{"addr": "cosmos1deqmwmwfwkgwulu8yyqcwdhs5mtmy2gfzhyyak"}}' --from nnw-wallet --gas 5000000 --gas-prices 0.025untrn | jq .
```

## Check 