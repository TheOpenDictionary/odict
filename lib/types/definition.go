package types

import (
	flatbuffers "github.com/google/flatbuffers/go"
	"github.com/samber/lo"
)

type Definition struct {
	ID       string   `json:"id,omitempty" xml:"id,attr,omitempty"`
	Value    MDString `json:"value,omitempty" xml:"value,attr"`
	Examples []string `json:"examples,omitempty" xml:"example,omitempty"`
	Notes    []Note   `json:"notes,omitempty" xml:"note,omitempty"`
}

func (definition *DefinitionBuffer) Struct() Definition {
	var note NoteBuffer

	examples := []string{}
	notes := []Note{}

	for n := 0; n < definition.NotesLength(); n++ {
		definition.Notes(&note, n)
		notes = append(notes, note.Struct())
	}

	for e := 0; e < definition.ExamplesLength(); e++ {
		examples = append(examples, string(definition.Examples(e)))
	}

	return Definition{
		ID:       string(definition.Id()),
		Value:    MDString(definition.Value()),
		Examples: examples,
		Notes:    notes,
	}
}

func (def *Definition) Table(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	id := builder.CreateString(def.ID)
	value := builder.CreateString(string(def.Value))
	examples := def.buildExampleVector(builder)
	notes := def.buildNoteVector(builder)

	DefinitionBufferStart(builder)
	DefinitionBufferAddId(builder, id)
	DefinitionBufferAddValue(builder, value)
	DefinitionBufferAddExamples(builder, examples)
	DefinitionBufferAddNotes(builder, notes)

	return DefinitionBufferEnd(builder)
}

func (def *Definition) buildExampleVector(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	examples := def.Examples

	exampleCount := len(examples)

	exampleBuffers := lo.Map(examples, func(example string, _ int) flatbuffers.UOffsetT {
		return builder.CreateString(example)
	})

	DefinitionBufferStartExamplesVector(builder, exampleCount)

	for i := exampleCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(exampleBuffers[i])
	}

	return builder.EndVector(exampleCount)
}

func (def *Definition) buildNoteVector(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	notes := def.Notes

	noteCount := len(notes)

	noteBuffers := lo.Map(notes, func(note Note, _ int) flatbuffers.UOffsetT {
		return note.Table(builder)
	})

	DefinitionBufferStartNotesVector(builder, noteCount)

	for i := noteCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(noteBuffers[i])
	}

	return builder.EndVector(noteCount)
}
