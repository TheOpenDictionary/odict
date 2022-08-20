package odict

import (
	"encoding/xml"

	flatbuffers "github.com/google/flatbuffers/go"
)

type EntryRepresentable struct {
	Key         string                   `json:"-" xml:"-"`
	Term        string                   `json:"term" xml:"term,attr"`
	Etymologies []EtymologyRepresentable `json:"etymologies" xml:"ety"`
	XMLName     xml.Name                 `json:"-" xml:"entry"`
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
	EntryStart(builder)
	EntryAddKey(builder, builder.CreateString(entry.Key))
	EntryAddTerm(builder, builder.CreateString(entry.Term))
	EntryAddEtymologies(builder, entry.buildEtymologyVector(builder))

	return EntryEnd(builder)
}

func (entry *EntryRepresentable) buildEtymologyVector(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	etymologies := entry.Etymologies
	etymologiesCount := len(etymologies)

	EntryStartEtymologiesVector(builder, etymologiesCount)

	for i := etymologiesCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(etymologies[i].AsBuffer(builder))
	}

	return builder.EndVector(etymologiesCount)
}
