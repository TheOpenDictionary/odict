package odict

import (
	"encoding/binary"
	"fmt"
	"github.com/odict/odict/schema"
	"os"

	"github.com/golang/snappy"
)

// ReadDictionary can go fuck itself
func ReadDictionary(inputPath string) {
	// Read input file
	file, err := os.Open(inputPath)

	Check(err)

	defer file.Close()

	// Read file signature as bytes
	sigBytes := make([]byte, 5)
	_, sigErr := file.Read(sigBytes)

	Check(sigErr)

	// Read ODict version as bytes
	file.Seek(5, 0)

	versionBytes := make([]byte, 2)
	_, versionError := file.Read(versionBytes)

	Check(versionError)

	// Read the compressed content size in bytes
	file.Seek(7, 0)

	contentSizeBytes := make([]byte, 4)
	_, contentSizeError := file.Read(contentSizeBytes)

	Check(contentSizeError)
	file.Seek(11, 0)

	// Decode bytes for signature, version, and contentSize
	signature := string(sigBytes)
	version := binary.LittleEndian.Uint16(versionBytes)
	contentSize := binary.LittleEndian.Uint32(contentSizeBytes)

	// Assert signature
	Assert(signature == "ODICT", "Invalid file signature")

	// Read compressed buffer content as bytes
	contentBytes := make([]byte, contentSize)

	_, contentError := file.Read(contentBytes)

	Check(contentError)

	fmt.Printf("ODict Version %d\n", version)

	decoded, decodedError := snappy.Decode(nil, contentBytes)

	Check(decodedError)

	dictionary := schema.GetRootAsDictionary(decoded, 0)
	print(dictionary.Entries)
}
