package types

import (
	"sort"

	flatbuffers "github.com/google/flatbuffers/go"
	"github.com/samber/lo"
)

type EtymologyRepresentable struct {
	ID          string                                  `json:"id,omitempty" xml:"id,attr,omitempty"`
	Description string                                  `json:"description,omitempty" xml:"description,attr,omitempty"`
	Senses      KVMap[PartOfSpeech, SenseRepresentable] `json:"senses" xml:"sense"`
}

func (etymology *Etymology) AsRepresentable() EtymologyRepresentable {
	var sense Sense
	senses := make(map[PartOfSpeech]SenseRepresentable)

	for u := 0; u < etymology.SensesLength(); u++ {
		etymology.Senses(&sense, u)
		representable := sense.AsRepresentable()
		senses[representable.POS] = representable
	}

	return EtymologyRepresentable{
		ID:          string(etymology.Id()),
		Description: string(etymology.Description()),
		Senses:      senses,
	}
}

func (ety *EtymologyRepresentable) AsBuffer(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	id := builder.CreateString(ety.ID)
	description := builder.CreateString(ety.Description)
	senses := ety.buildSenseVector(builder)

	EtymologyStart(builder)
	EtymologyAddId(builder, id)
	EtymologyAddDescription(builder, description)
	EtymologyAddSenses(builder, senses)

	return EtymologyEnd(builder)
}

func (ety *EtymologyRepresentable) buildSenseVector(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	senses := ety.Senses
	senseCount := len(senses)
	keys := make([]string, 0, senseCount)

	for key := range senses {
		keys = append(keys, key.Tag)
	}

	sort.Strings(keys)

	senseBuffers := lo.Map(keys, func(key string, _ int) flatbuffers.UOffsetT {
		sense := senses[strToPartOfSpeech(key)]
		return sense.AsBuffer(builder)
	})

	EtymologyStartSensesVector(builder, senseCount)

	for i := senseCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(senseBuffers[i])
	}

	return builder.EndVector(senseCount)
}
