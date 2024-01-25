package types

import (
	"strings"

	flatbuffers "github.com/google/flatbuffers/go"
	"github.com/samber/lo"
)

type Serializable interface {
	Table(builder *flatbuffers.Builder) flatbuffers.UOffsetT
}

func formatPOSTag(tag string) string {
	return strings.ReplaceAll(tag, "_", "-")
}

func strToPartOfSpeech(str string) PartOfSpeech {
	if val, ok := posTagPartOfSpeechMap[formatPOSTag(str)]; ok {
		return val
	}

	return Unknown
}

func getBytes(s Serializable) []byte {
	builder := flatbuffers.NewBuilder(0)

	buffer := s.Table(builder)

	builder.Finish(buffer)

	return builder.FinishedBytes()
}

func NestedEntriesStructs(entries [][]EntryBuffer) [][]Entry {
	return lo.Map(entries, func(e []EntryBuffer, _ int) []Entry {
		return EntriesStructs(e)
	})
}

func EntriesStructs(entries []EntryBuffer) []Entry {
	return lo.Map(entries, func(entry EntryBuffer, _ int) Entry {
		return entry.Struct()
	})
}
