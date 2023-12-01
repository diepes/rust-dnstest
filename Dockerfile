# https://github.com/GoogleContainerTools/distroless/tree/main
#
FROM rust:1.74.0 as build-env
WORKDIR /app
COPY . /app
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN cargo build --release 

# 2023-12-02 mv base from google gcr.io/distroless/cc to chainguard 27.6MB to 16.8MB final build.
#            rust 1.71.1 > 1.74.0 16.8MB to 17.1MB
# Use ChainGuard's glibc-dynamic image as the base image. More information at https://www.chainguard.dev/chainguard-images
FROM cgr.dev/chainguard/glibc-dynamic
COPY --from=build-env /app/target/release/dnstest /
ENTRYPOINT ["/dnstest"]
CMD ["google.com"]

## 2023-08-27 errors when container launch
# /dnstest: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.33' not found (required by /dnstest)
# FIX: rust:latest to rust:1.71.1