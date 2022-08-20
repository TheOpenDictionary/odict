package odict

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type GroupRepresentable struct {
	ID          string   `json:"id" xml:"id,attr"`
	Description string   `json:"description" xml:"description,attr,omitempty"`
	Definitions []string `json:"definitions" xml:"definition"`
}

func (group *Group) AsRepresentable() GroupRepresentable {
	definitions := []string{}

	for e := 0; e < group.DefinitionsLength(); e++ {
		definitions = append(definitions, string(group.Definitions(e)))
	}

	return GroupRepresentable{
		Description: string(group.Description()),
		Definitions: definitions,
	}
}

func (group *GroupRepresentable) AsBuffer(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	id := builder.CreateString(group.ID)
	description := builder.CreateString(group.Description)
	definitions := group.buildDefinitionVector(builder)

	GroupStart(builder)
	GroupAddId(builder, id)
	GroupAddDescription(builder, description)
	GroupAddDefinitions(builder, definitions)

	return GroupEnd(builder)
}

func (group *GroupRepresentable) buildDefinitionVector(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	definitions := group.Definitions
	definitionCount := len(definitions)
	bufs := make([]flatbuffers.UOffsetT, 0, definitionCount)

	for _, definition := range definitions {
		bufs = append(bufs, builder.CreateString(definition))
	}

	GroupStartDefinitionsVector(builder, definitionCount)

	for i := definitionCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(bufs[i])
	}

	return builder.EndVector(definitionCount)
}
