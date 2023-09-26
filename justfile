# ---------------------------------------------------------------------------- #
#                                    Global                                    #
# ---------------------------------------------------------------------------- #

@setup: 
  asdf install > /dev/null
  go install golang.org/x/tools/cmd/goimports@latest

@build: (cli "build")

@run +args="": (cli "run" args)

@test: (go "test") (jvm "test") (python "test") (js "test") clean

@clean: (python "clean") (jvm "clean") (js "clean")
  rm -rf **/*.odict 

@schema: (go "schema") (cli "schema") (js "schema")

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
