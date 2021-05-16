load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")

def odict_deps():
    RULES_JVM_EXTERNAL_TAG = "4.0"
    RULES_JVM_EXTERNAL_SHA = "31701ad93dbfe544d597dbe62c9a1fdd76d81d8a9150c2bf1ecf928ecdf97169"
    RULES_GO = "0.25.1"

    maybe(
        http_archive,
        name = "bazel_gazelle",
        sha256 = "62ca106be173579c0a167deb23358fdfe71ffa1e4cfdddf5582af26520f1c66f",
        urls = [
            "https://mirror.bazel.build/github.com/bazelbuild/bazel-gazelle/releases/download/v0.23.0/bazel-gazelle-v0.23.0.tar.gz",
            "https://github.com/bazelbuild/bazel-gazelle/releases/download/v0.23.0/bazel-gazelle-v0.23.0.tar.gz",
        ],
    )

    maybe(
        http_archive,
        name = "io_bazel_rules_go",
        sha256 = "7904dbecbaffd068651916dce77ff3437679f9d20e1a7956bff43826e7645fcc",
        urls = [
            "https://mirror.bazel.build/github.com/bazelbuild/rules_go/releases/download/v%s/rules_go-v%s.tar.gz" % (RULES_GO, RULES_GO),
            "https://github.com/bazelbuild/rules_go/releases/download/v%s/rules_go-v%s.tar.gz" % (RULES_GO, RULES_GO),
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
        sha256 = "62f2223fb9181d1d6338451375628975775f7522185266cd5296571ac152bc45",
        strip_prefix = "flatbuffers-1.12.0",
        urls = ["https://github.com/google/flatbuffers/archive/v1.12.0.tar.gz"],
        patch_args = ["-p1"],
        patches = ["@odict//bazel:flatbuffers_1_12_0.patch"],
    )
