
test: go-test jvm-test python-test

# ---------------------------------------------------------------------------- #
#                                    Schema                                    #
# ---------------------------------------------------------------------------- #

schema:
	flatc -g --gen-onefile --go-namespace odict -o ./go schema.fbs

# ---------------------------------------------------------------------------- #
#                                    Library                                   #
# ---------------------------------------------------------------------------- #

go-test:
	. ./versions.txt && go test -ldflags "-X 'github.com/TheOpenDictionary/odict/cli.version=$$CLI' -X 'github.com/TheOpenDictionary/odict/go.version=$$FILE'" ./go
	
# ---------------------------------------------------------------------------- #
#                                      CLI                                     #
# ---------------------------------------------------------------------------- #

cli-build:
	. ./versions.txt && go build -ldflags "-X 'github.com/TheOpenDictionary/odict/cli.version=$$CLI' -X 'github.com/TheOpenDictionary/odict/go.version=$$FILE'" -o ./bin/odict ./odict.go

# ---------------------------------------------------------------------------- #
#                                    Python                                    #
# ---------------------------------------------------------------------------- #

jvm-test: cli-build
	cd jvm && RUNTIME_ENV=test ./gradlew test

# ---------------------------------------------------------------------------- #
#                                    Python                                    #
# ---------------------------------------------------------------------------- #

python-install:
	cd python && poetry install 

python-test: cli-build python-install
	cd python && RUNTIME_ENV=test poetry run pytest ./odict

clean:
	rm -rf setup.py dist build schema/*.go