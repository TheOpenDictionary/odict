# gazelle:repo bazel_gazelle

workspace(name = "odict")

load("//bazel:odict_deps.bzl", "odict_deps")
odict_deps()

load("//bazel:odict_extra_deps.bzl", "odict_extra_deps")
# gazelle:repository_macro bazel/odict_extra_deps.bzl%odict_extra_deps
odict_extra_deps()

load("@rules_jvm_external//:repositories.bzl", "rules_jvm_external_deps")
rules_jvm_external_deps()

load("@rules_jvm_external//:setup.bzl", "rules_jvm_external_setup")
rules_jvm_external_setup()

load("//bazel:maven_deps.bzl", "maven_deps")
maven_deps()