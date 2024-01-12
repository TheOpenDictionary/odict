package sql

import (
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
	"github.com/bokwoon95/sq"
)

type ETYMOLOGIES struct {
	sq.TableStruct
	ID          sq.UUIDField `ddl:"primarykey"`
	DESCRIPTION sq.StringField
	ENTRY_ID    sq.UUIDField `ddl:"notnull references=entries.id"`
}

// Insert etymologies w/ relation to current entry
func insertEtymologies(builder *SQLBuilder, entryID string, etymologies []types.EtymologyRepresentable) {
	for _, etymology := range etymologies {
		ety := sq.New[ETYMOLOGIES]("")
		id := utils.CreateUUID()

		builder.AddCommand(
			sq.
				InsertInto(ety).
				Columns(ety.ID, ety.DESCRIPTION, ety.ENTRY_ID).
				Values(sq.Literal(id), sq.Literal(etymology.Description.String()), sq.Literal(entryID)),
		)

		insertSenses(builder, id, etymology.Senses)
	}
}
