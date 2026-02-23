ARG ODICT_VERSION=3.2.2

ARG ODICT_VERSION

FROM jdxcode/mise AS builder

SHELL ["/bin/bash", "-c"]

WORKDIR /odict
COPY . .

RUN mise trust -y
RUN mise install rust
RUN mise run build --release 

FROM debian:latest

COPY --from=builder /odict/target/release/odict /usr/local/bin/odict

ENTRYPOINT ["odict", "serve"]
CMD []