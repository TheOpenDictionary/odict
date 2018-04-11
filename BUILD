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
        "@boost//:filesystem",
        "@boost//:lexical_cast",
        "@boost//:property_tree",
        "@boost//:uuid",
        "@boost//:system",
        "@clownfish//:main",
        "@flatbuffers//:main",
        "@rapidxml//:main",
        "@snappy//:main",
    ]
)