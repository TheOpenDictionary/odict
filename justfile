# ---------------------------------------------------------------------------- #
#                                    Global                                    #
# ---------------------------------------------------------------------------- #

@deps: 
  asdf install > /dev/null
  go install golang.org/x/tools/cmd/goimports@latest

@build: deps (cli "schema") sync
  goreleaser build --id single --clean --snapshot --single-target

@build-all +args="": deps (cli "schema") sync
  goreleaser build --id odict --clean {{args}}

@schema: (go "schema") (cli "schema") (js "schema")

@run +args="": build
  ./bin/odict {{args}}
  
@test: deps (go "test") (jvm "test") (python "test") (js "test") (wasm "test") clean

@clean: (python "clean") (jvm "clean") (js "clean")

@publish +args="--auto-snapshot --clean":
  goreleaser release {{args}}

@sync:
  go work sync 

# ------------------------------------------------------------------------------ #
#                                    Platforms                                   #
# ------------------------------------------------------------------------------ #

@go +command:
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
