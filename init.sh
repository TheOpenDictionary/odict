#!/usr/bin/env bash

# You have to update these version numbers manually in CMakeLists.txt
SNAPPY_VERSION=1.1.7
FLATBUFFERS_VERSION=1.8.0
RAPIDXML_VERSION=1.13
LUCY_VERSION=0.6.1
CLOWNFISH_VERSION=0.6.2
BOOST_VERSION=1.65.1

function dependencies() {
    echo "1. 📁  Creating library directory..."
    mkdir -p lib && cd lib

    echo "2. 📝  Downloading Flatbuffers..."
    curl -s -L https://github.com/google/flatbuffers/archive/v${FLATBUFFERS_VERSION}.tar.gz | tar -zx

    echo "3. 💨  Downloading RapidXML..."
    curl -s -L https://downloads.sourceforge.net/project/rapidxml/rapidxml/rapidxml%201.13/rapidxml-${RAPIDXML_VERSION}.zip -o rapidxml.zip && unzip -o rapidxml.zip && rm -rf rapidxml.zip

    echo "4. 🚀  Downloading and building Boost (this may take awhile)..."
    curl -s -L https://dl.bintray.com/boostorg/release/${BOOST_VERSION}/source/boost_${BOOST_VERSION//\./_}.tar.gz | tar -zx
    cd ${BOOST_VERSION//\./_} && ./bootstrap.sh --prefix=./ && ./b2 install && cd ..

    echo "5. 👌  Downloading and building Snappy..."
    curl -s -L https://github.com/google/snappy/archive/${SNAPPY_VERSION}.tar.gz | tar -zx
    cd snappy-${SNAPPY_VERSION} && mkdir -p build && cd build && cmake ../ && make && cd ../../

    echo "6. 🐠  Downloading and building Clownfish..."
    curl -s -L http://mirrors.ocf.berkeley.edu/apache/lucy/clownfish/apache-clownfish-${CLOWNFISH_VERSION}.tar.gz | tar -zx
    cd apache-clownfish-${CLOWNFISH_VERSION}/compiler/c && ./configure && make && make test && cd ../../../
    cd apache-clownfish-${CLOWNFISH_VERSION}/runtime/c && ./configure && make && make test && cd ../../../
    source apache-clownfish-${CLOWNFISH_VERSION}/devel/bin/setup_env.sh

    echo "7. 👩  Downloading and building Lucy..."
    curl -s -L http://mirrors.ocf.berkeley.edu/apache/lucy/apache-lucy-${LUCY_VERSION}.tar.gz | tar -zx
    cd apache-lucy-${LUCY_VERSION}/c && ./configure && make && make test && cd ../

    cd ../../

    printf "\n✨  All done!\n"
}

function build() {
    mkdir -p build/bin && cd build && cmake .. && make
}

function schema() {
    flatc -c -o src src/schema.fbs
}

function clean() {
    rm -rf build
}

if [ "$#" -eq "0" ]
    then
        dependencies
        schema
        clean
        build
    else
        $1
fi
