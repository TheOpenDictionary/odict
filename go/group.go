package odict

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type GroupRepresentable struct {
	ID          string                    `json:"id,omitempty" xml:"id,attr"`
	Description string                    `json:"description" xml:"description,attr"`
	Definitions []DefinitionRepresentable `json:"definitions" xml:"definition"`
}

func (group *Group) AsRepresentable() GroupRepresentable {
	var definition Definition

	description := string(group.Description())
	definitions := []DefinitionRepresentable{}

	Assert(len(description) != 0, "All definition groups must have descriptions!")

	for d := 0; d < group.DefinitionsLength(); d++ {
		group.Definitions(&definition, d)
		definitions = append(definitions, definition.AsRepresentable())
	}

	return GroupRepresentable{
		Description: description,
		Definitions: definitions,
	}
}

func (group *GroupRepresentable) AsBuffer(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	id := builder.CreateString(group.ID)
	description := builder.CreateString(group.Description)
	definitions := group.buildDefinitionVector(builder)

	Assert(len(group.Description) != 0, "All definition groups must have descriptions!")

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
		bufs = append(bufs, definition.AsBuffer(builder))
	}

	GroupStartDefinitionsVector(builder, definitionCount)

	for i := definitionCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(bufs[i])
	}

	return builder.EndVector(definitionCount)
}
