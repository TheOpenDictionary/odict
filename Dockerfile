ARG GORELEASER_CROSS_VERSION=v1.21.3

FROM goreleaser/goreleaser-cross:$GORELEASER_CROSS_VERSION AS goreleaser-cross

RUN apt-get update -y && apt-get upgrade -y 
RUN apt-get install -y pkg-config libicu-dev libpython3-dev gcc-multilib libc6-dev
RUN curl -L ftp://xmlsoft.org/libxml2/LATEST_LIBXML2 -o ./LIBXML2_LATEST.tar.gz && \
  tar -xf ./LIBXML2_LATEST.tar.gz && \
  cd ./libxml2* && \
  ./configure --prefix=/usr  --enable-static --with-threads --with-history PYTHON=/usr/bin/python3 && \
  make && \
  make install