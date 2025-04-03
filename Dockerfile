ARG ODICT_VERSION=2.1.0

FROM debian:slim

ARG ODICT_VERSION
ENV ODICT_VERSION=${ODICT_VERSION}

RUN curl -fsSL https://github.com/TheOpenDictionary/odict/releases/download/cli%2F${ODICT_VERSION}/odict-cli-installer.sh | sh

WORKDIR /dictionaries
VOLUME /dictionaries

ENTRYPOINT ["odict", "serve"]
CMD []
