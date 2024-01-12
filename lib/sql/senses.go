package sql

import (
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
	"github.com/bokwoon95/sq"
)

type SENSES struct {
	sq.TableStruct
	ID           sq.UUIDField `ddl:"primarykey"`
	ETYMOLOGY_ID sq.UUIDField `ddl:"notnull references=etymologies.id"`
}

// Insert senses w/ relation to current ety
func insertSenses(builder *SQLBuilder, etymologyID string, senses types.KVMap[types.PartOfSpeech, types.SenseRepresentable]) {
	for _, sense := range senses {
		usg := sq.New[SENSES]("")
		id := utils.CreateUUID()

		builder.AddCommand(
			sq.
				InsertInto(usg).
				Columns(usg.ID, usg.ETYMOLOGY_ID).
				Values(sq.Literal(id), sq.Literal(etymologyID)),
		)

		insertGroups(builder, id, sense.Groups)
		insertDefinitions(builder, id, "", sense.Definitions)
	}
}
