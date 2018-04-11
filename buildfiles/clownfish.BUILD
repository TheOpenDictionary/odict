genrule(
    name = "build",
    srcs = glob(["compiler/c/**/*"]) + glob(["runtime/c/**/*"]),
    cmd = " && ".join([
        "cp -a $(SRCS) .",
        "cd compiler/c",
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