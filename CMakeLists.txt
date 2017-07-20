cmake_minimum_required(VERSION 3.7)
project(odict)

set(CMAKE_CXX_STANDARD 14)

set(EXECUTABLE_OUTPUT_PATH "${PROJECT_SOURCE_DIR}/build/bin")
set(LIBRARY_OUTPUT_PATH "${PROJECT_SOURCE_DIR}/build/bin")
set(LIB_VERSION "0.0.1")

add_compile_options(-std=c++0x)

include_directories(
        ${PROJECT_SOURCE_DIR}/lib/rapidxml
        ${PROJECT_SOURCE_DIR}/lib/snappy/include
        ${PROJECT_SOURCE_DIR}/src
)

file(GLOB sources "src/*.cpp")
file(GLOB libraries "lib/snappy/*.dylib")

add_library(-odict-${LIB_VERSION} STATIC ${sources})
add_executable(odict ${sources})

target_link_libraries(odict ${libraries})