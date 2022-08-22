package odict

import (
	"encoding/xml"
	"strings"

	flatbuffers "github.com/google/flatbuffers/go"
)

type EntryRepresentable struct {
	Term        string                   `json:"term,omitempty" xml:"term,attr"`
	Etymologies []EtymologyRepresentable `json:"etymologies" xml:"ety"`
	XMLName     xml.Name                 `json:"-" xml:"entry"`
}

func (entry EntryRepresentable) Key() string {
	return strings.ToLower(entry.Term)
}

func (entry *Entry) AsRepresentable() EntryRepresentable {
	var ety Etymology
	var etymologies []EtymologyRepresentable

	for b := 0; b < entry.EtymologiesLength(); b++ {
		entry.Etymologies(&ety, b)
		representable := ety.AsRepresentable()
		etymologies = append(etymologies, representable)
	}

	return EntryRepresentable{
		Term:        string(entry.Term()),
		Etymologies: etymologies,
	}
}

func (entry *EntryRepresentable) AsBuffer(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	key := builder.CreateString(entry.Key())
	term := builder.CreateString(entry.Term)
	etymologies := entry.buildEtymologyVector(builder)

	EntryStart(builder)
	EntryAddKey(builder, key)
	EntryAddTerm(builder, term)
	EntryAddEtymologies(builder, etymologies)

	return EntryEnd(builder)
}

func (entry *EntryRepresentable) buildEtymologyVector(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	etymologies := Map(entry.Etymologies, func(ety EtymologyRepresentable) flatbuffers.UOffsetT {
		return ety.AsBuffer(builder)
	})

	etymologiesCount := len(etymologies)

	EntryStartEtymologiesVector(builder, etymologiesCount)

	for i := etymologiesCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(etymologies[i])
	}

	return builder.EndVector(etymologiesCount)
}
