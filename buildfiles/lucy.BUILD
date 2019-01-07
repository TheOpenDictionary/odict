

config_setting(name = "windows")
config_setting(name = "osx")
config_setting(name = "linux")

steps = [
    # "ls $(SRCS)"
    "cd external/clownfish/compiler/c",
    "./configure",
    "make",
    "make test",
    "mkdir -p bin",
    "cp cfc ./bin/cfc",
    "cd ../../runtime/c",
    "./configure",
    "make",
    "make test",
    "cd ../../devel",
    "source bin/setup_env.sh ../",
    # "ls external/clownfish/compiler",
    "pwd",
    "cd ../../lucy/c",
    "./configure --clownfish-prefix=../../clownfish/compiler/c",
    "make",
    "make test"
    # "cd $OUT/get-lucy/lucy-rel-v0.6.1/c",
    # "./configure",
    # "make",
    # "make test"
]

genrule(
    name = "build",
    srcs = ["@clownfish//:source", "."],
    cmd = " && ".join(steps),
    local = 1,
    outs = [
        "libclownfish.0.6.1.dylib",
        "libtestcfish.0.6.1.dylib",
        "include"
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
    name = "lucy",
    srcs = [":build"],
    textual_hdrs = [":headers"],
    visibility = ["//visibility:public"]
)