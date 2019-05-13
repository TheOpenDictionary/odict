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
    "cd ../../",
    "cp ./runtime/c/libclownfish.0.6.2.dylib ../../$(@D)/",
    "cp ./runtime/c/libtestcfish.0.6.2.dylib ../../$(@D)/",
    "cp -r runtime/c/autogen/include ../../$(@D)/"
]

filegroup(
    name = "source", 
    srcs = glob(["**/*"]), 
    visibility = ["//visibility:public"]
)

genrule(
    name = "build",
    srcs = ["runtime"],
    cmd = " && ".join(steps),
    local = 1,
    outs = [
        "libclownfish.0.6.2.dylib",
        "libtestcfish.0.6.2.dylib",
        "include",
    ]
)

genrule(
    name = "headers",
    srcs = [":build"],
    cmd = "cp -r $$(dirname $$(echo \"$(SRCS)\" | cut -d \" \" -f1))/include/. $(@D)",
    local = 1,
    outs = [
        "cfish_parcel.h",
        "cfish_platform.h",
        "cfish_hostdefs.h",
        "Clownfish"
    ]
)

cc_library(
    name = "clownfish",
    srcs = [":build"],
    textual_hdrs = [":headers"],
    visibility = ["//visibility:public"]
)