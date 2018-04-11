genrule(
    name = "build",
    srcs = glob(["compiler/c/**/*"]) +
           glob(["compiler/common/**/*"]) +
           glob(["compiler/include/**/*"]) +
           glob(["compiler/src/**/*"]) +
           glob(["runtime/c/**/*"]) +
           glob(["runtime/core/**/*"]),
    cmd = " && ".join([
        "cd external/clownfish/compiler/c && echo $$PWD",
        "./configure",
        "make",
        "make test",
        "cd ../../runtime/c",
        "./configure",
        "make",
        "make test"
    ]),
    outs = ["libclownfish.0.6.2.dylib"]
)

cc_library(
    name = "main",
    srcs = [":build"],
    visibility = ["//visibility:public"]
)