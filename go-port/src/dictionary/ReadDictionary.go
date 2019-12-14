package dictionary

import (
	"encoding/binary"
	"fmt"
	"odict/schema"
	"odict/utils"
	"os"

	"github.com/golang/snappy"
)

// ReadDictionary can go fuck itself
func ReadDictionary(inputPath string) {
	// Read input file
	file, err := os.Open(inputPath)

	utils.Check(err)

	defer file.Close()

	// Read file signature as bytes
	sigBytes := make([]byte, 5)
	_, sigErr := file.Read(sigBytes)

	utils.Check(sigErr)

	// Read ODict version as bytes
	file.Seek(5, 0)

	versionBytes := make([]byte, 2)
	_, versionError := file.Read(versionBytes)

	utils.Check(versionError)

	// Read the compressed content size in bytes
	file.Seek(2, 1)

	contentSizeBytes := make([]byte, 4)
	_, contentSizeError := file.Read(contentSizeBytes)

	utils.Check(contentSizeError)
	file.Seek(4, 1)

	// Decode bytes for signature, version, and contentSize
	signature := string(sigBytes)
	version := binary.LittleEndian.Uint16(versionBytes)
	contentSize := binary.LittleEndian.Uint32(contentSizeBytes)

	// Assert signature
	utils.Assert(signature == "ODICT", "Invalid file signature")

	// Read compressed buffer content as bytes
	contentBytes := make([]byte, contentSize)

	_, contentError := file.Read(contentBytes)

	utils.Check(contentError)
	println(contentSize)

	fmt.Printf("ODict Version %d\n", version)

	decoded, decodedError := snappy.Decode(nil, contentBytes)

	utils.Check(decodedError)

	dictionary := schema.GetRootAsDictionary(decoded, 0)
	print(dictionary.Entries)
}
