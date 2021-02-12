load("@bazel_gazelle//:deps.bzl", "gazelle_dependencies", "go_repository")
load("@io_bazel_rules_go//go:deps.bzl", "go_register_toolchains", "go_rules_dependencies")
load("@com_google_protobuf//:protobuf_deps.bzl", "protobuf_deps")
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")
load("@rules_jvm_external//:defs.bzl", "maven_install")

def odict_extra_deps():
    go_rules_dependencies()

    go_register_toolchains(version = "1.15.6")

    gazelle_dependencies()

    protobuf_deps()

    maybe(
        go_repository,
        name = "com_github_armon_consul_api",
        importpath = "github.com/armon/consul-api",
        sum = "h1:G1bPvciwNyF7IUmKXNt9Ak3m6u9DE1rF+RmtIkBpVdA=",
        version = "v0.0.0-20180202201655-eb2c6b5be1b6",
    )

    maybe(
        go_repository,
        name = "com_github_blevesearch_bleve_index_api",
        importpath = "github.com/blevesearch/bleve_index_api",
        sum = "h1:Ds3XeuTxjXCkG6pgIwWDRyooJKNIuOKemnN0N0IkhTU=",
        version = "v1.0.0",
    )

    maybe(
        go_repository,
        name = "com_github_blevesearch_bleve_v2",
        importpath = "github.com/blevesearch/bleve/v2",
        build_file_proto_mode = "disable",
        sum = "h1:v1eV5K+/lndsjnykeVcuU9J4cJnjKLUKSwxXFxZsLuY=",
        version = "v2.0.1",
    )

    maybe(
        go_repository,
        name = "com_github_blevesearch_go_porterstemmer",
        importpath = "github.com/blevesearch/go-porterstemmer",
        sum = "h1:GtmsqID0aZdCSNiY8SkuPJ12pD4jI+DdXTAn4YRcHCo=",
        version = "v1.0.3",
    )

    maybe(
        go_repository,
        name = "com_github_blevesearch_mmap_go",
        importpath = "github.com/blevesearch/mmap-go",
        sum = "h1:JtMHb+FgQCTTYIhtMvimw15dJwu1Y5lrZDMOFXVWPk0=",
        version = "v1.0.2",
    )

    maybe(
        go_repository,
        name = "com_github_blevesearch_scorch_segment_api",
        importpath = "github.com/blevesearch/scorch_segment_api",
        sum = "h1:BUkCPWDg2gimTEyVDXf85I2buqqt4lh28uaVMiJsIYk=",
        version = "v1.0.0",
    )

    maybe(
        go_repository,
        name = "com_github_blevesearch_segment",
        importpath = "github.com/blevesearch/segment",
        sum = "h1:5lG7yBCx98or7gK2cHMKPukPZ/31Kag7nONpoBt22Ac=",
        version = "v0.9.0",
    )

    maybe(
        go_repository,
        name = "com_github_blevesearch_snowballstem",
        importpath = "github.com/blevesearch/snowballstem",
        sum = "h1:lMQ189YspGP6sXvZQ4WZ+MLawfV8wOmPoD/iWeNXm8s=",
        version = "v0.9.0",
    )

    maybe(
        go_repository,
        name = "com_github_blevesearch_upsidedown_store_api",
        importpath = "github.com/blevesearch/upsidedown_store_api",
        sum = "h1:1SYRwyoFLwG3sj0ed89RLtM15amfX2pXlYbFOnF8zNU=",
        version = "v1.0.1",
    )

    maybe(
        go_repository,
        name = "com_github_blevesearch_zapx_v11",
        importpath = "github.com/blevesearch/zapx/v11",
        sum = "h1:8Eo3rXiHsVSP9Sk+4StrrwLrj9vyulhMVPmxTf8ZuDg=",
        version = "v11.1.10",
    )

    maybe(
        go_repository,
        name = "com_github_blevesearch_zapx_v12",
        importpath = "github.com/blevesearch/zapx/v12",
        sum = "h1:sqR+/0Z4dSTovApRqLA1HnilMtQer7a4UvPrNmPzlTM=",
        version = "v12.1.10",
    )

    maybe(
        go_repository,
        name = "com_github_blevesearch_zapx_v13",
        importpath = "github.com/blevesearch/zapx/v13",
        sum = "h1:zCneEVRJDXwtDfSwh+33Dxguliv192vCK283zdGH4Sw=",
        version = "v13.1.10",
    )

    maybe(
        go_repository,
        name = "com_github_blevesearch_zapx_v14",
        importpath = "github.com/blevesearch/zapx/v14",
        sum = "h1:nD0vw2jxKogJFfA5WyoS4wNwZlVby3Aq8aW7CZi6YIw=",
        version = "v14.1.10",
    )

    maybe(
        go_repository,
        name = "com_github_blevesearch_zapx_v15",
        importpath = "github.com/blevesearch/zapx/v15",
        sum = "h1:kZR3b9jO9l6s2B5UHI+1N1llLzJ4nYikkXQTMrDl1vQ=",
        version = "v15.1.10",
    )

    maybe(
        go_repository,
        name = "com_github_burntsushi_toml",
        importpath = "github.com/BurntSushi/toml",
        sum = "h1:WXkYYl6Yr3qBf1K79EBnL4mak0OimBfB0XUf9Vl28OQ=",
        version = "v0.3.1",
    )

    maybe(
        go_repository,
        name = "com_github_coreos_etcd",
        importpath = "github.com/coreos/etcd",
        sum = "h1:jFneRYjIvLMLhDLCzuTuU4rSJUjRplcJQ7pD7MnhC04=",
        version = "v3.3.10+incompatible",
    )

    maybe(
        go_repository,
        name = "com_github_coreos_go_etcd",
        importpath = "github.com/coreos/go-etcd",
        sum = "h1:bXhRBIXoTm9BYHS3gE0TtQuyNZyeEMux2sDi4oo5YOo=",
        version = "v2.0.0+incompatible",
    )

    maybe(
        go_repository,
        name = "com_github_coreos_go_semver",
        importpath = "github.com/coreos/go-semver",
        sum = "h1:3Jm3tLmsgAYcjC+4Up7hJrFBPr+n7rAqYeSw/SZazuY=",
        version = "v0.2.0",
    )

    maybe(
        go_repository,
        name = "com_github_couchbase_ghistogram",
        importpath = "github.com/couchbase/ghistogram",
        sum = "h1:b95QcQTCzjTUocDXp/uMgSNQi8oj1tGwnJ4bODWZnps=",
        version = "v0.1.0",
    )

    maybe(
        go_repository,
        name = "com_github_couchbase_moss",
        importpath = "github.com/couchbase/moss",
        sum = "h1:HCL+xxHUwmOaL44kMM/gU08OW6QGCui1WVFO58bjhNI=",
        version = "v0.1.0",
    )

    maybe(
        go_repository,
        name = "com_github_couchbase_vellum",
        importpath = "github.com/couchbase/vellum",
        sum = "h1:BrbP0NKiyDdndMPec8Jjhy0U47CZ0Lgx3xUC2r9rZqw=",
        version = "v1.0.2",
    )

    maybe(
        go_repository,
        name = "com_github_cpuguy83_go_md2man",
        importpath = "github.com/cpuguy83/go-md2man",
        sum = "h1:BSKMNlYxDvnunlTymqtgONjNnaRV1sTpcovwwjF22jk=",
        version = "v1.0.10",
    )
    maybe(
        go_repository,
        name = "com_github_cpuguy83_go_md2man_v2",
        importpath = "github.com/cpuguy83/go-md2man/v2",
        sum = "h1:U+s90UTSYgptZMwQh2aRr3LuazLJIa+Pg3Kc1ylSYVY=",
        version = "v2.0.0-20190314233015-f79a8a8ca69d",
    )

    maybe(
        go_repository,
        name = "com_github_davecgh_go_spew",
        importpath = "github.com/davecgh/go-spew",
        sum = "h1:vj9j/u1bqnvCEfJOwUhtlOARqs3+rkHYY13jYWTU97c=",
        version = "v1.1.1",
    )

    maybe(
        go_repository,
        name = "com_github_fsnotify_fsnotify",
        importpath = "github.com/fsnotify/fsnotify",
        sum = "h1:IXs+QLmnXW2CcXuY+8Mzv/fWEsPGWxqefPtCP5CnV9I=",
        version = "v1.4.7",
    )

    maybe(
        go_repository,
        name = "com_github_glycerine_go_unsnap_stream",
        importpath = "github.com/glycerine/go-unsnap-stream",
        sum = "h1:Ujru1hufTHVb++eG6OuNDKMxZnGIvF6o/u8q/8h2+I4=",
        version = "v0.0.0-20181221182339-f9677308dec2",
    )

    maybe(
        go_repository,
        name = "com_github_glycerine_goconvey",
        importpath = "github.com/glycerine/goconvey",
        sum = "h1:gclg6gY70GLy3PbkQ1AERPfmLMMagS60DKF78eWwLn8=",
        version = "v0.0.0-20190410193231-58a59202ab31",
    )

    maybe(
        go_repository,
        name = "com_github_golang_protobuf",
        importpath = "github.com/golang/protobuf",
        sum = "h1:6nsPYzhq5kReh6QImI3k5qWzO4PEbvbIW2cwSfR/6xs=",
        version = "v1.3.2",
    )

    maybe(
        go_repository,
        name = "com_github_golang_snappy",
        importpath = "github.com/golang/snappy",
        sum = "h1:Qgr9rKW7uDUkrbSmQeiDsGa8SjGyCOGtuasMWwvp2P4=",
        version = "v0.0.1",
    )

    maybe(
        go_repository,
        name = "com_github_google_uuid",
        importpath = "github.com/google/uuid",
        sum = "h1:qJYtXnJRWmpe7m/3XlyhrsLrEURqHRM2kxzoxXqyUDs=",
        version = "v1.2.0",
    )

    maybe(
        go_repository,
        name = "com_github_gopherjs_gopherjs",
        importpath = "github.com/gopherjs/gopherjs",
        sum = "h1:twflg0XRTjwKpxb/jFExr4HGq6on2dEOmnL6FV+fgPw=",
        version = "v0.0.0-20190910122728-9d188e94fb99",
    )

    maybe(
        go_repository,
        name = "com_github_hashicorp_hcl",
        importpath = "github.com/hashicorp/hcl",
        sum = "h1:0Anlzjpi4vEasTeNFn2mLJgTSwt0+6sfsiTG8qcWGx4=",
        version = "v1.0.0",
    )

    maybe(
        go_repository,
        name = "com_github_hpcloud_tail",
        importpath = "github.com/hpcloud/tail",
        sum = "h1:nfCOvKYfkgYP8hkirhJocXT2+zOD8yUNjXaWfTlyFKI=",
        version = "v1.0.0",
    )

    maybe(
        go_repository,
        name = "com_github_imdario_mergo",
        importpath = "github.com/imdario/mergo",
        sum = "h1:UauaLniWCFHWd+Jp9oCEkTBj8VO/9DKg3PV3VCNMDIg=",
        version = "v0.3.9",
    )

    maybe(
        go_repository,
        name = "com_github_inconshreveable_mousetrap",
        importpath = "github.com/inconshreveable/mousetrap",
        sum = "h1:Z8tu5sraLXCXIcARxBp/8cbvlwVa7Z1NHg9XEKhtSvM=",
        version = "v1.0.0",
    )

    maybe(
        go_repository,
        name = "com_github_jtolds_gls",
        importpath = "github.com/jtolds/gls",
        sum = "h1:xdiiI2gbIgH/gLH7ADydsJ1uDOEzR8yvV7C0MuV77Wo=",
        version = "v4.20.0+incompatible",
    )

    maybe(
        go_repository,
        name = "com_github_k0kubun_go_ansi",
        importpath = "github.com/k0kubun/go-ansi",
        sum = "h1:qGQQKEcAR99REcMpsXCp3lJ03zYT1PkRd3kQGPn9GVg=",
        version = "v0.0.0-20180517002512-3bf9e2903213",
    )

    maybe(
        go_repository,
        name = "com_github_kljensen_snowball",
        importpath = "github.com/kljensen/snowball",
        sum = "h1:6DZLCcZeL0cLfodx+Md4/OLC6b/bfurWUOUGs1ydfOU=",
        version = "v0.6.0",
    )

    maybe(
        go_repository,
        name = "com_github_magiconair_properties",
        importpath = "github.com/magiconair/properties",
        sum = "h1:LLgXmsheXeRoUOBOjtwPQCWIYqM/LU1ayDtDePerRcY=",
        version = "v1.8.0",
    )

    maybe(
        go_repository,
        name = "com_github_mattn_go_isatty",
        importpath = "github.com/mattn/go-isatty",
        sum = "h1:wuysRhFDzyxgEmMf5xjvJ2M9dZoWAXNNr5LSBS7uHXY=",
        version = "v0.0.12",
    )

    maybe(
        go_repository,
        name = "com_github_mattn_go_runewidth",
        importpath = "github.com/mattn/go-runewidth",
        sum = "h1:CoZ3S2P7pvtP45xOtBw+/mDL2z0RKI576gSkzRRpdGg=",
        version = "v0.0.10",
    )

    maybe(
        go_repository,
        name = "com_github_mitchellh_colorstring",
        importpath = "github.com/mitchellh/colorstring",
        sum = "h1:62I3jR2EmQ4l5rM/4FEfDWcRD+abF5XlKShorW5LRoQ=",
        version = "v0.0.0-20190213212951-d06e56a500db",
    )

    maybe(
        go_repository,
        name = "com_github_mitchellh_go_homedir",
        importpath = "github.com/mitchellh/go-homedir",
        sum = "h1:lukF9ziXFxDFPkA1vsr5zpc1XuPDn/wFntq5mG+4E0Y=",
        version = "v1.1.0",
    )

    maybe(
        go_repository,
        name = "com_github_mitchellh_mapstructure",
        importpath = "github.com/mitchellh/mapstructure",
        sum = "h1:fmNYVwqnSfB9mZU6OS2O6GsXM+wcskZDuKQzvN1EDeE=",
        version = "v1.1.2",
    )

    maybe(
        go_repository,
        name = "com_github_mschoch_smat",
        importpath = "github.com/mschoch/smat",
        sum = "h1:8imxQsjDm8yFEAVBe7azKmKSgzSkZXDuKkSq9374khM=",
        version = "v0.2.0",
    )

    maybe(
        go_repository,
        name = "com_github_onsi_ginkgo",
        importpath = "github.com/onsi/ginkgo",
        sum = "h1:WSHQ+IS43OoUrWtD1/bbclrwK8TTH5hzp+umCiuxHgs=",
        version = "v1.7.0",
    )

    maybe(
        go_repository,
        name = "com_github_onsi_gomega",
        importpath = "github.com/onsi/gomega",
        sum = "h1:RE1xgDvH7imwFD45h+u2SgIfERHlS2yNG4DObb5BSKU=",
        version = "v1.4.3",
    )

    maybe(
        go_repository,
        name = "com_github_pelletier_go_toml",
        importpath = "github.com/pelletier/go-toml",
        sum = "h1:T5zMGML61Wp+FlcbWjRDT7yAxhJNAiPPLOFECq181zc=",
        version = "v1.2.0",
    )

    maybe(
        go_repository,
        name = "com_github_philhofer_fwd",
        importpath = "github.com/philhofer/fwd",
        sum = "h1:UbZqGr5Y38ApvM/V/jEljVxwocdweyH+vmYvRPBnbqQ=",
        version = "v1.0.0",
    )

    maybe(
        go_repository,
        name = "com_github_pmezard_go_difflib",
        importpath = "github.com/pmezard/go-difflib",
        sum = "h1:4DBwDE0NGyQoBHbLQYPwSUPoCMWR5BEzIk/f1lZbAQM=",
        version = "v1.0.0",
    )

    maybe(
        go_repository,
        name = "com_github_rcrowley_go_metrics",
        importpath = "github.com/rcrowley/go-metrics",
        sum = "h1:dY6ETXrvDG7Sa4vE8ZQG4yqWg6UnOcbqTAahkV813vQ=",
        version = "v0.0.0-20190826022208-cac0b30c2563",
    )

    maybe(
        go_repository,
        name = "com_github_rivo_uniseg",
        importpath = "github.com/rivo/uniseg",
        sum = "h1:S1pD9weZBuJdFmowNwbpi7BJ8TNftyUImj/0WQi72jY=",
        version = "v0.2.0",
    )

    maybe(
        go_repository,
        name = "com_github_roaringbitmap_roaring",
        importpath = "github.com/RoaringBitmap/roaring",
        sum = "h1:gpyfd12QohbqhFO4NVDUdoPOCXsyahYRQhINmlHxKeo=",
        version = "v0.4.23",
    )

    maybe(
        go_repository,
        name = "com_github_russross_blackfriday",
        importpath = "github.com/russross/blackfriday",
        sum = "h1:HyvC0ARfnZBqnXwABFeSZHpKvJHJJfPz81GNueLj0oo=",
        version = "v1.5.2",
    )
    maybe(
        go_repository,
        name = "com_github_russross_blackfriday_v2",
        importpath = "github.com/russross/blackfriday/v2",
        sum = "h1:lPqVAte+HuHNfhJ/0LC98ESWRz8afy9tM/0RK8m9o+Q=",
        version = "v2.0.1",
    )

    maybe(
        go_repository,
        name = "com_github_schollz_progressbar_v3",
        importpath = "github.com/schollz/progressbar/v3",
        sum = "h1:G2HfclnGJR2HtTOmFkERQcRqo9J20asOFiuD6AnI5EQ=",
        version = "v3.7.4",
    )

    maybe(
        go_repository,
        name = "com_github_shurcool_sanitized_anchor_name",
        importpath = "github.com/shurcooL/sanitized_anchor_name",
        sum = "h1:PdmoCO6wvbs+7yrJyMORt4/BmY5IYyJwS/kOiWx8mHo=",
        version = "v1.0.0",
    )

    maybe(
        go_repository,
        name = "com_github_spf13_afero",
        importpath = "github.com/spf13/afero",
        sum = "h1:m8/z1t7/fwjysjQRYbP0RD+bUIF/8tJwPdEZsI83ACI=",
        version = "v1.1.2",
    )

    maybe(
        go_repository,
        name = "com_github_spf13_cast",
        importpath = "github.com/spf13/cast",
        sum = "h1:oget//CVOEoFewqQxwr0Ej5yjygnqGkvggSE/gB35Q8=",
        version = "v1.3.0",
    )

    maybe(
        go_repository,
        name = "com_github_spf13_cobra",
        importpath = "github.com/spf13/cobra",
        sum = "h1:f0B+LkLX6DtmRH1isoNA9VTtNUK9K8xYd28JNNfOv/s=",
        version = "v0.0.5",
    )

    maybe(
        go_repository,
        name = "com_github_spf13_jwalterweatherman",
        importpath = "github.com/spf13/jwalterweatherman",
        sum = "h1:XHEdyB+EcvlqZamSM4ZOMGlc93t6AcsBEu9Gc1vn7yk=",
        version = "v1.0.0",
    )

    maybe(
        go_repository,
        name = "com_github_spf13_pflag",
        importpath = "github.com/spf13/pflag",
        sum = "h1:zPAT6CGy6wXeQ7NtTnaTerfKOsV6V6F8agHXFiazDkg=",
        version = "v1.0.3",
    )

    maybe(
        go_repository,
        name = "com_github_spf13_viper",
        importpath = "github.com/spf13/viper",
        sum = "h1:VUFqw5KcqRf7i70GOzW7N+Q7+gxVBkSSqiXB12+JQ4M=",
        version = "v1.3.2",
    )

    maybe(
        go_repository,
        name = "com_github_steveyen_gtreap",
        importpath = "github.com/steveyen/gtreap",
        sum = "h1:CjhzTa274PyJLJuMZwIzCO1PfC00oRa8d1Kc78bFXJM=",
        version = "v0.1.0",
    )

    maybe(
        go_repository,
        name = "com_github_stretchr_objx",
        importpath = "github.com/stretchr/objx",
        sum = "h1:4G4v2dO3VZwixGIRoQ5Lfboy6nUhCyYzaqnIAPPhYs4=",
        version = "v0.1.0",
    )

    maybe(
        go_repository,
        name = "com_github_stretchr_testify",
        importpath = "github.com/stretchr/testify",
        sum = "h1:2E4SXV/wtOkTonXsotYi4li6zVWxYlZuYNCXe9XRJyk=",
        version = "v1.4.0",
    )

    maybe(
        go_repository,
        name = "com_github_syndtr_goleveldb",
        importpath = "github.com/syndtr/goleveldb",
        sum = "h1:fBdIW9lB4Iz0n9khmH8w27SJ3QEJ7+IgjPEwGSZiFdE=",
        version = "v1.0.0",
    )

    maybe(
        go_repository,
        name = "com_github_tinylib_msgp",
        importpath = "github.com/tinylib/msgp",
        sum = "h1:9fQd+ICuRIu/ue4vxJZu6/LzxN0HwMds2nq/0cFvxHU=",
        version = "v1.1.0",
    )

    maybe(
        go_repository,
        name = "com_github_ugorji_go_codec",
        importpath = "github.com/ugorji/go/codec",
        sum = "h1:3SVOIvH7Ae1KRYyQWRjXWJEA9sS/c/pjvH++55Gr648=",
        version = "v0.0.0-20181204163529-d75b2dcb6bc8",
    )
    maybe(
        go_repository,
        name = "com_github_urfave_cli_v2",
        importpath = "github.com/urfave/cli/v2",
        sum = "h1:qph92Y649prgesehzOrQjdWyxFOp/QVM+6imKHad91M=",
        version = "v2.3.0",
    )

    maybe(
        go_repository,
        name = "com_github_willf_bitset",
        importpath = "github.com/willf/bitset",
        sum = "h1:NotGKqX0KwQ72NUzqrjZq5ipPNDQex9lo3WpaS8L2sc=",
        version = "v1.1.10",
    )

    maybe(
        go_repository,
        name = "com_github_xordataexchange_crypt",
        importpath = "github.com/xordataexchange/crypt",
        sum = "h1:ESFSdwYZvkeru3RtdrYueztKhOBCSAAzS4Gf+k0tEow=",
        version = "v0.0.3-0.20170626215501-b2862e3d0a77",
    )

    maybe(
        go_repository,
        name = "in_gopkg_check_v1",
        importpath = "gopkg.in/check.v1",
        sum = "h1:yhCVgyC4o1eVCa2tZl7eS0r+SDo693bJlVdllGtEeKM=",
        version = "v0.0.0-20161208181325-20d25e280405",
    )

    maybe(
        go_repository,
        name = "in_gopkg_fsnotify_v1",
        importpath = "gopkg.in/fsnotify.v1",
        sum = "h1:xOHLXZwVvI9hhs+cLKq5+I5onOuwQLhQwiu63xxlHs4=",
        version = "v1.4.7",
    )

    maybe(
        go_repository,
        name = "in_gopkg_tomb_v1",
        importpath = "gopkg.in/tomb.v1",
        sum = "h1:uRGJdciOHaEIrze2W8Q3AKkepLTh2hOroT7a+7czfdQ=",
        version = "v1.0.0-20141024135613-dd632973f1e7",
    )

    maybe(
        go_repository,
        name = "in_gopkg_yaml_v2",
        importpath = "gopkg.in/yaml.v2",
        sum = "h1:clyUAQHOM3G0M3f5vQj7LuJrETvjVot3Z5el9nffUtU=",
        version = "v2.3.0",
    )

    maybe(
        go_repository,
        name = "io_etcd_go_bbolt",
        importpath = "go.etcd.io/bbolt",
        sum = "h1:XAzx9gjCb0Rxj7EoqcClPD1d5ZBxZJk0jbuoPHenBt0=",
        version = "v1.3.5",
    )
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

    maybe(
        go_repository,
        name = "org_golang_x_crypto",
        importpath = "golang.org/x/crypto",
        sum = "h1:DN0cp81fZ3njFcrLCytUHRSUkqBjfTo4Tx9RJTWs0EY=",
        version = "v0.0.0-20201221181555-eec23a3978ad",
    )

    maybe(
        go_repository,
        name = "org_golang_x_net",
        importpath = "golang.org/x/net",
        sum = "h1:0GoQqolDA55aaLxZyTzK/Y2ePZzZTUrRacwib7cNsYQ=",
        version = "v0.0.0-20190404232315-eb5bcb51f2a3",
    )

    maybe(
        go_repository,
        name = "org_golang_x_sync",
        importpath = "golang.org/x/sync",
        sum = "h1:wMNYb4v58l5UBM7MYRLPG6ZhfOqbKu7X5eyFl8ZhKvA=",
        version = "v0.0.0-20180314180146-1d60e4601c6f",
    )

    maybe(
        go_repository,
        name = "org_golang_x_sys",
        importpath = "golang.org/x/sys",
        sum = "h1:VwygUrnw9jn88c4u8GD3rZQbqrP/tgas88tPUbBxQrk=",
        version = "v0.0.0-20210124154548-22da62e12c0c",
    )

    maybe(
        go_repository,
        name = "org_golang_x_term",
        importpath = "golang.org/x/term",
        sum = "h1:MZ2shdL+ZM/XzY3ZGOnh4Nlpnxz5GSOhOmtHo3iPU6M=",
        version = "v0.0.0-20201210144234-2321bbc49cbf",
    )

    maybe(
        go_repository,
        name = "org_golang_x_text",
        importpath = "golang.org/x/text",
        sum = "h1:g61tztE5qeGQ89tm6NTjjM9VPIm088od1l6aSorWRWg=",
        version = "v0.3.0",
    )
