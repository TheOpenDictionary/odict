# ---------------------------------------------------------------------------- #
#                                    Global                                    #
# ---------------------------------------------------------------------------- #

@setup: && (node "setup")
  mise install
  corepack install
  corepack enable pnpm

@bench +args="":
  cargo bench {{args}}

build +args="":
  cargo build -p cli {{args}}

# @build-all +args="": deps (cli "schema") sync
#   goreleaser build --id odict --clean {{args}}

@insta +args="":
  cargo insta {{args}}

@update-snaps:
  cargo insta accept

@run +args="":
  cargo run {{args}}

test: && (node "test")
  cargo test
  rm -rf **/.odict
# deps xsd (go "test") (jvm "test") (python "test") (js "test") (wasm "test") clean

@clean: (python "clean") (jvm "clean")

# @publish +args="--auto-snapshot --clean":
#   goreleaser release {{args}}

# ------------------------------------------------------------------------------ #
#                                    Platforms                                   #
# ------------------------------------------------------------------------------ #

@ffi +command:
  just ffi/{{command}}

@rust +command:
  just lib/{{command}}

@cli +command:
	just cli/{{command}}

@jvm +command:
	just jvm/{{command}}

@node +command:
	just node/{{command}}

@python +command:
	just python/{{command}}

@wasm +command:
	just wasm/{{command}}
