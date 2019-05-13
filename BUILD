alias(name = "it", actual = ":odict")

cc_library(
    name = "odict_lib",
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
        # "@clownfish//:clownfish",
        # "@lucy//:lucy"
    ]
)

cc_binary(
    name = "odict",
    deps = [
        ":odict_lib"
    ]
)