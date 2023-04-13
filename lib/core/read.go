package core

import (
	"encoding/binary"
	"os"
	"strconv"

	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
	"github.com/golang/snappy"
)

func readODictFile(path string) (string, uint16, []byte) {
	// Read input file
	file, err := os.Open(path)

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
	file.Seek(7, 0)

	contentSizeBytes := make([]byte, 8)
	_, contentSizeError := file.Read(contentSizeBytes)

	utils.Check(contentSizeError)

	file.Seek(15, 0)

	// Decode bytes for signature, version, and contentSize
	signature := string(sigBytes)
	readVersion := binary.LittleEndian.Uint16(versionBytes)
	contentSize := binary.LittleEndian.Uint64(contentSizeBytes)
	expectedVersion, parseErr := strconv.Atoi(version)

	utils.Check(parseErr)

	// Assert signature
	utils.Assert(signature == "ODICT", "This is not an ODict file!")

	// Assert version
	utils.Assert(readVersion == uint16(expectedVersion), "This file is not compatible with the latest version of the ODict schema!")

	// Read compressed buffer content as bytes
	contentBytes := make([]byte, contentSize)

	_, contentError := file.Read(contentBytes)

	utils.Check(contentError)

	decoded, decodedError := snappy.Decode(nil, contentBytes)

	utils.Check(decodedError)

	return signature, readVersion, decoded
}

// ReadDictionaryFromPath loads a compiled ODict dictionary from the provided
// path and returns a Dictionary model, with the ability to forcibly re-index
// the dictionary when it loads
func ReadDictionaryFromPath(path string) *types.Dictionary {
	_, _, bytes := readODictFile(path)
	return types.GetRootAsDictionary(bytes, 0)
}
