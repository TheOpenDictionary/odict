#########################
#     Google Snappy     #
#########################
new_http_archive(
    name = "snappy",
    url = "https://github.com/google/snappy/archive/1.1.7.zip",
    strip_prefix = "snappy-1.1.7",
    build_file = "buildfiles/snappy.BUILD"
)

##############################
#     Google Flatbuffers     #
##############################
new_http_archive(
    name = "flatbuffers",
    url = "https://github.com/google/flatbuffers/archive/v1.8.0.zip",
    strip_prefix = "flatbuffers-1.8.0",
    build_file = "buildfiles/flatbuffers.BUILD"
)

############################
#     Apache Clownfish     #
############################
new_http_archive(
    name = "clownfish",
    url = "https://github.com/odict/lucy-clownfish/archive/rel/v0.6.2.zip",
    strip_prefix = "lucy-clownfish-rel-v0.6.2",
    build_file = "buildfiles/clownfish.BUILD"
)

#################
#     Boost     #
#################
git_repository(
    name = "com_github_nelhage_rules_boost",
    commit = "239ce40e42ab0e3fe7ce84c2e9303ff8a277c41a",
    remote = "https://github.com/nelhage/rules_boost",
)
load("@com_github_nelhage_rules_boost//:boost/boost.bzl", "boost_deps")
boost_deps()

####################
#     RapidXML     #
####################
new_http_archive(
    name = "rapidxml",
    url = "https://github.com/odict/rapidxml/archive/v1.13.zip",
    strip_prefix = "rapidxml-1.13",
    build_file = "buildfiles/rapidxml.BUILD"
)