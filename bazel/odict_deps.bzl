load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")

def odict_deps():
    RULES_JVM_EXTERNAL_TAG = "4.2"
    RULES_JVM_EXTERNAL_SHA = "cd1a77b7b02e8e008439ca76fd34f5b07aecb8c752961f9640dea15e9e5ba1ca"
    RULES_GO = "0.31.0"
    RULES_GAZELLE = "0.24.0"

    maybe(
        http_archive,
        name = "bazel_gazelle",
        sha256 = "de69a09dc70417580aabf20a28619bb3ef60d038470c7cf8442fafcf627c21cb",
        urls = [
            "https://mirror.bazel.build/github.com/bazelbuild/bazel-gazelle/releases/download/v%s/bazel-gazelle-v%s.tar.gz" % (RULES_GAZELLE, RULES_GAZELLE),
            "https://github.com/bazelbuild/bazel-gazelle/releases/download/v%s/bazel-gazelle-v%s.tar.gz" % (RULES_GAZELLE, RULES_GAZELLE),
        ],
    )

    maybe(
        http_archive,
        name = "io_bazel_rules_go",
        sha256 = "f2dcd210c7095febe54b804bb1cd3a58fe8435a909db2ec04e31542631cf715c",
        urls = [
            "https://mirror.bazel.build/github.com/bazelbuild/rules_go/releases/download/v%s/rules_go-v%s.zip" % (RULES_GO, RULES_GO),
            "https://github.com/bazelbuild/rules_go/releases/download/v%s/rules_go-v%s.zip" % (RULES_GO, RULES_GO),
        ],
    )

    maybe(
        http_archive,
        name = "rules_jvm_external",
        strip_prefix = "rules_jvm_external-%s" % RULES_JVM_EXTERNAL_TAG,
        sha256 = RULES_JVM_EXTERNAL_SHA,
        url = "https://github.com/bazelbuild/rules_jvm_external/archive/%s.zip" % RULES_JVM_EXTERNAL_TAG,
    )

    maybe(
        http_archive,
        name = "com_google_protobuf",
        sha256 = "9748c0d90e54ea09e5e75fb7fac16edce15d2028d4356f32211cfa3c0e956564",
        strip_prefix = "protobuf-3.11.4",
        urls = ["https://github.com/protocolbuffers/protobuf/archive/v3.11.4.zip"],
    )

    maybe(
        http_archive,
        name = "native_utils",
        sha256 = "8387afc2d09fb8ee35aff74d2b755632679fe3127caa946ec1263dc23ec68079",
        strip_prefix = "native-utils-master",
        urls = ["https://github.com/TheOpenDictionary/native-utils/archive/master.tar.gz"],
        build_file = "@odict//third_party:native_utils.bazel",
    )

    maybe(
        http_archive,
        name = "com_github_google_flatbuffers",
        sha256 = "9ddb9031798f4f8754d00fca2f1a68ecf9d0f83dfac7239af1311e4fd9a565c4",
        strip_prefix = "flatbuffers-2.0.0",
        urls = ["https://github.com/google/flatbuffers/archive/v2.0.0.tar.gz"],
        patch_args = ["-p1"],
        patches = ["@odict//bazel:flatbuffers_1_12_0.patch"],
    )
