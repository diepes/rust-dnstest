# dnstest

Domain INformation Gatherer, Obviously.
[![Docker Image CI](https://github.com/diepes/rust-dnstest/actions/workflows/docker-image.yml/badge.svg)](https://github.com/diepes/rust-dnstest/actions/workflows/docker-image.yml)

* forked from adamchalmers/dingo (2023)

* Native rust dns msg parsing - raw udp dns connections.
* repeat lookup and display basic stats, min/max etc up to 1request/second
* docker container - for testing dns response time from within in k8s, not using local dns cache.

## Quick run docker

* ```docker run -it --rm docker.io/diepes/dnstest:latest -i 1 www.microsoft.com``` 

        msec:11  min:11  max:11  ave:11.0  cnt:0001 fail:0  Q:"A: www.microsoft.com." R:"1.1.1.1:53"
        Answer records:
            CNAME: www.microsoft.com-c-3.edgekey.net. (TTL 3590)
            CNAME: www.microsoft.com-c-3.edgekey.net.globalredir.akadns.net. (TTL 890)
            CNAME: e13678.dscb.akamaiedge.net. (TTL 890)
            A: 23.212.169.169 (TTL 10)....

## Installation

1. Install cargo, see [instructions on the Rust website](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. Clone git repo ```git clone https://github.com/diepes/rust-dnstest.git```
3. Install or Run
   1. Build & Install exe ```$ ./install.sh``` (to install it just does cargo build and copies the program to `/usr/local/bin/dnstest`)
   2. Build & run ```$ cargo run -- -i 2 -s 10,50,100 -- www.google.com```

## Examples

```sh
$ dnstest -i 1 -- google.com

# Output
time:  7ms min:6  max:8  ave:7.1  cnt:029 fail:0 Q:"A: google.com." R:"1.1.1.1:53" Ans:"A: 142.250.204.14 (TTL 279)..."
time:  7ms min:6  max:8  ave:7.1  cnt:030 fail:0 Q:"A: google.com." R:"1.1.1.1:53" Ans:"A: 142.250.204.14 (TTL 249)..."
time:  7ms min:6  max:8  ave:7.1  cnt:031 fail:0 Q:"A: google.com." R:"1.1.1.1:53" Ans:"A: 172.217.24.46 (TTL 264)...."

## Usage

```bash

dnstest [OPTIONS] --record-type TYPE NAME

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

```bash
K8S_NS="kube-system"
K8S_POD="dnstest"
kubectl run  -n "${K8S_NS:-kube-system}" -it --rm ${K8S_POD:-dnstest} --image=docker.io/diepes/dnstest:latest -- -i 1  -r 100.96.0.10 -s 30,100,150 -- microsoft.com
```

Monitor with

```bash
kubectl logs -n kube-system dnstest -f
```

Remove k8s pod with

```bash
    kubectl delete pod -n kube-system dnstest
```
