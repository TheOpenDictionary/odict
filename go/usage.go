package odict

import (
	"encoding/xml"

	flatbuffers "github.com/google/flatbuffers/go"
)

type UsageRepresentable struct {
	POS         PartOfSpeech              `json:"-" xml:"pos,attr"`
	Definitions []DefinitionRepresentable `json:"definitions,omitempty" xml:"definition"`
	Groups      []GroupRepresentable      `json:"groups,omitempty" xml:"group"`
	XMLName     xml.Name                  `json:"-" xml:"usage"`
}

func (usage UsageRepresentable) Key() PartOfSpeech {
	return usage.POS
}

func (usage *Usage) AsRepresentable() UsageRepresentable {
	var group Group
	var definition Definition

	groups := []GroupRepresentable{}
	definitions := []DefinitionRepresentable{}

	for d := 0; d < usage.DefinitionsLength(); d++ {
		usage.Definitions(&definition, d)
		definitions = append(definitions, definition.AsRepresentable())
	}

	for g := 0; g < usage.GroupsLength(); g++ {
		usage.Groups(&group, g)
		groups = append(groups, group.AsRepresentable())
	}

	return UsageRepresentable{
		POS:         PartOfSpeech(usage.Pos().String()),
		Groups:      groups,
		Definitions: definitions,
	}
}

func (usage *UsageRepresentable) AsBuffer(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	pos := partOfSpeechToPOS(usage.POS)
	groups := usage.buildGroupVector(builder)
	definitions := usage.buildDefinitionVector(builder)

	UsageStart(builder)
	UsageAddPos(builder, pos)
	UsageAddGroups(builder, groups)
	UsageAddDefinitions(builder, definitions)

	return UsageEnd(builder)
}

func (usage *UsageRepresentable) buildGroupVector(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	groups := Map(usage.Groups, func(group GroupRepresentable) flatbuffers.UOffsetT {
		return group.AsBuffer(builder)
	})

	groupCount := len(groups)

	UsageStartGroupsVector(builder, groupCount)

	for i := groupCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(groups[i])
	}

	return builder.EndVector(groupCount)
}

func (usage *UsageRepresentable) buildDefinitionVector(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	definitions := Map(usage.Definitions, func(d DefinitionRepresentable) flatbuffers.UOffsetT {
		return d.AsBuffer(builder)
	})

	definitionCount := len(definitions)

	GroupStartDefinitionsVector(builder, definitionCount)

	for i := definitionCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(definitions[i])
	}

	return builder.EndVector(definitionCount)
}
