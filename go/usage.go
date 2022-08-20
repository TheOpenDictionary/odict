package odict

import (
	"encoding/xml"
	"fmt"
	"strings"

	flatbuffers "github.com/google/flatbuffers/go"
)

func resolveSchemaPOS(pos PartOfSpeech) POS {
	posMap := map[PartOfSpeech]POS{
		"adjective":    POSadj,
		"adj":          POSadj,
		"adverb":       POSadv,
		"adv":          POSadv,
		"verb":         POSv,
		"v":            POSv,
		"n":            POSn,
		"noun":         POSn,
		"pronoun":      POSpro,
		"pn":           POSpro,
		"prep":         POSprep,
		"preposition":  POSprep,
		"conj":         POSconj,
		"conjugation":  POSconj,
		"intj":         POSintj,
		"interjection": POSintj,
		"prefix":       POSpref,
		"pre":          POSpref,
		"suffix":       POSsuff,
		"suf":          POSsuff,
		"particle":     POSpart,
		"part":         POSpart,
		"article":      POSart,
		"art":          POSart,
		"un":           POSun,
		"unknown":      POSun,
	}

	if val, ok := posMap[pos]; ok {
		return val
	} else if len(strings.TrimSpace(string(pos))) == 0 {
		return POSun
	} else {
		panic(fmt.Sprintf("Compilation error: invalid part-of-speech used: %s", pos))
	}
}

type UsageRepresentable struct {
	POS         PartOfSpeech         `json:"pos,omitempty" xml:"pos,attr"`
	Definitions []string             `json:"definitions,omitempty" xml:"definition"`
	Groups      []GroupRepresentable `json:"groups,omitempty" xml:"group"`
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
	pos := resolveSchemaPOS(usage.POS)
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
