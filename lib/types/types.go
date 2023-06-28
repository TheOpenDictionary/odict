package types

import (
	"encoding/xml"
	"io"

	flatbuffers "github.com/google/flatbuffers/go"
	"github.com/imdario/mergo"
)

func Serialize(b Serializable) []byte {
	builder := flatbuffers.NewBuilder(0)
	buffer := b.AsBuffer(builder)

	builder.Finish(buffer)

	return builder.FinishedBytes()
}

type Keyable[T comparable] interface {
	Key() T
}

type Serializable interface {
	AsBuffer(builder *flatbuffers.Builder) flatbuffers.UOffsetT
}

type Representable interface {
	AsRepresentable() interface{}
}

type KVMap[K comparable, V Keyable[K]] map[K]V

func (m KVMap[K, V]) MarshalXML(e *xml.Encoder, start xml.StartElement) error {
	for key := range m {
		e.Encode(m[key])
	}
	return nil
}

func (m *KVMap[K, V]) UnmarshalXML(d *xml.Decoder, start xml.StartElement) error {

	var e V

	err := d.DecodeElement(&e, &start)

	if *m == nil {
		*m = KVMap[K, V]{}
	}

	if err != nil {
		return err
	}

	k := e.Key()

	if _, ok := (*m)[k]; ok {
		existing := (*m)[k]

		if err := mergo.Merge(&e, existing, mergo.WithAppendSlice); err != nil {
			return err
		}
	}

	(*m)[k] = e

	for {
		_, err := d.Token()

		if err != nil {
			if err == io.EOF {
				return nil
			}
			return err
		}
	}
}
