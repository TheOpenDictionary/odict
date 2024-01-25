package sql

import (
	"sort"

	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
	"github.com/bokwoon95/sq"
)

type SENSES struct {
	sq.TableStruct
	ID           sq.UUIDField `ddl:"primarykey"`
	POS          sq.EnumField `ddl:"default='unknown'"`
	ETYMOLOGY_ID sq.UUIDField `ddl:"notnull references=etymologies.id"`
}

// Insert senses w/ relation to current ety
func insertSenses(builder *SQLBuilder, etymologyID string, senseMap types.KVMap[types.PartOfSpeech, types.Sense]) {
	senses := senseMap.Values()

	sort.Slice(senses, func(i, j int) bool {
		return senses[i].POS.Tag() < senses[j].POS.Tag()
	})

	for _, sense := range senses {
		usg := sq.New[SENSES]("")
		id := utils.CreateUUID()
		pos := sense.POS.Tag()

		builder.AddCommand(
			sq.
				InsertInto(usg).
				ColumnValues(func(col *sq.Column) {
					col.Set(usg.ID, sq.Literal(id))
					col.Set(usg.ETYMOLOGY_ID, sq.Literal(etymologyID))

					if len(pos) > 0 {
						col.Set(usg.POS, sq.Literal(pos))
					}
				}),
		)

		insertGroups(builder, id, sense.Groups)
		insertDefinitions(builder, id, "", sense.Definitions)
	}
}
