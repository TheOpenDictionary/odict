GOLANG_CROSS_VERSION := "v1.21.3"

# ---------------------------------------------------------------------------- #
#                                    Global                                    #
# ---------------------------------------------------------------------------- #

@deps: xsd 
  asdf install > /dev/null
  go install golang.org/x/tools/cmd/goimports@latest

@build: deps (cli "schema") sync
  go build -o bin/odict odict.go

@build-all +args="": deps (cli "schema") sync
  goreleaser build --id odict --clean {{args}}

@schema: (go "schema") (cli "schema") (js "schema")

@xsd:
  go run xsd/xsd.go

@run +args="":
  go run odict.go {{args}}
  
@test: deps xsd (go "test") (jvm "test") (python "test") (js "test") (wasm "test") clean

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
