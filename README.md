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