package api

import (
	"encoding/binary"
	"os"

	"github.com/blevesearch/bleve"
	"github.com/odict/odict/schema"
	"github.com/odict/odict/utils"

	"github.com/golang/snappy"
)

func createIndex(dictionary OpenDictionary) {
	mapping := bleve.NewIndexMapping()
	index, err := bleve.New("~/.odict/example.index", mapping)

	utils.Check(err)

	for entryIdx := range dictionary.Entries {
		entry := dictionary.Entries[entryIdx]
		err = index.Index(entry.Term, entry)
		utils.Check(err)
	}

	query := bleve.NewMatchQuery("run")
	search := bleve.NewSearchRequest(query)
	searchResults, err := index.Search(search)

	print(searchResults.Total)
}

func getODDefinitionGroups(usage schema.Usage) []OpenDictionaryDefinitionGroup {
	var definitionGroup schema.Group
	var definitionGroups []OpenDictionaryDefinitionGroup

	for d := 0; d < usage.GroupsLength(); d++ {
		usage.Groups(&definitionGroup, d)

		odGroup := OpenDictionaryDefinitionGroup{
			Description: string(definitionGroup.Description()),
		}

		definitionGroups = append(definitionGroups, odGroup)
	}

	return definitionGroups
}

func getODUsages(etymology schema.Etymology) []OpenDictionaryUsage {
	var usage schema.Usage
	var usages []OpenDictionaryUsage

	for c := 0; c < etymology.UsagesLength(); c++ {
		etymology.Usages(&usage, c)

		odUsage := OpenDictionaryUsage{
			POS:              string(usage.Pos()),
			DefinitionGroups: getODDefinitionGroups(usage),
		}

		usages = append(usages, odUsage)
	}

	return usages
}

func getODEtymologies(entry schema.Entry) []OpenDictionaryEtymology {
	var ety schema.Etymology
	var etymologies []OpenDictionaryEtymology

	for b := 0; b < entry.EtymologiesLength(); b++ {
		entry.Etymologies(&ety, b)

		odEty := OpenDictionaryEtymology{
			ID:     string(ety.Id()),
			Usages: getODUsages(ety),
		}

		etymologies = append(etymologies, odEty)
	}

	return etymologies
}

func getODEntries(dictionary *schema.Dictionary) []OpenDictionaryEntry {
	var entry schema.Entry
	var entries []OpenDictionaryEntry

	for a := 0; a < dictionary.EntriesLength(); a++ {
		dictionary.Entries(&entry, a)

		odEntry := OpenDictionaryEntry{
			Term:        string(entry.Term()),
			Etymologies: getODEtymologies(entry),
		}

		entries = append(entries, odEntry)
	}

	return entries
}

// LoadDictionary can go fuck itself
func LoadDictionary(inputPath string) OpenDictionary {
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
	file.Seek(7, 0)

	contentSizeBytes := make([]byte, 4)
	_, contentSizeError := file.Read(contentSizeBytes)

	utils.Check(contentSizeError)
	file.Seek(11, 0)

	// Decode bytes for signature, version, and contentSize
	signature := string(sigBytes)
	version := binary.LittleEndian.Uint16(versionBytes)
	contentSize := binary.LittleEndian.Uint32(contentSizeBytes)

	// utils.Assert signature
	utils.Assert(signature == "ODICT", "Invalid file signature")

	// Read compressed buffer content as bytes
	contentBytes := make([]byte, contentSize)

	_, contentError := file.Read(contentBytes)

	utils.Check(contentError)

	decoded, decodedError := snappy.Decode(nil, contentBytes)

	utils.Check(decodedError)

	buffer := schema.GetRootAsDictionary(decoded, 0)
	dictionary := OpenDictionary{
		Version: version,
		Entries: getODEntries(buffer),
	}

	createBleveIndex(dictionary)

	return dictionary
}
