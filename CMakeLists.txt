cmake_minimum_required(VERSION 3.7)
project(odict)

set(CMAKE_CXX_STANDARD 14)

set(EXECUTABLE_OUTPUT_PATH "${PROJECT_SOURCE_DIR}/build/bin")
set(LIBRARY_OUTPUT_PATH "${PROJECT_SOURCE_DIR}/build/bin")
set(LIB_VERSION "1.0")

add_compile_options(-std=c++0x)

include_directories(
        ${PROJECT_SOURCE_DIR}/lib/rapidxml-1.13
        ${PROJECT_SOURCE_DIR}/lib/snappy-1.1.7
        ${PROJECT_SOURCE_DIR}/lib/snappy-1.1.7/build
        ${PROJECT_SOURCE_DIR}/lib/flatbuffers-1.7.1/include
        ${PROJECT_SOURCE_DIR}/src
)

file(GLOB sources "src/*.cpp")
file(GLOB libraries "lib/snappy-1.1.7/build/*.dylib" "lib/snappy-1.1.7/build/*.so" "lib/snappy-1.1.7/build/*.a" "lib/snappy-1.1.7/build/*.dll")

add_library(odict-${LIB_VERSION} STATIC ${sources})
add_executable(odict ${sources})

target_link_libraries(odict ${libraries})