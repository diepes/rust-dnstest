# Dingo

Domain INformation Gatherer, Obviously.
* forked from adamchalmers/dingo (2023)

* Native rust dns msg parsing
* repeat lookup and display basic stats, min/max etc
* docker container - for testing dns in k8s

## Installation

1. Install cargo, see [instructions on the Rust website](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. Install or Run
  a. Build & Install exe $ ./install.sh (to install it just does cargo build and copies the program to `/usr/local/bin/dingo`)
  b. Build & run $ cargo run -- -i 2 www.google.com

## Examples

```sh
$ dingo google.com -i 1

# Output
time:  7ms min:6  max:8  ave:7.1  cnt:029 fail:0 Q:"A: google.com." R:"1.1.1.1:53" Ans:"A: 142.250.204.14 (TTL 279)..."
time:  7ms min:6  max:8  ave:7.1  cnt:030 fail:0 Q:"A: google.com." R:"1.1.1.1:53" Ans:"A: 142.250.204.14 (TTL 249)..."
time:  7ms min:6  max:8  ave:7.1  cnt:031 fail:0 Q:"A: google.com." R:"1.1.1.1:53" Ans:"A: 172.217.24.46 (TTL 264)...."

## Usage

```bash

dingo [OPTIONS] --record-type TYPE NAME

FLAGS:
  -h, --help                Prints help information
OPTIONS:
  -t, --record-type TYPE    Choose the DNS record type (supports A, CNAME, SOA and AAAA) (default A)
  -r, --resolver IP         Which DNS resolver to query (default is 1.1.1.1:53)
  -i, --interval Seconds    If specified repeats and sleeps interval seconds between dns queries.
ARGS:
  NAME A domain name to look up. Remember, these must be ASCII.(Default google.com)

```
## Run in k8s(Kubernetes)

Run container in k8s
run full debug container:

    kubectl run  -n kube-system dns-test --image=docker.io/diepes/dns-test:latest -- -i 1

Monitor with

    kubectl logs -n kube-system dns-test -f
