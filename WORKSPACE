load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

#########################
#     Google Snappy     #
#########################
http_archive(
    name = "snappy",
    urls = ["https://github.com/google/snappy/archive/1.1.7.zip"],
    strip_prefix = "snappy-1.1.7",
    build_file = "buildfiles/snappy.BUILD"
)

##############################
#     Google Flatbuffers     #
##############################
http_archive(
    name = "flatbuffers",
    urls = ["https://github.com/google/flatbuffers/archive/v1.8.0.zip"],
    strip_prefix = "flatbuffers-1.8.0",
    build_file = "buildfiles/flatbuffers.BUILD"
)

############################
#     Apache Clownfish     #
############################
http_archive(
    name = "clownfish",
    urls = ["https://github.com/apache/lucy-clownfish/archive/rel/v0.6.2.zip"],
    strip_prefix = "lucy-clownfish-rel-v0.6.2",
    build_file = "buildfiles/clownfish.BUILD"
)

#######################
#     Apache Lucy     #
#######################
http_archive(
    name = "lucy",
    urls = ["https://github.com/odict/lucy/archive/rel/v0.6.1.zip"],
    strip_prefix = "lucy-rel-v0.6.1",
    build_file = "buildfiles/lucy.BUILD"
)

#################
#     Boost     #
#################
git_repository(
    name = "com_github_nelhage_rules_boost",
    commit = "6681419da0163d097d4e09c0756c0d8b785d2aa8",
    remote = "https://github.com/nelhage/rules_boost",
)
load("@com_github_nelhage_rules_boost//:boost/boost.bzl", "boost_deps")
boost_deps()

####################
#     RapidXML     #
####################
http_archive(
    name = "rapidxml",
    urls = ["https://github.com/odict/rapidxml/archive/v1.13.zip"],
    strip_prefix = "rapidxml-1.13",
    build_file = "buildfiles/rapidxml.BUILD"
)