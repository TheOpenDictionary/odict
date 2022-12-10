package odict

import (
	"encoding/xml"

	flatbuffers "github.com/google/flatbuffers/go"
)

type EntryRepresentable struct {
	Term        string                   `json:"term" xml:"term,attr"`
	SeeAlso     string                   `json:"see" xml:"see,attr,omitempty"`
	Etymologies []EtymologyRepresentable `json:"etymologies" xml:"ety"`
	XMLName     xml.Name                 `json:"-" xml:"entry"`
}

func (entry EntryRepresentable) Key() string {
	return entry.Term
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
		SeeAlso:     string(entry.See()),
		Etymologies: etymologies,
	}
}

func (entry *EntryRepresentable) AsBuffer(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	key := builder.CreateString(entry.Key())
	term := builder.CreateString(entry.Term)
	see := builder.CreateString(entry.SeeAlso)
	etymologies := entry.buildEtymologyVector(builder)

	EntryStart(builder)
	EntryAddKey(builder, key)
	EntryAddSee(builder, see)
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
