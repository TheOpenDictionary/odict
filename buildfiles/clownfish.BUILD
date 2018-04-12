config_setting(name = "windows")
config_setting(name = "osx")
config_setting(name = "linux")

steps = [
    "cd external/clownfish/compiler/c",
    "./configure",
    "make",
    "make test",
    "cd ../../runtime/c",
    "./configure",
    "make",
    "make test",
    "cd ../../ && ls",
    "cp runtime/c/libclownfish.0.6.0.dylib ../../$(@D)/",
    "cp runtime/c/libtestcfish.0.6.0.dylib ../../$(@D)/",
    "cp -r runtime/c/autogen/include ../../$(@D)/"
]

genrule(
    name = "build",
    srcs = ["compiler", "runtime"],
    cmd = " && ".join(steps),
    local = 1,
    outs = [
        "libclownfish.0.6.0.dylib",
        "libtestcfish.0.6.0.dylib",
        "include"
    ]
)

genrule(
    name = "headers",
    srcs = [":build"],
    cmd = "ls",
    outs = ["test.h"]
)

cc_library(
    name = "clownfish",
    srcs = [":build"],
    textual_hdrs = [":headers"],
    visibility = ["//visibility:public"]
)