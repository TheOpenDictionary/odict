FROM jdxcode/mise:debian AS builder

SHELL ["/bin/bash", "-c"]

RUN apt-get update \
    && apt-get install -y --no-install-recommends build-essential cmake \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /odict
COPY . .

RUN mise trust -y
RUN mise --no-hooks install rust
RUN mise run --skip-tools build -- --release

FROM debian:latest

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /odict/target/release/odict /usr/local/bin/odict

ENTRYPOINT ["odict", "serve"]
CMD []
