ARG GORELEASER_CROSS_VERSION 0.21.3 

FROM ghcr.io/goreleaser/goreleaser-cross:${GOLANG_CROSS_VERSION} AS goreleaser-cross

RUN apt-get update -y && apt-get upgrade -y 
RUN apt-get install -y pkg-config libxml2-dev libicu-dev gcc-multilib