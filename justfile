mod binding "./bindings/justfile"

# ---------------------------------------------------------------------------- #
#                                    Global                                    #
# ---------------------------------------------------------------------------- #

@setup:
  mise install
  just binding node setup 
  just binding python setup

@bench +args="":
  cargo bench {{args}}

build +args="":
  cargo build -p cli {{args}}

@insta +args="":
  cargo insta {{args}}

@update-snaps:
  cargo insta accept

@run +args="":
  cargo run {{args}}

test:
  cargo test
  rm -rf **/.odict 
  just binding node test 

@clean: 
  just binding python clean 
  just binding java clean

@uniffi +args="":
  cargo run -p uniffi-bindgen {{args}}