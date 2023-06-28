package types

import (
	"sort"

	flatbuffers "github.com/google/flatbuffers/go"
	"github.com/samber/lo"
)

type EtymologyRepresentable struct {
	ID          string                                  `json:"id,omitempty" xml:"id,attr,omitempty"`
	Description string                                  `json:"description,omitempty" xml:"description,attr,omitempty"`
	Usages      KVMap[PartOfSpeech, UsageRepresentable] `json:"usages" xml:"usage"`
}

func (etymology *Etymology) AsRepresentable() EtymologyRepresentable {
	var usage Usage
	usages := make(map[PartOfSpeech]UsageRepresentable)

	for u := 0; u < etymology.UsagesLength(); u++ {
		etymology.Usages(&usage, u)
		representable := usage.AsRepresentable()
		usages[representable.POS] = representable
	}

	return EtymologyRepresentable{
		ID:          string(etymology.Id()),
		Description: string(etymology.Description()),
		Usages:      usages,
	}
}

func (ety *EtymologyRepresentable) AsBuffer(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	id := builder.CreateString(ety.ID)
	description := builder.CreateString(ety.Description)
	usages := ety.buildUsageVector(builder)

	EtymologyStart(builder)
	EtymologyAddId(builder, id)
	EtymologyAddDescription(builder, description)
	EtymologyAddUsages(builder, usages)

	return EtymologyEnd(builder)
}

func (ety *EtymologyRepresentable) buildUsageVector(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	usages := ety.Usages
	usageCount := len(usages)
	keys := make([]string, 0, usageCount)

	for key := range usages {
		keys = append(keys, key.Tag)
	}

	sort.Strings(keys)

	usageBuffers := lo.Map(keys, func(key string, _ int) flatbuffers.UOffsetT {
		usage := usages[strToPartOfSpeech(key)]
		return usage.AsBuffer(builder)
	})

	EtymologyStartUsagesVector(builder, usageCount)

	for i := usageCount - 1; i >= 0; i-- {
		builder.PrependUOffsetT(usageBuffers[i])
	}

	return builder.EndVector(usageCount)
}
