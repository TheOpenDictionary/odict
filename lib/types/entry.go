package types

import (
	"encoding/xml"

	flatbuffers "github.com/google/flatbuffers/go"
	"github.com/samber/lo"
)

type Entry struct {
	Term        string      `json:"term" xml:"term,attr"`
	SeeAlso     string      `json:"see,omitempty" xml:"see,attr,omitempty"`
	Etymologies []Etymology `json:"etymologies" xml:"ety,omitempty"`
	XMLName     xml.Name    `json:"-" xml:"entry"`
}

func (entry Entry) Key() string {
	return entry.Term
}

func (entry *Entry) Bytes() []byte {
	return getBytes(entry)
}

func (entry *EntryBuffer) Struct() Entry {
	var ety EtymologyBuffer
	var etymologies []Etymology

	for b := 0; b < entry.EtymologiesLength(); b++ {
		entry.Etymologies(&ety, b)
		s := ety.Struct()
		etymologies = append(etymologies, s)
	}

	return Entry{
		Term:        string(entry.Term()),
		SeeAlso:     string(entry.See()),
		Etymologies: etymologies,
	}
}

func (entry *Entry) Table(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	key := builder.CreateString(entry.Key())
	term := builder.CreateString(entry.Term)
	see := builder.CreateString(entry.SeeAlso)
	etymologies := entry.buildEtymologyVector(builder)

	EntryBufferStart(builder)
	EntryBufferAddKey(builder, key)
	EntryBufferAddSee(builder, see)
	EntryBufferAddTerm(builder, term)
	EntryBufferAddEtymologies(builder, etymologies)

	return EntryBufferEnd(builder)
}

func (entry *Entry) buildEtymologyVector(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	etymologies := lo.Map(entry.Etymologies, func(ety Etymology, _ int) flatbuffers.UOffsetT {
		return ety.Table(builder)
	})

	etymologiesCount := len(etymologies)

	EntryBufferStartEtymologiesVector(builder, etymologiesCount)

	for i := etymologiesCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(etymologies[i])
	}

	return builder.EndVector(etymologiesCount)
}
