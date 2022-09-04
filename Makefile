
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
	go test ./go
	
# ---------------------------------------------------------------------------- #
#                                      CLI                                     #
# ---------------------------------------------------------------------------- #

cli-build:
	go build -o ./bin/odict ./odict.go

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
