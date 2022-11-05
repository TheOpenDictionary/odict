
test: go-test jvm-test python-test js-test

clean:
	rm -rf setup.py dist build schema/*.go

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
#                                  JavaScript                                  #
# ---------------------------------------------------------------------------- #

js-install:
	cd js && npm install

js-build: js-install
	cd js && npm run build

js-test: cli-build js-install
	cd js && RUNTIME_ENV=test npm run test


# ---------------------------------------------------------------------------- #
#                                    Python                                    #
# ---------------------------------------------------------------------------- #

python-install:
	cd python && poetry install 

python-test: cli-build python-install
	cd python && RUNTIME_ENV=test poetry run pytest ./odict
