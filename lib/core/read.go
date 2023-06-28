package core

import (
	"bytes"
	"encoding/binary"
	"errors"
	"io/ioutil"
	"os"
	"strconv"

	"github.com/TheOpenDictionary/lib/config"
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/golang/snappy"
)

type ODictFile struct {
	Signature string
	Version   uint16
	Content   []byte
}

func readODictBytes(data []byte) (*ODictFile, error) {
	// Create a bytes reader
	reader := bytes.NewReader(data)

	// Read file signature as bytes
	sigBytes := make([]byte, 5)
	_, sigErr := reader.Read(sigBytes)

	if sigErr != nil {
		return nil, sigErr
	}

	// Read ODict version as bytes
	reader.Seek(5, 0)

	versionBytes := make([]byte, 2)
	_, versionError := reader.Read(versionBytes)

	if versionError != nil {
		return nil, versionError
	}

	// Read the compressed content size in bytes
	reader.Seek(7, 0)

	contentSizeBytes := make([]byte, 8)
	_, contentSizeError := reader.Read(contentSizeBytes)

	if contentSizeError != nil {
		return nil, contentSizeError
	}

	reader.Seek(15, 0)

	// Decode bytes for signature, version, and contentSize
	signature := string(sigBytes)
	readVersion := binary.LittleEndian.Uint16(versionBytes)
	contentSize := binary.LittleEndian.Uint64(contentSizeBytes)
	expectedVersion, parseErr := strconv.Atoi(version)

	if parseErr != nil {
		return nil, parseErr
	}

	// Assert signature
	if signature != "ODICT" {
		return nil, errors.New("this is not an ODict file")
	}

	// Assert version
	if readVersion != uint16(expectedVersion) {
		return nil, errors.New("this file is not compatible with the latest version of the ODict schema")
	}

	// Read compressed buffer content as bytes
	contentBytes := make([]byte, contentSize)

	_, contentError := reader.Read(contentBytes)

	if contentError != nil {
		return nil, contentError
	}

	decoded, decodedError := snappy.Decode(nil, contentBytes)

	if decodedError != nil {
		return nil, decodedError
	}

	return &ODictFile{Signature: signature, Version: readVersion, Content: decoded}, nil
}

func readODictFile(path string) (*ODictFile, error) {
	// Read input file
	file, err := os.Open(path)

	if err != nil {
		return nil, err
	}

	defer file.Close()

	// Read file contents as bytes
	fileBytes, readErr := ioutil.ReadAll(file)

	if readErr != nil {
		return nil, readErr
	}

	// Parse bytes using readODictBytes function
	return readODictBytes(fileBytes)
}

// ReadDictionaryFromPath loads a compiled ODict dictionary from the provided
// path and returns a Dictionary model, with the ability to forcibly re-index
// the dictionary when it loads
func ReadDictionaryFromPath(path string) (*types.Dictionary, error) {
	dict, err := readODictFile(path)

	if err != nil {
		return nil, err
	}

	return types.GetRootAsDictionary(dict.Content, 0), nil
}

// ReadDictionary loads a compiled ODict dictionary from the provided
// path and returns a Dictionary model, with the ability to forcibly re-index
// the dictionary when it loads
func ReadDictionary(pathOrAlias string) (*types.Dictionary, error) {
	dict, err := ReadDictionaryFromPath(pathOrAlias)

	if dict == nil || os.IsNotExist(err) {
		path, err := config.GetDictionaryPathFromAlias(pathOrAlias)

		if err != nil {
			return nil, err
		}

		return ReadDictionaryFromPath(path)
	}

	return dict, err
}

// ReadDictionaryFromBytes loads a compiled ODict dictionary from the provided
// bytes and returns a Dictionary model, with the ability to forcibly re-index
// the dictionary when it loads
func ReadDictionaryFromBytes(bytes []byte) (*types.Dictionary, error) {
	file, err := readODictBytes(bytes)

	if err != nil {
		return nil, err
	}

	return types.GetRootAsDictionary(file.Content, 0), nil
}
