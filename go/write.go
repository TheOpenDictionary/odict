package odict

import (
	"bufio"
	"os"

	"github.com/golang/snappy"
)

func WriteDictionaryFromExisting(outputPath string, dictionary DictionaryRepresentable) int {
	dictionaryBytes := serialize(&dictionary)
	compressed := snappy.Encode(nil, dictionaryBytes)
	file, err := os.Create(outputPath)

	Check(err)

	defer file.Close()

	signature := []byte("ODICT")
	version := Uint16ToBytes(2)
	compressedSize := uint64(len(compressed))
	compressedSizeBytes := Uint64ToBytes(compressedSize)

	writer := bufio.NewWriter(file)

	sigBytes, sigErr := writer.Write(signature)
	versionBytes, versionErr := writer.Write(version)
	contentSizeBytes, contentCountErr := writer.Write(compressedSizeBytes)
	contentBytes, contentErr := writer.Write(compressed)
	total := sigBytes + versionBytes + contentSizeBytes + contentBytes

	Check(sigErr)
	Check(versionErr)
	Check(contentCountErr)
	Check(contentErr)

	Assert(sigBytes == 5, "Signature bytes do not equal 5")
	Assert(versionBytes == 2, "Version bytes do not equal 2")
	Assert(contentSizeBytes == 8, "Content byte count does not equal 8")
	Assert(contentBytes == int(compressedSize), "Content does not equal the computed byte count")

	writer.Flush()

	return total
}

// WriteDictionary generates an ODict binary file given
// a ODXML input file path
func WriteDictionaryFromXML(xmlStr, outputPath string) int {
	return WriteDictionaryFromExisting(outputPath, xmlToDictionaryRepresentable(xmlStr))
}
