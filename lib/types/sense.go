package types

import (
	"encoding/xml"

	flatbuffers "github.com/google/flatbuffers/go"
	"github.com/samber/lo"
)

type SenseRepresentable struct {
	POS         PartOfSpeech              `json:"-" xml:"pos,attr"`
	Groups      []GroupRepresentable      `json:"groups,omitempty" xml:"group"`
	Definitions []DefinitionRepresentable `json:"definitions,omitempty" xml:"definition"`
	XMLName     xml.Name                  `json:"-" xml:"sense"`
}

func (sr *SenseRepresentable) UnmarshalXML(d *xml.Decoder, start xml.StartElement) error {
	type SR SenseRepresentable // new type to prevent recursion

	item := SR{POS: Unknown} // sets default POS to Unknown if unspecified in XML

	if err := d.DecodeElement(&item, &start); err != nil {
		return err
	}

	*sr = (SenseRepresentable)(item)

	return nil
}

func (sense SenseRepresentable) Key() PartOfSpeech {
	return sense.POS
}

func (sense *Sense) AsRepresentable() SenseRepresentable {
	var group Group
	var definition Definition

	groups := []GroupRepresentable{}
	definitions := []DefinitionRepresentable{}

	for d := 0; d < sense.DefinitionsLength(); d++ {
		sense.Definitions(&definition, d)
		definitions = append(definitions, definition.AsRepresentable())
	}

	for g := 0; g < sense.GroupsLength(); g++ {
		sense.Groups(&group, g)
		groups = append(groups, group.AsRepresentable())
	}

	return SenseRepresentable{
		POS:         strToPartOfSpeech(sense.Pos().String()),
		Groups:      groups,
		Definitions: definitions,
	}
}

func (sense *SenseRepresentable) AsBuffer(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	pos := sense.POS.Buf
	groups := sense.buildGroupVector(builder)
	definitions := sense.buildDefinitionVector(builder)

	SenseStart(builder)
	SenseAddPos(builder, pos)
	SenseAddGroups(builder, groups)
	SenseAddDefinitions(builder, definitions)

	return SenseEnd(builder)
}

func (sense *SenseRepresentable) buildGroupVector(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	groups := lo.Map(sense.Groups, func(group GroupRepresentable, _ int) flatbuffers.UOffsetT {
		return group.AsBuffer(builder)
	})

	groupCount := len(groups)

	SenseStartGroupsVector(builder, groupCount)

	for i := groupCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(groups[i])
	}

	return builder.EndVector(groupCount)
}

func (sense *SenseRepresentable) buildDefinitionVector(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	definitions := lo.Map(sense.Definitions, func(d DefinitionRepresentable, _ int) flatbuffers.UOffsetT {
		return d.AsBuffer(builder)
	})

	definitionCount := len(definitions)

	SenseStartDefinitionsVector(builder, definitionCount)

	for i := definitionCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(definitions[i])
	}

	return builder.EndVector(definitionCount)
}
