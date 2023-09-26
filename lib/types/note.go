package types

import (
	flatbuffers "github.com/google/flatbuffers/go"
	"github.com/samber/lo"
)

type NoteRepresentable struct {
	ID       string   `json:"id,omitempty" xml:"id,attr,omitempty"`
	Value    MDString `json:"value,omitempty" xml:"value,attr"`
	Examples []string `json:"examples,omitempty" xml:"example"`
}

func (note *Note) AsRepresentable() NoteRepresentable {
	examples := []string{}

	for e := 0; e < note.ExamplesLength(); e++ {
		examples = append(examples, string(note.Examples(e)))
	}

	return NoteRepresentable{
		ID:       string(note.Id()),
		Value:    MDString(note.Value()),
		Examples: examples,
	}
}

func (note *NoteRepresentable) AsBuffer(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	id := builder.CreateString(note.ID)
	value := builder.CreateString(note.Value.String())
	examples := note.buildExampleVector(builder)

	NoteStart(builder)
	NoteAddId(builder, id)
	NoteAddValue(builder, value)
	NoteAddExamples(builder, examples)

	return NoteEnd(builder)
}

func (note *NoteRepresentable) buildExampleVector(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	examples := note.Examples
	exampleCount := len(examples)

	exampleBuffers := lo.Map(examples, func(example string, _ int) flatbuffers.UOffsetT {
		return builder.CreateString(example)
	})

	NoteStartExamplesVector(builder, exampleCount)

	for i := exampleCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(exampleBuffers[i])
	}

	return builder.EndVector(exampleCount)
}
