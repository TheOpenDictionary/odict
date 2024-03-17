GOLANG_CROSS_VERSION := "v1.21.3"

# ---------------------------------------------------------------------------- #
#                                    Global                                    #
# ---------------------------------------------------------------------------- #

@deps:
  asdf install > /dev/null

@build +args="": deps
  cargo build {{args}}

# @build-all +args="": deps (cli "schema") sync
#   goreleaser build --id odict --clean {{args}}

@schema: (go "schema") (cli "schema") (js "schema")

@insta +args="":
  cargo insta {{args}}

@update-snaps:
  cargo insta accept

@xsd:
  go run xsd/xsd.go

@run +args="":
  cargo run {{args}}

@test:
  cargo test
  rm -rf **/.odict
# deps xsd (go "test") (jvm "test") (python "test") (js "test") (wasm "test") clean

@clean: (python "clean") (jvm "clean") (js "clean")

# @publish +args="--auto-snapshot --clean":
#   goreleaser release {{args}}

@sync:
  go work sync

# ------------------------------------------------------------------------------ #
#                                    Platforms                                   #
# ------------------------------------------------------------------------------ #

@go +command:
	just go/lib/{{command}}

@rust +command:
  just lib/{{command}}

@cli +command:
	just cli/{{command}}

@jvm +command:
	just jvm/{{command}}

@js +command:
	just js/{{command}}

@python +command:
	just python/{{command}}

@wasm +command:
	just wasm/{{command}}
