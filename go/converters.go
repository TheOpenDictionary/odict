package odict

import (
	"encoding/json"
	"encoding/xml"
	"fmt"
	"html"
	"regexp"
	"strings"

	flatbuffers "github.com/google/flatbuffers/go"
)

var partOfSpeechNameMap = map[PartOfSpeech]string{
	Adjective:    "adjective",
	Adverb:       "adverb",
	Article:      "article",
	Conjugation:  "conjugation",
	Interjection: "interjection",
	Noun:         "noun",
	Particle:     "particle",
	Prefix:       "prefix",
	Preposition:  "preposition",
	Pronoun:      "pronoun",
	Suffix:       "suffix",
	Unknown:      "unknown part-of-speech",
	Verb:         "verb",
}

var partOfSpeechPOSMap = map[PartOfSpeech]POS{
	Adjective:    POSadj,
	Adverb:       POSadv,
	Article:      POSart,
	Conjugation:  POSconj,
	Interjection: POSintj,
	Noun:         POSn,
	Particle:     POSpart,
	Prefix:       POSpref,
	Preposition:  POSprep,
	Pronoun:      POSpro,
	Suffix:       POSsuff,
	Unknown:      POSun,
	Verb:         POSv,
}

func (pos PartOfSpeech) Name() string {
	if val, ok := partOfSpeechNameMap[pos]; ok {
		return val
	}

	panic(fmt.Sprintf("Error: invalid part-of-speech: %s", pos))
}

var posPartOfSpeechMap = func() map[POS]PartOfSpeech {
	posMap := map[POS]PartOfSpeech{}

	for k, v := range partOfSpeechPOSMap {
		posMap[v] = k
	}

	return posMap
}()

func posToPartOfSpeech(pos POS) PartOfSpeech {
	if val, ok := posPartOfSpeechMap[pos]; ok {
		return val
	}

	panic(fmt.Sprintf("Read error: invalid part-of-speech used: %s", pos))
}

func partOfSpeechToPOS(pos PartOfSpeech) POS {
	if val, ok := partOfSpeechPOSMap[pos]; ok {
		return val
	}

	if len(strings.TrimSpace(string(pos))) == 0 {
		return POSun
	}

	panic(fmt.Sprintf("Write error: invalid part-of-speech used: %s", pos))
}

func xmlToDictionaryRepresentable(xmlStr string) DictionaryRepresentable {
	var dictionary DictionaryRepresentable

	xml.Unmarshal([]byte(xmlStr), &dictionary)

	r := regexp.MustCompile("<entry.*?term=\"(.*?)\">")
	entries := r.FindAllStringSubmatch(xmlStr, -1)
	expectedEntries := len(entries)
	actualEntries := len(dictionary.Entries)

	if expectedEntries != actualEntries {

		fmt.Printf("WARNING: The dictionary that was read into memory from XML is missing entries. %d entries were read when there should be %d total. Are you sure your XML is 100%% valid and there are no duplicate entries?\n", actualEntries, expectedEntries)

		for _, entry := range entries {
			v := html.UnescapeString(entry[1])

			if _, ok := dictionary.Entries[v]; !ok {
				fmt.Printf("- %s\n", v)
			}
		}
	}

	return dictionary
}

func serialize(b Serializable) []byte {
	builder := flatbuffers.NewBuilder(0)
	buffer := b.AsBuffer(builder)

	builder.Finish(buffer)

	return builder.FinishedBytes()
}

func JSON(any interface{}) string {
	b, err := json.MarshalIndent(&any, "", " ")

	Check(err)

	return string(b)
}

func XML(any interface{}) string {
	str, err := xml.MarshalIndent(&any, "", " ")

	Check(err)

	return string(str)
}
