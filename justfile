# ---------------------------------------------------------------------------- #
#                                    Global                                    #
# ---------------------------------------------------------------------------- #

@deps:
  mise install
  corepack enable 
  corepack install -g pnpm 
  just python deps

@build +args="":
  cargo build -p cli {{args}}

# @build-all +args="": deps (cli "schema") sync
#   goreleaser build --id odict --clean {{args}}

@schema: (cli "schema") (js "schema")

@insta +args="":
  cargo insta {{args}}

@update-snaps:
  cargo insta accept

@run +args="":
  cargo run {{args}}

@test:
  cargo test
  rm -rf **/.odict
# deps xsd (go "test") (jvm "test") (python "test") (js "test") (wasm "test") clean

@clean: (python "clean") (jvm "clean") (js "clean")

# @publish +args="--auto-snapshot --clean":
#   goreleaser release {{args}}

# ------------------------------------------------------------------------------ #
#                                    Platforms                                   #
# ------------------------------------------------------------------------------ #

@rust +command:
  just lib/{{command}}

@cli +command:
	just cli/{{command}}

@jvm +command:
	just jvm/{{command}}

@js +command:
	just js/{{command}}

@node +command:
	just node/{{command}}

@python +command:
	just python/{{command}}

@wasm +command:
	just wasm/{{command}}
