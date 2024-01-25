package types

import (
	"encoding/xml"

	flatbuffers "github.com/google/flatbuffers/go"
	"github.com/samber/lo"
)

type Sense struct {
	POS         PartOfSpeech `json:"-" xml:"pos,attr,omitempty"`
	Groups      []Group      `json:"groups,omitempty" xml:"group,omitempty"`
	Definitions []Definition `json:"definitions,omitempty" xml:"definition,omitempty"`
	XMLName     xml.Name     `json:"-" xml:"sense"`
}

func (sr *Sense) UnmarshalXML(d *xml.Decoder, start xml.StartElement) error {
	type SR Sense // new type to prevent recursion

	item := SR{POS: Unknown} // sets default POS to Unknown if unspecified in XML

	if err := d.DecodeElement(&item, &start); err != nil {
		return err
	}

	*sr = (Sense)(item)

	return nil
}

func (sense Sense) Key() PartOfSpeech {
	return sense.POS
}

func (sense *SenseBuffer) Struct() Sense {
	var group GroupBuffer
	var definition DefinitionBuffer

	groups := []Group{}
	definitions := []Definition{}

	for d := 0; d < sense.DefinitionsLength(); d++ {
		sense.Definitions(&definition, d)
		definitions = append(definitions, definition.Struct())
	}

	for g := 0; g < sense.GroupsLength(); g++ {
		sense.Groups(&group, g)
		groups = append(groups, group.Struct())
	}

	return Sense{
		POS:         strToPartOfSpeech(sense.Pos().String()),
		Groups:      groups,
		Definitions: definitions,
	}
}

func (sense *Sense) Table(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	pos := sense.POS.Buf
	groups := sense.buildGroupVector(builder)
	definitions := sense.buildDefinitionVector(builder)

	SenseBufferStart(builder)
	SenseBufferAddPos(builder, pos)
	SenseBufferAddGroups(builder, groups)
	SenseBufferAddDefinitions(builder, definitions)

	return SenseBufferEnd(builder)
}

func (sense *Sense) buildGroupVector(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	groups := lo.Map(sense.Groups, func(group Group, _ int) flatbuffers.UOffsetT {
		return group.Table(builder)
	})

	groupCount := len(groups)

	SenseBufferStartGroupsVector(builder, groupCount)

	for i := groupCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(groups[i])
	}

	return builder.EndVector(groupCount)
}

func (sense *Sense) buildDefinitionVector(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	definitions := lo.Map(sense.Definitions, func(d Definition, _ int) flatbuffers.UOffsetT {
		return d.Table(builder)
	})

	definitionCount := len(definitions)

	SenseBufferStartDefinitionsVector(builder, definitionCount)

	for i := definitionCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(definitions[i])
	}

	return builder.EndVector(definitionCount)
}
