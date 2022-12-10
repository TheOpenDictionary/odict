
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
	go test -ldflags "-X 'github.com/TheOpenDictionary/odict/cli.version=$(cat version.txt)'" ./go
	
# ---------------------------------------------------------------------------- #
#                                      CLI                                     #
# ---------------------------------------------------------------------------- #

cli-build:  
	go build -ldflags "-X 'github.com/TheOpenDictionary/odict/cli.version=$(cat version.txt)'" -o ./bin/odict ./odict.go

# ---------------------------------------------------------------------------- #
#                                     Java                                     #
# ---------------------------------------------------------------------------- #

jvm-test: cli-build
	cd jvm && RUNTIME_ENV=test ./gradlew test --info

# ---------------------------------------------------------------------------- #
#                                  JavaScript                                  #
# ---------------------------------------------------------------------------- #

js-install:
	cd js && npm install

js-build: js-install
	cd js && npm run build

js-test: cli-build js-install
	cd js && RUNTIME_ENV=test npm run test

js-publish: js-install
	cd js && npm publish

# ---------------------------------------------------------------------------- #
#                                    Python                                    #
# ---------------------------------------------------------------------------- #

python-install: python-update
	cd python && poetry install 

python-test: cli-build python-install
	cd python && RUNTIME_ENV=test poetry run pytest ./theopendictionary

python-update:
	cd python && poetry update 

python-build: python-install
	cd python && poetry build

python-publish: python-build
	cd python && poetry publish