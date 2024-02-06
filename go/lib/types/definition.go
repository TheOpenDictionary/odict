package types

import (
	flatbuffers "github.com/google/flatbuffers/go"
	"github.com/samber/lo"
)

type DefinitionRepresentable struct {
	ID       string              `json:"id,omitempty" xml:"id,attr,omitempty"`
	Value    MDString            `json:"value,omitempty" xml:"value,attr"`
	Examples []string            `json:"examples,omitempty" xml:"example,omitempty"`
	Notes    []NoteRepresentable `json:"notes,omitempty" xml:"note,omitempty"`
}

func (definition *Definition) AsRepresentable() DefinitionRepresentable {
	var note Note

	examples := []string{}
	notes := []NoteRepresentable{}

	for n := 0; n < definition.NotesLength(); n++ {
		definition.Notes(&note, n)
		notes = append(notes, note.AsRepresentable())
	}

	for e := 0; e < definition.ExamplesLength(); e++ {
		examples = append(examples, string(definition.Examples(e)))
	}

	return DefinitionRepresentable{
		ID:       string(definition.Id()),
		Value:    MDString(definition.Value()),
		Examples: examples,
		Notes:    notes,
	}
}

func (def *DefinitionRepresentable) AsBuffer(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	id := builder.CreateString(def.ID)
	value := builder.CreateString(string(def.Value))
	examples := def.buildExampleVector(builder)
	notes := def.buildNoteVector(builder)

	DefinitionStart(builder)
	DefinitionAddId(builder, id)
	DefinitionAddValue(builder, value)
	DefinitionAddExamples(builder, examples)
	DefinitionAddNotes(builder, notes)

	return DefinitionEnd(builder)
}

func (def *DefinitionRepresentable) buildExampleVector(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	examples := def.Examples

	exampleCount := len(examples)

	exampleBuffers := lo.Map(examples, func(example string, _ int) flatbuffers.UOffsetT {
		return builder.CreateString(example)
	})

	DefinitionStartExamplesVector(builder, exampleCount)

	for i := exampleCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(exampleBuffers[i])
	}

	return builder.EndVector(exampleCount)
}

func (def *DefinitionRepresentable) buildNoteVector(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	notes := def.Notes

	noteCount := len(notes)

	noteBuffers := lo.Map(notes, func(note NoteRepresentable, _ int) flatbuffers.UOffsetT {
		return note.AsBuffer(builder)
	})

	DefinitionStartNotesVector(builder, noteCount)

	for i := noteCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(noteBuffers[i])
	}

	return builder.EndVector(noteCount)
}
