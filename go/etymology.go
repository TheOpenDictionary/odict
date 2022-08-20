package odict

import (
	"sort"

	flatbuffers "github.com/google/flatbuffers/go"
)

type EtymologyRepresentable struct {
	ID          string                              `json:"id" xml:"id,attr"`
	Description string                              `json:"description,omitempty" xml:"description,attr,omitempty"`
	Usages      map[PartOfSpeech]UsageRepresentable `json:"usages" xml:"usage"`
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
	EtymologyStart(builder)
	EtymologyAddId(builder, builder.CreateString(ety.ID))
	EtymologyAddDescription(builder, builder.CreateString(ety.Description))
	EtymologyAddUsages(builder, ety.buildUsageVector(builder))

	return EtymologyEnd(builder)
}

func (ety *EtymologyRepresentable) buildUsageVector(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	usages := ety.Usages
	usageCount := len(usages)
	keys := make([]string, 0, usageCount)

	for key := range usages {
		keys = append(keys, string(key))
	}

	sort.Strings(keys)

	EtymologyStartUsagesVector(builder, usageCount)

	for i := usageCount - 1; i >= 0; i-- {
		usage := usages[PartOfSpeech(keys[i])]
		builder.PrependUOffsetT(usage.AsBuffer(builder))
	}

	return builder.EndVector(usageCount)
}
