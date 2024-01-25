package types

import (
	"sort"

	flatbuffers "github.com/google/flatbuffers/go"
	"github.com/samber/lo"
)

type Etymology struct {
	ID            string                     `json:"id,omitempty" xml:"id,attr,omitempty"`
	Pronunciation string                     `json:"pronunciation,omitempty" xml:"pronunciation,attr,omitempty"`
	Description   MDString                   `json:"description,omitempty" xml:"description,attr,omitempty"`
	Senses        KVMap[PartOfSpeech, Sense] `json:"senses" xml:"sense"`
}

func (etymology *EtymologyBuffer) Struct() Etymology {
	var sense SenseBuffer
	senses := make(map[PartOfSpeech]Sense)

	for u := 0; u < etymology.SensesLength(); u++ {
		etymology.Senses(&sense, u)
		s := sense.Struct()
		senses[s.POS] = s
	}

	return Etymology{
		ID:            string(etymology.Id()),
		Pronunciation: string(etymology.Pronunciation()),
		Description:   MDString(etymology.Description()),
		Senses:        senses,
	}
}

func (ety *Etymology) Table(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	id := builder.CreateString(ety.ID)
	description := builder.CreateString(ety.Description.String())
	senses := ety.buildSenseVector(builder)
	pronunciation := builder.CreateString(ety.Pronunciation)

	EtymologyBufferStart(builder)
	EtymologyBufferAddId(builder, id)
	EtymologyBufferAddPronunciation(builder, pronunciation)
	EtymologyBufferAddDescription(builder, description)
	EtymologyBufferAddSenses(builder, senses)

	return EtymologyBufferEnd(builder)
}

func (ety *Etymology) buildSenseVector(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	senses := ety.Senses
	senseCount := len(senses)
	keys := make([]string, 0, senseCount)

	for key := range senses {
		keys = append(keys, key.Tag())
	}

	sort.Strings(keys)

	senseBuffers := lo.Map(keys, func(key string, _ int) flatbuffers.UOffsetT {
		sense := senses[strToPartOfSpeech(key)]
		return sense.Table(builder)
	})

	EtymologyBufferStartSensesVector(builder, senseCount)

	for i := senseCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(senseBuffers[i])
	}

	return builder.EndVector(senseCount)
}
