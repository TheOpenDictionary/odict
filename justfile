
test: (go "test") (jvm "test") (python "test") (js "test")

clean:
	rm -rf setup.py dist build schema/*.go

# ---------------------------------------------------------------------------- #
#                                    Schema                                    #
# ---------------------------------------------------------------------------- #

schema:
	flatc -g --gen-onefile --go-namespace odict -o ./go schema.fbs

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
<<<<<<< HEAD

wasm +command:
	just wasm/{{command}}
=======
>>>>>>> main
