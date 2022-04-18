# [WIP] - zkp-rs
Zero Knowledge Proof demonstration in Rust using client-server model. 

| ![CI Build and Tests](https://github.com/suren-m/zkp-rs/actions/workflows/pr-checks.yml/badge.svg) | 
| ![build-and-push-zkp-container-images](https://github.com/suren-m/zkp-rs/actions/workflows/build-and-publish-images.yml/badge.svg) |
| ![Publish Docs](https://github.com/suren-m/zkp-rs/actions/workflows/publish-docs.yml/badge.svg) |
| ![Integration Tests](https://github.com/suren-m/zkp-rs/actions/workflows/integration-tests.yml/badge.svg) |

---

## Documentation

Generated from `Cargo doc` and published to github pages

### https://suren-m.github.io/zkp-rs/

By Crate:
* [zkp_client](https://suren-m.github.io/zkp-rs/zkp_client/index.html)
* [zkp_common](https://suren-m.github.io/zkp-rs/zkp_common/index.html)
* [zkp_server](https://suren-m.github.io/zkp-rs/zkp_server/index.html)
---

## Getting Started

### 1. Docker-compose 

* Using the latest published images from ghcr.io (quick run)
    ```bash
    docker-compose -f docker-compose.prebuilt.yml pull && docker-compose -f docker-compose.prebuilt.yml up
    ```

* Or to build and Run from source 
    ```bash
    # clone this repo, and then
    docker-compose up
    ```

### 2. Helm Chart deployment on a Kubernetes / Minikube cluster

```bash
# clone this repo

# package the chart
cd _charts
helm package zkp-rs

# Set env vars
export ZKP_USERNAME=helm-demo-user
export ZKP_SECRET=15

# install
helm install zkp-rs zkp-rs-0.1.0.tgz --set username=$ZKP_USERNAME --set secret=$ZKP_SECRET --namespace=zkp-rs-demo --create-namespace

# verify both the server and client are running
kubectl get all -n zkp-rs-demo

# check logs
kubectl logs deploy/zkp-client-deploy --follow -n zkp-rs-demo
```

### 3. Local using `cargo run`

```bash
# clone this repo

# Run server
cargo run --bin zkp-server

# And then from another terminal / pane,
# set env variables. for e.g,
export ZKP_USERNAME=demouser
export ZKP_SECRET=10

# Run client
cargo run --bin zkp-client
---