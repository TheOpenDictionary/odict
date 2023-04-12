package core

import (
	"encoding/binary"
	"os"
	"strconv"

	"github.com/golang/snappy"
)

func readODictFile(path string) (string, uint16, []byte) {
	// Read input file
	file, err := os.Open(path)

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

	contentSizeBytes := make([]byte, 8)
	_, contentSizeError := file.Read(contentSizeBytes)

	Check(contentSizeError)

	file.Seek(15, 0)

	// Decode bytes for signature, version, and contentSize
	signature := string(sigBytes)
	readVersion := binary.LittleEndian.Uint16(versionBytes)
	contentSize := binary.LittleEndian.Uint64(contentSizeBytes)
	expectedVersion, parseErr := strconv.Atoi(version)

	Check(parseErr)

	// Assert signature
	Assert(signature == "ODICT", "This is not an ODict file!")

	// Assert version
	Assert(readVersion == uint16(expectedVersion), "This file is not compatible with the latest version of the ODict schema!")

	// Read compressed buffer content as bytes
	contentBytes := make([]byte, contentSize)

	_, contentError := file.Read(contentBytes)

	Check(contentError)

	decoded, decodedError := snappy.Decode(nil, contentBytes)

	Check(decodedError)

	return signature, readVersion, decoded
}

// ReadDictionaryFromPath loads a compiled ODict dictionary from the provided
// path and returns a Dictionary model, with the ability to forcibly re-index
// the dictionary when it loads
func ReadDictionaryFromPath(path string) *Dictionary {
	_, _, bytes := readODictFile(path)
	return GetRootAsDictionary(bytes, 0)
}
