cc_library(
    name = "libodict",
    # include_prefix = "odict",
    hdrs = glob([
        "src/*.h",
        "src/**/*.h"
    ]),
    srcs = glob([
        "src/**/*.cpp",
    ]),
    deps = [
        "@snappy//:main",
        "@flatbuffers//:main"
    ]
)