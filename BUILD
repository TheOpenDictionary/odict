alias(name = "it", actual = ":odict")

cc_library(
    name = "odict",
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
        "@flatbuffers//:main",
        "@rapidxml//:main",
        "@snappy//:main",
        "@clownfish//:clownfish"
    ]
)