# gazelle:repo bazel_gazelle

workspace(name = "odict")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("//bazel:odict_deps.bzl", "odict_deps")

odict_deps()

load("//bazel:odict_extra_deps.bzl", "odict_extra_deps")

# gazelle:repository_macro bazel/odict_extra_deps.bzl%odict_extra_deps
odict_extra_deps()

load("@rules_jvm_external//:repositories.bzl", "rules_jvm_external_deps")

rules_jvm_external_deps()

load("@rules_jvm_external//:setup.bzl", "rules_jvm_external_setup")

rules_jvm_external_setup()

# Maven deployment-specific dependencies
RULES_JVM_EXTERNAL_TAG = "4.2"
RULES_JVM_EXTERNAL_SHA = "cd1a77b7b02e8e008439ca76fd34f5b07aecb8c752961f9640dea15e9e5ba1ca"

http_archive(
    name = "rules_jvm_external",
    strip_prefix = "rules_jvm_external-%s" % RULES_JVM_EXTERNAL_TAG,
    sha256 = RULES_JVM_EXTERNAL_SHA,
    url = "https://github.com/bazelbuild/rules_jvm_external/archive/%s.zip" % RULES_JVM_EXTERNAL_TAG,
)