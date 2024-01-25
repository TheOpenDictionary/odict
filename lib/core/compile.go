package core

import (
	"errors"
	"io"
	"os"
	"strconv"

	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
	"github.com/golang/snappy"
)

func createFileBytes(dictionary *types.Dictionary) ([]byte, error) {
	compressed := snappy.Encode(nil, dictionary.Bytes())
	versionInt, parseErr := strconv.Atoi(version)

	if parseErr != nil {
		return nil, parseErr
	}

	signature := []byte("ODICT")

	version := utils.Uint16ToBytes(uint16(versionInt))

	compressedSize := uint64(len(compressed))

	compressedSizeBytes := utils.Uint64ToBytes(compressedSize)
	totalSize := len(signature) + len(version) + len(compressedSizeBytes) + len(compressed)

	output := make([]byte, 0, totalSize)

	output = append(output, signature...)
	output = append(output, version...)
	output = append(output, compressedSizeBytes...)
	output = append(output, compressed...)

	if len(signature) != 5 {
		return nil, errors.New("signature bytes do not equal 5")
	}

	if len(version) != 2 {
		return nil, errors.New("version bytes do not equal 2")
	}

	if len(compressedSizeBytes) != 8 {
		return nil, errors.New("content byte count does not equal 8")
	}

	if len(compressed) != int(compressedSize) {
		return nil, errors.New("content does not equal the computed byte count")
	}

	return output, nil
}

func CompileDictionary(dictionary *types.Dictionary) ([]byte, error) {
	serialized, err := createFileBytes(dictionary)

	if err != nil {
		return nil, err
	}

	return serialized, nil
}

func CompileXML(xmlStr string) ([]byte, error) {
	return CompileDictionary(types.NewDictionary(xmlStr))
}

// CompilePath compiles an XML file at the given path into an ODict binary and writes to disk
func CompilePath(xmlPath string, outputPath string) (int, error) {
	xmlFile, err := os.Open(xmlPath)

	if err != nil {
		return 0, err
	}

	defer xmlFile.Close()

	xmlStr, err := io.ReadAll(xmlFile)

	if err != nil {
		return 0, err
	}

	bytes, err := CompileXML(string(xmlStr))

	if err != nil {
		return 0, err
	}

	return writeToFile(bytes, outputPath)
}
