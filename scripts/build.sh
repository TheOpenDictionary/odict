mkdir -p bin
go build -o bin/odict odict.go
go build -o bin/odict.so -buildmode=c-shared odict.go