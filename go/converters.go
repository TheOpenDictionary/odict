package odict

import (
	"encoding/json"
	"encoding/xml"
	"log"

	flatbuffers "github.com/google/flatbuffers/go"
)

func posToPartOfSpeech(pos POS) PartOfSpeech {
	if val, ok := posPartOfSpeechMap[pos]; ok {
		return val
	}

	log.Fatalf("Invalid part-of-speech used: %s", pos)

	return PartOfSpeech{} // Should never happen, dw
}

func strToPartOfSpeech(str string) PartOfSpeech {
	if val, ok := posTagPartOfSpeechMap[str]; ok {
		return val
	}

	log.Fatalf("Invalid part-of-speech used: %s", str)

	return PartOfSpeech{} // Should never happen, dw
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
