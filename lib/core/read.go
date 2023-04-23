package core

import (
	"bytes"
	"encoding/binary"
	"io/ioutil"
	"os"
	"strconv"

	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
	"github.com/golang/snappy"
)

func readODictBytes(data []byte) (string, uint16, []byte) {
	// Create a bytes reader
	reader := bytes.NewReader(data)

	// Read file signature as bytes
	sigBytes := make([]byte, 5)
	_, sigErr := reader.Read(sigBytes)

	utils.Check(sigErr)

	// Read ODict version as bytes
	reader.Seek(5, 0)

	versionBytes := make([]byte, 2)
	_, versionError := reader.Read(versionBytes)

	utils.Check(versionError)

	// Read the compressed content size in bytes
	reader.Seek(7, 0)

	contentSizeBytes := make([]byte, 8)
	_, contentSizeError := reader.Read(contentSizeBytes)

	utils.Check(contentSizeError)

	reader.Seek(15, 0)

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

	_, contentError := reader.Read(contentBytes)

	utils.Check(contentError)

	decoded, decodedError := snappy.Decode(nil, contentBytes)

	utils.Check(decodedError)

	return signature, readVersion, decoded
}

func readODictFile(path string) (string, uint16, []byte) {
	// Read input file
	file, err := os.Open(path)

	utils.Check(err)

	defer file.Close()

	// Read file contents as bytes
	fileBytes, readErr := ioutil.ReadAll(file)

	utils.Check(readErr)

	// Parse bytes using readODictBytes function
	return readODictBytes(fileBytes)
}

// ReadDictionaryFromPath loads a compiled ODict dictionary from the provided
// path and returns a Dictionary model, with the ability to forcibly re-index
// the dictionary when it loads
func ReadDictionaryFromPath(path string) *types.Dictionary {
	_, _, bytes := readODictFile(path)
	return types.GetRootAsDictionary(bytes, 0)
}

// ReadDictionaryFromBytes loads a compiled ODict dictionary from the provided
// bytes and returns a Dictionary model, with the ability to forcibly re-index
// the dictionary when it loads
func ReadDictionaryFromBytes(bytes []byte) *types.Dictionary {
	_, _, bytes_ := readODictBytes(bytes)
	return types.GetRootAsDictionary(bytes_, 0)
}
