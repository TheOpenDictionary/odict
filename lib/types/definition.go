package types

import (
	flatbuffers "github.com/google/flatbuffers/go"
	"github.com/samber/lo"
)

type DefinitionRepresentable struct {
	ID       string   `json:"id,omitempty" xml:"id,attr,omitempty"`
	Value    string   `json:"value,omitempty" xml:"value,attr"`
	Examples []string `json:"examples,omitempty" xml:"example"`
}

func (definition *Definition) AsRepresentable() DefinitionRepresentable {
	examples := []string{}

	for e := 0; e < definition.ExamplesLength(); e++ {
		examples = append(examples, string(definition.Examples(e)))
	}

	return DefinitionRepresentable{
		ID:       string(definition.Id()),
		Value:    string(definition.Value()),
		Examples: examples,
	}
}

func (def *DefinitionRepresentable) AsBuffer(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	id := builder.CreateString(def.ID)
	value := builder.CreateString(def.Value)
	examples := def.buildExampleVector(builder)

	DefinitionStart(builder)
	DefinitionAddId(builder, id)
	DefinitionAddValue(builder, value)
	DefinitionAddExamples(builder, examples)

	return DefinitionEnd(builder)
}

func (def *DefinitionRepresentable) buildExampleVector(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	examples := def.Examples
	exampleCount := len(examples)

	exampleBuffers := lo.Map(examples, func(example string, _ int) flatbuffers.UOffsetT {
		return builder.CreateString(example)
	})

	EtymologyStartUsagesVector(builder, exampleCount)

	for i := exampleCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(exampleBuffers[i])
	}

	return builder.EndVector(exampleCount)
}
