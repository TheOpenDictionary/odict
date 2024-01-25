package types

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type Group struct {
	ID          string       `json:"id,omitempty" xml:"id,attr,omitempty"`
	Description MDString     `json:"description" xml:"description,attr,omitempty"`
	Definitions []Definition `json:"definitions" xml:"definition"`
}

func (group *GroupBuffer) Struct() Group {
	var definition DefinitionBuffer

	description := string(group.Description())
	definitions := []Definition{}

	if len(description) == 0 {
		panic("All definition groups must have descriptions!")
	}

	for d := 0; d < group.DefinitionsLength(); d++ {
		group.Definitions(&definition, d)
		definitions = append(definitions, definition.Struct())
	}

	return Group{
		Description: MDString(description),
		Definitions: definitions,
	}
}

func (group *Group) Table(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	id := builder.CreateString(group.ID)
	description := builder.CreateString(group.Description.String())
	definitions := group.buildDefinitionVector(builder)

	if len(group.Description) == 0 {
		panic("All definition groups must have descriptions!")
	}

	GroupBufferStart(builder)
	GroupBufferAddId(builder, id)
	GroupBufferAddDescription(builder, description)
	GroupBufferAddDefinitions(builder, definitions)

	return GroupBufferEnd(builder)
}

func (group *Group) buildDefinitionVector(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	definitions := group.Definitions
	definitionCount := len(definitions)
	bufs := make([]flatbuffers.UOffsetT, 0, definitionCount)

	for _, definition := range definitions {
		bufs = append(bufs, definition.Table(builder))
	}

	GroupBufferStartDefinitionsVector(builder, definitionCount)

	for i := definitionCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(bufs[i])
	}

	return builder.EndVector(definitionCount)
}
