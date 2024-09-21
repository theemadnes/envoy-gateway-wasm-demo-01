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

### now add the envoy filters defined in ef_filter_yaml

```
kubectl apply -f ef_filter_yaml/

# test to make sure both request audience header and response header are added
curl --verbose -i --header "Host: www.example.com" http://$GATEWAY_HOST/get
*   Trying 34.27.59.231:80...
* Connected to 34.27.59.231 (34.27.59.231) port 80
* using HTTP/1.x
> GET /get HTTP/1.1
> Host: www.example.com
> User-Agent: curl/8.10.1
> Accept: */*
> 
* Request completely sent off
< HTTP/1.1 200 OK
HTTP/1.1 200 OK
< content-type: application/json
content-type: application/json
< x-content-type-options: nosniff
x-content-type-options: nosniff
< date: Sat, 21 Sep 2024 03:47:32 GMT
date: Sat, 21 Sep 2024 03:47:32 GMT
< content-length: 528
content-length: 528
< x-custom-header: added-by-wasm-plugin
x-custom-header: added-by-wasm-plugin
< 

{
 "path": "/get",
 "host": "www.example.com",
 "method": "GET",
 "proto": "HTTP/1.1",
 "headers": {
  "Accept": [
   "*/*"
  ],
  "User-Agent": [
   "curl/8.10.1"
  ],
  "X-Envoy-External-Address": [
   "34.136.209.54"
  ],
  "X-Forwarded-For": [
   "34.136.209.54"
  ],
  "X-Forwarded-Proto": [
   "http"
  ],
  "X-Hello": [
   "Hello world from www.example.com"
  ],
  "X-Request-Id": [
   "24132c0e-30cd-454a-ae25-b10e3588fed3"
  ]
 },
 "namespace": "eg",
 "ingress": "",
 "service": "",
 "pod": "backend-6fd745bd84-5b79s"
* Connection #0 to host 34.27.59.231 left intact
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