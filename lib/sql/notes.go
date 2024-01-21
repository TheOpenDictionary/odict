package sql

import (
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
	"github.com/bokwoon95/sq"
)

type NOTES struct {
	sq.TableStruct
	ID            sq.UUIDField `ddl:"primarykey"`
	VALUE         sq.StringField
	DEFINITION_ID sq.UUIDField `ddl:"notnull references=definitions.id"`
}

// Insert notes w/ relation to current definition
func insertNotes(builder *SQLBuilder, definitionID string, notes []types.NoteRepresentable) {
	for _, note := range notes {
		nt := sq.New[NOTES]("")
		id := utils.CreateUUID()
		value := note.Value.String()

		builder.AddCommand(
			sq.
				InsertInto(nt).
				Columns(nt.ID, nt.VALUE, nt.DEFINITION_ID).
				Values(
					sq.Literal(id),
					sq.Literal(value),
					sq.Literal(definitionID),
				),
		)

		insertExamples(builder, "", id, note.Examples)
	}
}
