package odict

import (
	"encoding/json"
	"encoding/xml"

	flatbuffers "github.com/google/flatbuffers/go"
)

var sqlDictionaryId int = 1
var sqlEntryId int = 1
var sqlEtymologyId int = 1
var sqlUsageId int = 1
var sqlGroupId int = 1
var sqlDefinitionId int = 1
var sqlExampleId int = 1

func strToPartOfSpeech(str string) PartOfSpeech {
	if val, ok := posTagPartOfSpeechMap[str]; ok {
		return val
	}

	return Unknown
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
