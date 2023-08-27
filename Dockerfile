# https://github.com/GoogleContainerTools/distroless/tree/main
#
FROM rust:1.71.1 as build-env
WORKDIR /app
COPY . /app
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/dnstest /
ENTRYPOINT ["/dnstest"]
CMD ["google.com"]

## 2023-08-27 errors when container launch
# /dnstest: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.33' not found (required by /dnstest)
# FIX: rust:latest to rust:1.71.1