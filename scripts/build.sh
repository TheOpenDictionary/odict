mkdir -p bin
cd cli
go build -o ../bin/odict -v
go build -o ../bin/odict.so -buildmode=c-shared -v