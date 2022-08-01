# gazelle:prefix https://github.com/TheOpenDictionary/odict
workspace(name = "odict")

RULES_JVM_EXTERNAL_TAG = "4.2"

RULES_JVM_EXTERNAL_SHA = "cd1a77b7b02e8e008439ca76fd34f5b07aecb8c752961f9640dea15e9e5ba1ca"

RULES_GO = "0.33.0"

RULES_GAZELLE = "0.24.0"

RULES_PLATFORMS = "0.0.5"

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "bazel_gazelle",
    sha256 = "de69a09dc70417580aabf20a28619bb3ef60d038470c7cf8442fafcf627c21cb",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-gazelle/releases/download/v%s/bazel-gazelle-v%s.tar.gz" % (RULES_GAZELLE, RULES_GAZELLE),
        "https://github.com/bazelbuild/bazel-gazelle/releases/download/v%s/bazel-gazelle-v%s.tar.gz" % (RULES_GAZELLE, RULES_GAZELLE),
    ],
)

http_archive(
    name = "platforms",
    sha256 = "379113459b0feaf6bfbb584a91874c065078aa673222846ac765f86661c27407",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/platforms/releases/download/%s/platforms-%s.tar.gz" % (RULES_PLATFORMS, RULES_PLATFORMS),
        "https://github.com/bazelbuild/platforms/releases/download/%s/platforms-%s.tar.gz" % (RULES_PLATFORMS, RULES_PLATFORMS),
    ],
)

http_archive(
    name = "io_bazel_rules_go",
    sha256 = "685052b498b6ddfe562ca7a97736741d87916fe536623afb7da2824c0211c369",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/rules_go/releases/download/v%s/rules_go-v%s.zip" % (RULES_GO, RULES_GO),
        "https://github.com/bazelbuild/rules_go/releases/download/v%s/rules_go-v%s.zip" % (RULES_GO, RULES_GO),
    ],
)

http_archive(
    name = "rules_jvm_external",
    sha256 = RULES_JVM_EXTERNAL_SHA,
    strip_prefix = "rules_jvm_external-%s" % RULES_JVM_EXTERNAL_TAG,
    url = "https://github.com/bazelbuild/rules_jvm_external/archive/%s.zip" % RULES_JVM_EXTERNAL_TAG,
)

http_archive(
    name = "com_google_protobuf",
    sha256 = "9748c0d90e54ea09e5e75fb7fac16edce15d2028d4356f32211cfa3c0e956564",
    strip_prefix = "protobuf-3.11.4",
    urls = ["https://github.com/protocolbuffers/protobuf/archive/v3.11.4.zip"],
)

http_archive(
    name = "native_utils",
    build_file = "@odict//third_party:native_utils.bazel",
    sha256 = "8387afc2d09fb8ee35aff74d2b755632679fe3127caa946ec1263dc23ec68079",
    strip_prefix = "native-utils-master",
    urls = ["https://github.com/TheOpenDictionary/native-utils/archive/master.tar.gz"],
)

http_archive(
    name = "com_github_google_flatbuffers",
    patch_args = ["-p1"],
    patches = ["@odict//bazel:flatbuffers_1_12_0.patch"],
    sha256 = "9ddb9031798f4f8754d00fca2f1a68ecf9d0f83dfac7239af1311e4fd9a565c4",
    strip_prefix = "flatbuffers-2.0.0",
    urls = ["https://github.com/google/flatbuffers/archive/v2.0.0.tar.gz"],
)

load("@rules_jvm_external//:repositories.bzl", "rules_jvm_external_deps")

rules_jvm_external_deps()

load("@rules_jvm_external//:setup.bzl", "rules_jvm_external_setup")

rules_jvm_external_setup()

load("@io_bazel_rules_go//go:deps.bzl", "go_register_toolchains", "go_rules_dependencies")

go_rules_dependencies()

go_register_toolchains(version = "1.18")

load("@rules_jvm_external//:defs.bzl", "maven_install")

maven_install(
    name = "odict_java_deps",
    artifacts = [
        "org.xerial.snappy:snappy-java:1.1.8.4",
        "com.fasterxml.jackson.core:jackson-core:2.12.1",
        "com.fasterxml.jackson.core:jackson-annotations:2.12.1",
        "com.fasterxml.jackson.core:jackson-databind:2.12.1",
    ],
    repositories = [
        "https://jcenter.bintray.com/",
        "https://maven.google.com",
        "https://repo1.maven.org/maven2",
    ],
)

load("@bazel_gazelle//:deps.bzl", "gazelle_dependencies", "go_repository")

go_repository(
    name = "com_github_armon_consul_api",
    importpath = "github.com/armon/consul-api",
    sum = "h1:G1bPvciwNyF7IUmKXNt9Ak3m6u9DE1rF+RmtIkBpVdA=",
    version = "v0.0.0-20180202201655-eb2c6b5be1b6",
)

go_repository(
    name = "com_github_bits_and_blooms_bitset",
    importpath = "github.com/bits-and-blooms/bitset",
    sum = "h1:h7mv5q31cthBTd7V4kLAZaIThj1e8vPGcSqpPue9KVI=",
    version = "v1.3.0",
)

go_repository(
    name = "com_github_blevesearch_bleve_index_api",
    importpath = "github.com/blevesearch/bleve_index_api",
    sum = "h1:DDSWaPXOZZJ2BB73ZTWjKxydAugjwywcqU+91AAqcAg=",
    version = "v1.0.3",
)

go_repository(
    name = "com_github_blevesearch_bleve_v2",
    importpath = "github.com/blevesearch/bleve/v2",
    sum = "h1:aDvFXy9TxdVLcu/YmHAiLQ2VsCO7hcAYDCgm0SYDEVE=",
    version = "v2.3.3",
)

go_repository(
    name = "com_github_blevesearch_geo",
    importpath = "github.com/blevesearch/geo",
    sum = "h1:5uNzC0Mn/8aCGbSJA6T8ZCjrKW8MKsZKQYBDowmeV/g=",
    version = "v0.1.12-0.20220606102651-aab42add3121",
)

go_repository(
    name = "com_github_blevesearch_go_metrics",
    importpath = "github.com/blevesearch/go-metrics",
    sum = "h1:kDy+zgJFJJoJYBvdfBSiZYBbdsUL0XcjHYWezpQBGPA=",
    version = "v0.0.0-20201227073835-cf1acfcdf475",
)

go_repository(
    name = "com_github_blevesearch_go_porterstemmer",
    importpath = "github.com/blevesearch/go-porterstemmer",
    sum = "h1:GtmsqID0aZdCSNiY8SkuPJ12pD4jI+DdXTAn4YRcHCo=",
    version = "v1.0.3",
)

go_repository(
    name = "com_github_blevesearch_goleveldb",
    importpath = "github.com/blevesearch/goleveldb",
    sum = "h1:iAtV2Cu5s0GD1lwUiekkFHe2gTMCCNVj2foPclDLIFI=",
    version = "v1.0.1",
)

go_repository(
    name = "com_github_blevesearch_gtreap",
    importpath = "github.com/blevesearch/gtreap",
    sum = "h1:2JWigFrzDMR+42WGIN/V2p0cUvn4UP3C4Q5nmaZGW8Y=",
    version = "v0.1.1",
)

go_repository(
    name = "com_github_blevesearch_mmap_go",
    importpath = "github.com/blevesearch/mmap-go",
    sum = "h1:OVhDhT5B/M1HNPpYPBKIEJaD0F3Si+CrEKULGCDPWmc=",
    version = "v1.0.4",
)

go_repository(
    name = "com_github_blevesearch_scorch_segment_api_v2",
    importpath = "github.com/blevesearch/scorch_segment_api/v2",
    sum = "h1:TAte9VZLWda5WAVlZTTZ+GCzEHqGJb4iB2aiZSA6Iv8=",
    version = "v2.1.2",
)

go_repository(
    name = "com_github_blevesearch_segment",
    importpath = "github.com/blevesearch/segment",
    sum = "h1:5lG7yBCx98or7gK2cHMKPukPZ/31Kag7nONpoBt22Ac=",
    version = "v0.9.0",
)

go_repository(
    name = "com_github_blevesearch_snowball",
    importpath = "github.com/blevesearch/snowball",
    sum = "h1:cDYjn/NCH+wwt2UdehaLpr2e4BwLIjN4V/TdLsL+B5A=",
    version = "v0.6.1",
)

go_repository(
    name = "com_github_blevesearch_snowballstem",
    importpath = "github.com/blevesearch/snowballstem",
    sum = "h1:lMQ189YspGP6sXvZQ4WZ+MLawfV8wOmPoD/iWeNXm8s=",
    version = "v0.9.0",
)

go_repository(
    name = "com_github_blevesearch_upsidedown_store_api",
    importpath = "github.com/blevesearch/upsidedown_store_api",
    sum = "h1:1SYRwyoFLwG3sj0ed89RLtM15amfX2pXlYbFOnF8zNU=",
    version = "v1.0.1",
)

go_repository(
    name = "com_github_blevesearch_vellum",
    importpath = "github.com/blevesearch/vellum",
    sum = "h1:iMGh4lfxza4BnWO/UJTMPlI3HsK9YawjPv+TteVa9ck=",
    version = "v1.0.8",
)

go_repository(
    name = "com_github_blevesearch_zapx_v11",
    importpath = "github.com/blevesearch/zapx/v11",
    sum = "h1:eBQWQ7huA+mzm0sAGnZDwgGGli7S45EO+N+ObFWssbI=",
    version = "v11.3.5",
)

go_repository(
    name = "com_github_blevesearch_zapx_v12",
    importpath = "github.com/blevesearch/zapx/v12",
    sum = "h1:5pX2hU+R1aZihT7ac1dNWh1n4wqkIM9pZzWp0ANED9s=",
    version = "v12.3.5",
)

go_repository(
    name = "com_github_blevesearch_zapx_v13",
    importpath = "github.com/blevesearch/zapx/v13",
    sum = "h1:eJ3gbD+Nu8p36/O6lhfdvWQ4pxsGYSuTOBrLLPVWJ74=",
    version = "v13.3.5",
)

go_repository(
    name = "com_github_blevesearch_zapx_v14",
    importpath = "github.com/blevesearch/zapx/v14",
    sum = "h1:hEvVjZaagFCvOUJrlFQ6/Z6Jjy0opM3g7TMEo58TwP4=",
    version = "v14.3.5",
)

go_repository(
    name = "com_github_blevesearch_zapx_v15",
    importpath = "github.com/blevesearch/zapx/v15",
    sum = "h1:/y6AOxRuBiZPFAItqcrKcXPPtlAwuW/jMoOFO7tc7rs=",
    version = "v15.3.4",
)

go_repository(
    name = "com_github_burntsushi_toml",
    importpath = "github.com/BurntSushi/toml",
    sum = "h1:ksErzDEI1khOiGPgpwuI7x2ebx/uXQNw7xJpn9Eq1+I=",
    version = "v1.1.0",
)

go_repository(
    name = "com_github_coreos_etcd",
    importpath = "github.com/coreos/etcd",
    sum = "h1:jFneRYjIvLMLhDLCzuTuU4rSJUjRplcJQ7pD7MnhC04=",
    version = "v3.3.10+incompatible",
)

go_repository(
    name = "com_github_coreos_go_etcd",
    importpath = "github.com/coreos/go-etcd",
    sum = "h1:bXhRBIXoTm9BYHS3gE0TtQuyNZyeEMux2sDi4oo5YOo=",
    version = "v2.0.0+incompatible",
)

go_repository(
    name = "com_github_coreos_go_semver",
    importpath = "github.com/coreos/go-semver",
    sum = "h1:3Jm3tLmsgAYcjC+4Up7hJrFBPr+n7rAqYeSw/SZazuY=",
    version = "v0.2.0",
)

go_repository(
    name = "com_github_couchbase_ghistogram",
    importpath = "github.com/couchbase/ghistogram",
    sum = "h1:b95QcQTCzjTUocDXp/uMgSNQi8oj1tGwnJ4bODWZnps=",
    version = "v0.1.0",
)

go_repository(
    name = "com_github_couchbase_moss",
    importpath = "github.com/couchbase/moss",
    sum = "h1:VCYrMzFwEryyhRSeI+/b3tRBSeTpi/8gn5Kf6dxqn+o=",
    version = "v0.2.0",
)

go_repository(
    name = "com_github_cpuguy83_go_md2man",
    importpath = "github.com/cpuguy83/go-md2man",
    sum = "h1:BSKMNlYxDvnunlTymqtgONjNnaRV1sTpcovwwjF22jk=",
    version = "v1.0.10",
)

go_repository(
    name = "com_github_cpuguy83_go_md2man_v2",
    importpath = "github.com/cpuguy83/go-md2man/v2",
    sum = "h1:p1EgwI/C7NhT0JmVkwCD2ZBK8j4aeHQX2pMHHBfMQ6w=",
    version = "v2.0.2",
)

go_repository(
    name = "com_github_davecgh_go_spew",
    importpath = "github.com/davecgh/go-spew",
    sum = "h1:vj9j/u1bqnvCEfJOwUhtlOARqs3+rkHYY13jYWTU97c=",
    version = "v1.1.1",
)

go_repository(
    name = "com_github_fsnotify_fsnotify",
    importpath = "github.com/fsnotify/fsnotify",
    sum = "h1:IXs+QLmnXW2CcXuY+8Mzv/fWEsPGWxqefPtCP5CnV9I=",
    version = "v1.4.7",
)

go_repository(
    name = "com_github_golang_geo",
    importpath = "github.com/golang/geo",
    sum = "h1:gtexQ/VGyN+VVFRXSFiguSNcXmS6rkKT+X7FdIrTtfo=",
    version = "v0.0.0-20210211234256-740aa86cb551",
)

go_repository(
    name = "com_github_golang_protobuf",
    importpath = "github.com/golang/protobuf",
    sum = "h1:ROPKBNFfQgOUMifHyP+KYbvpjbdoFNs+aK7DXlji0Tw=",
    version = "v1.5.2",
)

go_repository(
    name = "com_github_golang_snappy",
    importpath = "github.com/golang/snappy",
    sum = "h1:yAGX7huGHXlcLOEtBnF4w7FQwA26wojNCwOYAEhLjQM=",
    version = "v0.0.4",
)

go_repository(
    name = "com_github_google_go_cmp",
    importpath = "github.com/google/go-cmp",
    sum = "h1:Khx7svrCpmxxtHBq5j2mp/xVjsi8hQMfNLvJFAlrGgU=",
    version = "v0.5.5",
)

go_repository(
    name = "com_github_google_gofuzz",
    importpath = "github.com/google/gofuzz",
    sum = "h1:xRy4A+RhZaiKjJ1bPfwQ8sedCA+YS2YcCHW6ec7JMi0=",
    version = "v1.2.0",
)

go_repository(
    name = "com_github_google_uuid",
    importpath = "github.com/google/uuid",
    sum = "h1:t6JiXgmwXMjEs8VusXIJk2BXHsn+wx8BZdTaoZ5fu7I=",
    version = "v1.3.0",
)

go_repository(
    name = "com_github_hashicorp_hcl",
    importpath = "github.com/hashicorp/hcl",
    sum = "h1:0Anlzjpi4vEasTeNFn2mLJgTSwt0+6sfsiTG8qcWGx4=",
    version = "v1.0.0",
)

go_repository(
    name = "com_github_hpcloud_tail",
    importpath = "github.com/hpcloud/tail",
    sum = "h1:nfCOvKYfkgYP8hkirhJocXT2+zOD8yUNjXaWfTlyFKI=",
    version = "v1.0.0",
)

go_repository(
    name = "com_github_imdario_mergo",
    importpath = "github.com/imdario/mergo",
    sum = "h1:lFzP57bqS/wsqKssCGmtLAb8A0wKjLGrve2q3PPVcBk=",
    version = "v0.3.13",
)

go_repository(
    name = "com_github_inconshreveable_mousetrap",
    importpath = "github.com/inconshreveable/mousetrap",
    sum = "h1:Z8tu5sraLXCXIcARxBp/8cbvlwVa7Z1NHg9XEKhtSvM=",
    version = "v1.0.0",
)

go_repository(
    name = "com_github_json_iterator_go",
    importpath = "github.com/json-iterator/go",
    sum = "h1:YrgBGwxMRK0Vq0WSCWFaZUnTsrA/PZE/xs1QZh+/edg=",
    version = "v0.0.0-20171115153421-f7279a603ede",
)

go_repository(
    name = "com_github_k0kubun_go_ansi",
    importpath = "github.com/k0kubun/go-ansi",
    sum = "h1:qGQQKEcAR99REcMpsXCp3lJ03zYT1PkRd3kQGPn9GVg=",
    version = "v0.0.0-20180517002512-3bf9e2903213",
)

go_repository(
    name = "com_github_magiconair_properties",
    importpath = "github.com/magiconair/properties",
    sum = "h1:LLgXmsheXeRoUOBOjtwPQCWIYqM/LU1ayDtDePerRcY=",
    version = "v1.8.0",
)

go_repository(
    name = "com_github_mattn_go_isatty",
    importpath = "github.com/mattn/go-isatty",
    sum = "h1:yVuAays6BHfxijgZPzw+3Zlu5yQgKGP2/hcQbHb7S9Y=",
    version = "v0.0.14",
)

go_repository(
    name = "com_github_mattn_go_runewidth",
    importpath = "github.com/mattn/go-runewidth",
    sum = "h1:lTGmDsbAYt5DmK6OnoV7EuIF1wEIFAcxld6ypU4OSgU=",
    version = "v0.0.13",
)

go_repository(
    name = "com_github_mitchellh_colorstring",
    importpath = "github.com/mitchellh/colorstring",
    sum = "h1:62I3jR2EmQ4l5rM/4FEfDWcRD+abF5XlKShorW5LRoQ=",
    version = "v0.0.0-20190213212951-d06e56a500db",
)

go_repository(
    name = "com_github_mitchellh_go_homedir",
    importpath = "github.com/mitchellh/go-homedir",
    sum = "h1:lukF9ziXFxDFPkA1vsr5zpc1XuPDn/wFntq5mG+4E0Y=",
    version = "v1.1.0",
)

go_repository(
    name = "com_github_mitchellh_mapstructure",
    importpath = "github.com/mitchellh/mapstructure",
    sum = "h1:fmNYVwqnSfB9mZU6OS2O6GsXM+wcskZDuKQzvN1EDeE=",
    version = "v1.1.2",
)

go_repository(
    name = "com_github_mschoch_smat",
    importpath = "github.com/mschoch/smat",
    sum = "h1:8imxQsjDm8yFEAVBe7azKmKSgzSkZXDuKkSq9374khM=",
    version = "v0.2.0",
)

go_repository(
    name = "com_github_onsi_ginkgo",
    importpath = "github.com/onsi/ginkgo",
    sum = "h1:WSHQ+IS43OoUrWtD1/bbclrwK8TTH5hzp+umCiuxHgs=",
    version = "v1.7.0",
)

go_repository(
    name = "com_github_onsi_gomega",
    importpath = "github.com/onsi/gomega",
    sum = "h1:RE1xgDvH7imwFD45h+u2SgIfERHlS2yNG4DObb5BSKU=",
    version = "v1.4.3",
)

go_repository(
    name = "com_github_pelletier_go_toml",
    importpath = "github.com/pelletier/go-toml",
    sum = "h1:T5zMGML61Wp+FlcbWjRDT7yAxhJNAiPPLOFECq181zc=",
    version = "v1.2.0",
)

go_repository(
    name = "com_github_pmezard_go_difflib",
    importpath = "github.com/pmezard/go-difflib",
    sum = "h1:4DBwDE0NGyQoBHbLQYPwSUPoCMWR5BEzIk/f1lZbAQM=",
    version = "v1.0.0",
)

go_repository(
    name = "com_github_rivo_uniseg",
    importpath = "github.com/rivo/uniseg",
    sum = "h1:SDPP7SHNl1L7KrEFCSJslJ/DM9DT02Nq2C61XrfHMmk=",
    version = "v0.3.1",
)

go_repository(
    name = "com_github_roaringbitmap_roaring",
    importpath = "github.com/RoaringBitmap/roaring",
    sum = "h1:58/LJlg/81wfEHd5L9qsHduznOIhyv4qb1yWcSvVq9A=",
    version = "v1.2.1",
)

go_repository(
    name = "com_github_russross_blackfriday",
    importpath = "github.com/russross/blackfriday",
    sum = "h1:HyvC0ARfnZBqnXwABFeSZHpKvJHJJfPz81GNueLj0oo=",
    version = "v1.5.2",
)

go_repository(
    name = "com_github_russross_blackfriday_v2",
    importpath = "github.com/russross/blackfriday/v2",
    sum = "h1:JIOH55/0cWyOuilr9/qlrm0BSXldqnqwMsf35Ld67mk=",
    version = "v2.1.0",
)

go_repository(
    name = "com_github_schollz_progressbar_v3",
    importpath = "github.com/schollz/progressbar/v3",
    sum = "h1:k9SRNQ8KZyibz1UZOaKxnkUE3iGtmGSDt1YY9KlCYQk=",
    version = "v3.9.0",
)

go_repository(
    name = "com_github_spf13_afero",
    importpath = "github.com/spf13/afero",
    sum = "h1:m8/z1t7/fwjysjQRYbP0RD+bUIF/8tJwPdEZsI83ACI=",
    version = "v1.1.2",
)

go_repository(
    name = "com_github_spf13_cast",
    importpath = "github.com/spf13/cast",
    sum = "h1:oget//CVOEoFewqQxwr0Ej5yjygnqGkvggSE/gB35Q8=",
    version = "v1.3.0",
)

go_repository(
    name = "com_github_spf13_cobra",
    importpath = "github.com/spf13/cobra",
    sum = "h1:f0B+LkLX6DtmRH1isoNA9VTtNUK9K8xYd28JNNfOv/s=",
    version = "v0.0.5",
)

go_repository(
    name = "com_github_spf13_jwalterweatherman",
    importpath = "github.com/spf13/jwalterweatherman",
    sum = "h1:XHEdyB+EcvlqZamSM4ZOMGlc93t6AcsBEu9Gc1vn7yk=",
    version = "v1.0.0",
)

go_repository(
    name = "com_github_spf13_pflag",
    importpath = "github.com/spf13/pflag",
    sum = "h1:zPAT6CGy6wXeQ7NtTnaTerfKOsV6V6F8agHXFiazDkg=",
    version = "v1.0.3",
)

go_repository(
    name = "com_github_spf13_viper",
    importpath = "github.com/spf13/viper",
    sum = "h1:VUFqw5KcqRf7i70GOzW7N+Q7+gxVBkSSqiXB12+JQ4M=",
    version = "v1.3.2",
)

go_repository(
    name = "com_github_stretchr_objx",
    importpath = "github.com/stretchr/objx",
    sum = "h1:M2gUjqZET1qApGOWNSnZ49BAIMX4F/1plDv3+l31EJ4=",
    version = "v0.4.0",
)

go_repository(
    name = "com_github_stretchr_testify",
    importpath = "github.com/stretchr/testify",
    sum = "h1:pSgiaMZlXftHpm5L7V1+rVB+AZJydKsMxsQBIJw4PKk=",
    version = "v1.8.0",
)

go_repository(
    name = "com_github_ugorji_go_codec",
    importpath = "github.com/ugorji/go/codec",
    sum = "h1:3SVOIvH7Ae1KRYyQWRjXWJEA9sS/c/pjvH++55Gr648=",
    version = "v0.0.0-20181204163529-d75b2dcb6bc8",
)

go_repository(
    name = "com_github_urfave_cli_v2",
    importpath = "github.com/urfave/cli/v2",
    sum = "h1:UKK6SP7fV3eKOefbS87iT9YHefv7iB/53ih6e+GNAsE=",
    version = "v2.11.1",
)

go_repository(
    name = "com_github_xordataexchange_crypt",
    importpath = "github.com/xordataexchange/crypt",
    sum = "h1:ESFSdwYZvkeru3RtdrYueztKhOBCSAAzS4Gf+k0tEow=",
    version = "v0.0.3-0.20170626215501-b2862e3d0a77",
)

go_repository(
    name = "com_github_xrash_smetrics",
    importpath = "github.com/xrash/smetrics",
    sum = "h1:bAn7/zixMGCfxrRTfdpNzjtPYqr8smhKouy9mxVdGPU=",
    version = "v0.0.0-20201216005158-039620a65673",
)

go_repository(
    name = "in_gopkg_check_v1",
    importpath = "gopkg.in/check.v1",
    sum = "h1:yhCVgyC4o1eVCa2tZl7eS0r+SDo693bJlVdllGtEeKM=",
    version = "v0.0.0-20161208181325-20d25e280405",
)

go_repository(
    name = "in_gopkg_fsnotify_v1",
    importpath = "gopkg.in/fsnotify.v1",
    sum = "h1:xOHLXZwVvI9hhs+cLKq5+I5onOuwQLhQwiu63xxlHs4=",
    version = "v1.4.7",
)

go_repository(
    name = "in_gopkg_tomb_v1",
    importpath = "gopkg.in/tomb.v1",
    sum = "h1:uRGJdciOHaEIrze2W8Q3AKkepLTh2hOroT7a+7czfdQ=",
    version = "v1.0.0-20141024135613-dd632973f1e7",
)

go_repository(
    name = "in_gopkg_yaml_v2",
    importpath = "gopkg.in/yaml.v2",
    sum = "h1:ZCJp+EgiOT7lHqUV2J862kp8Qj64Jo6az82+3Td9dZw=",
    version = "v2.2.2",
)

go_repository(
    name = "in_gopkg_yaml_v3",
    importpath = "gopkg.in/yaml.v3",
    sum = "h1:fxVm/GzAzEWqLHuvctI91KS9hhNmmWOoWu0XTYJS7CA=",
    version = "v3.0.1",
)

go_repository(
    name = "io_etcd_go_bbolt",
    importpath = "go.etcd.io/bbolt",
    sum = "h1:/ecaJf0sk1l4l6V4awd65v2C3ILy7MSj+s/x1ADCIMU=",
    version = "v1.3.6",
)

go_repository(
    name = "org_golang_google_protobuf",
    importpath = "google.golang.org/protobuf",
    sum = "h1:bxAC2xTBsZGibn2RTntX0oH50xLsqy1OxA9tTL3p/lk=",
    version = "v1.26.0",
)

go_repository(
    name = "org_golang_x_crypto",
    importpath = "golang.org/x/crypto",
    sum = "h1:zuSxTR4o9y82ebqCUJYNGJbGPo6sKVl54f/TVDObg1c=",
    version = "v0.0.0-20220722155217-630584e8d5aa",
)

go_repository(
    name = "org_golang_x_mod",
    importpath = "golang.org/x/mod",
    sum = "h1:OJxoQ/rynoF0dcCdI7cLPktw/hR2cueqYfjm43oqK38=",
    version = "v0.5.1",
)

go_repository(
    name = "org_golang_x_net",
    importpath = "golang.org/x/net",
    sum = "h1:CIJ76btIcR3eFI5EgSo6k1qKw9KJexJuRLI9G7Hp5wE=",
    version = "v0.0.0-20211112202133-69e39bad7dc2",
)

go_repository(
    name = "org_golang_x_sync",
    importpath = "golang.org/x/sync",
    sum = "h1:wMNYb4v58l5UBM7MYRLPG6ZhfOqbKu7X5eyFl8ZhKvA=",
    version = "v0.0.0-20180314180146-1d60e4601c6f",
)

go_repository(
    name = "org_golang_x_sys",
    importpath = "golang.org/x/sys",
    sum = "h1:Sv5ogFZatcgIMMtBSTTAgMYsicp25MXBubjXNDKwm80=",
    version = "v0.0.0-20220731174439-a90be440212d",
)

go_repository(
    name = "org_golang_x_term",
    importpath = "golang.org/x/term",
    sum = "h1:Q5284mrmYTpACcm+eAKjKJH48BBwSyfJqmmGDTtT8Vc=",
    version = "v0.0.0-20220722155259-a9ba230a4035",
)

go_repository(
    name = "org_golang_x_text",
    importpath = "golang.org/x/text",
    sum = "h1:olpwvP2KacW1ZWvsR7uQhoyTYvKAupfQrRGBFM352Gk=",
    version = "v0.3.7",
)

go_repository(
    name = "org_golang_x_tools",
    importpath = "golang.org/x/tools",
    sum = "h1:aZzprAO9/8oim3qStq3wc1Xuxx4QmAGriC4VU4ojemQ=",
    version = "v0.0.0-20191119224855-298f0cb1881e",
)

go_repository(
    name = "org_golang_x_xerrors",
    importpath = "golang.org/x/xerrors",
    sum = "h1:uF6paiQQebLeSXkrTqHqz0MXhXXS1KgF41eUdBNvxK0=",
    version = "v0.0.0-20220609144429-65e65417b02f",
)

gazelle_dependencies()

# load("@com_google_protobuf//:protobuf_deps.bzl", "protobuf_deps")

# protobuf_deps()
