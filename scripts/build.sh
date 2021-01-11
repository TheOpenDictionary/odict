mkdir -p bin
go build -o bin/odict
go build -o bin/odict.so -buildmode=c-shared