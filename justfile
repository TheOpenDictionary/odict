# ---------------------------------------------------------------------------- #
#                                    Global                                    #
# ---------------------------------------------------------------------------- #

build: (cli "build")

test: (go "test") (jvm "test") (python "test") (js "test")

clean: (python "clean") (jvm "clean") (js "clean")
  rm -rf **/*.odict 

sync:
  go work sync
  
# ---------------------------------------------------------------------------- #
#                                    Schema                                    #
# ---------------------------------------------------------------------------- #
schema: (go "schema")

# ------------------------------------------------------------------------------ #
#                                    Platforms                                   #
# ------------------------------------------------------------------------------ #

go +command:
	just lib/{{command}}

cli +command:
	just cli/{{command}}

jvm +command:
	just jvm/{{command}}

js +command:
	just js/{{command}}

python +command:
	just python/{{command}}

wasm +command:
	just wasm/{{command}}
