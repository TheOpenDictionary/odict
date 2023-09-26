package core

import (
	"encoding/binary"
	"errors"
	"os"
	"strconv"

	"github.com/TheOpenDictionary/odict/lib/config"
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
	"github.com/golang/snappy"
)

type ODictFile struct {
	Signature string
	Version   uint16
	Content   []byte
}

func readODictFile(path string, decryptionKey *string) (*ODictFile, error) {
	// Read input file
	file, err := os.Open(path)

	if err != nil {
		return nil, err
	}

	defer file.Close()

	// Read file signature as bytes
	sigBytes := make([]byte, 5)

	if _, sigErr := file.Read(sigBytes); sigErr != nil {
		return nil, sigErr
	}

	// Read ODict version as bytes
	file.Seek(5, 0)

	versionBytes := make([]byte, 2)

	if _, versionError := file.Read(versionBytes); versionError != nil {
		return nil, versionError
	}

	encryptedBytes := make([]byte, 2)

	if _, encryptedError := file.Read(versionBytes); encryptedError != nil {
		return nil, encryptedError
	}

	// Read the compressed content size in bytes
	file.Seek(7, 0)

	contentSizeBytes := make([]byte, 8)

	if _, contentSizeError := file.Read(contentSizeBytes); contentSizeError != nil {
		return nil, contentSizeError
	}

	file.Seek(15, 0)

	// Decode bytes for signature, version, and contentSize
	signature := string(sigBytes)
	readVersion := binary.LittleEndian.Uint16(versionBytes)
	encryptedFlag := binary.LittleEndian.Uint16(encryptedBytes)
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
		return nil, errors.New("This file is not compatible with the latest version of the ODict schema")
	}

	// Assert version
	if encryptedFlag == uint16(1) && decryptionKey == nil {
		return nil, errors.New("This dictionary is encrypted. Please pass in a valid decryption key.")
	}

	// Read compressed buffer content as bytes
	contentBytes := make([]byte, contentSize)

	if _, contentError := file.Read(contentBytes); contentError != nil {
		return nil, contentError
	}

	decoded, decodedError := snappy.Decode(nil, contentBytes)

	if decodedError != nil {
		return nil, decodedError
	}

	if decryptionKey != nil {
		bytes, err := utils.DecryptData(*decryptionKey, decoded)

		if err != nil {
			return nil, err
		}

		decoded = bytes
	}

	return &ODictFile{Signature: signature, Version: readVersion, Content: decoded}, nil
}

// ReadDictionary loads a compiled ODict dictionary from the provided
// path and returns a Dictionary model, with the ability to forcibly re-index
// the dictionary when it loads
func ReadDictionaryFromPath(path string, decryptionKey *string) (*types.Dictionary, error) {
	dict, err := readODictFile(path, decryptionKey)

	if err != nil {
		return nil, err
	}

	return types.GetRootAsDictionary(dict.Content, 0), nil
}

// ReadDictionary loads a compiled ODict dictionary from the provided
// path and returns a Dictionary model, with the ability to forcibly re-index
// the dictionary when it loads
func ReadDictionary(pathOrAlias string, decryptionKey *string) (*types.Dictionary, error) {
	dict, err := ReadDictionaryFromPath(pathOrAlias, nil)

	if os.IsNotExist(err) {
		path, err := config.GetDictionaryPathFromAlias(pathOrAlias)

		if err != nil {
			return nil, err
		}

		return ReadDictionaryFromPath(path, nil)
	}

	return dict, err
}
