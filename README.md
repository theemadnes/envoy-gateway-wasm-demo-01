# envoy-gateway-wasm-demo-01
using envoy gateway + WASM as a demo

### deploy envoy gateway

```
kubectl apply --server-side -f https://github.com/envoyproxy/gateway/releases/download/v1.1.1/install.yaml --force-conflicts

brew install egctl 

kubectl create ns eg

kubectl apply -f https://github.com/envoyproxy/gateway/releases/download/v1.1.1/quickstart.yaml -n eg

export GATEWAY_HOST=$(kubectl -n eg get gateway/eg -o jsonpath='{.status.addresses[0].value}')

curl --verbose --header "Host: www.example.com" http://$GATEWAY_HOST/get

```

### scratch commands

```
rustup target add wasm32-unknown-unknown
cargo init --lib

### add this to Cargo.toml
[dependencies]
log = "0.4.8"
proxy-wasm = "0.1.0" # The Rust SDK for proxy-wasm

[lib]
path = "src/lib.rs"
crate-type = ["cdylib"]


cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/ef_wasm_rust_add_response_header.wasm ./

shasum -a 256 ef_wasm_rust_add_response_header.wasm

```