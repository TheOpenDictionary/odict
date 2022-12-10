package odict

import (
	"encoding/xml"

	flatbuffers "github.com/google/flatbuffers/go"
)

type UsageRepresentable struct {
	POS         PartOfSpeech         `json:"pos,omitempty" xml:"pos,attr"`
	Definitions []string             `json:"definitions" xml:"definition"`
	Groups      []GroupRepresentable `json:"groups" xml:"group"`
	XMLName     xml.Name             `json:"-" xml:"usage"`
}

func (usage UsageRepresentable) Key() PartOfSpeech {
	return usage.POS
}

func (usage *Usage) AsRepresentable() UsageRepresentable {
	var group Group

	groups := []GroupRepresentable{}
	definitions := []string{}

	for d := 0; d < usage.DefinitionsLength(); d++ {
		definitions = append(definitions, string(usage.Definitions(d)))
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
	definitions := Map(usage.Definitions, builder.CreateString)
	definitionCount := len(definitions)

	GroupStartDefinitionsVector(builder, definitionCount)

	for i := definitionCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(definitions[i])
	}

	return builder.EndVector(definitionCount)
}
