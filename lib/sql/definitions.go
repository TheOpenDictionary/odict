package sql

import (
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/bokwoon95/sq"
	"github.com/google/uuid"
)

type DEFINITIONS struct {
	sq.TableStruct
	ID       sq.UUIDField   `ddl:"primarykey"`
	VALUE    sq.StringField `ddl:"notnull"`
	SENSE_ID sq.UUIDField   `ddl:"references=senses.id"`
	GROUP_ID sq.UUIDField   `ddl:"references=groups.id"`
}

// Insert definitions w/ relation to current sense/group
func insertDefinitions(builder *SQLBuilder, senseID string, groupID string, definitions []types.DefinitionRepresentable) {
	for _, definition := range definitions {
		def := sq.New[DEFINITIONS]("")
		id := uuid.New().String()
		value := definition.Value.String()

		builder.AddCommand(
			sq.
				InsertInto(def).
				ColumnValues(func(col *sq.Column) {
					col.Set(def.ID, sq.Literal(id))
					col.Set(def.VALUE, sq.Literal(value))

					if len(senseID) > 0 {
						col.Set(def.SENSE_ID, sq.Literal(senseID))
					}

					if len(groupID) > 0 {
						col.Set(def.GROUP_ID, sq.Literal(groupID))
					}
				}),
		)

		insertNotes(builder, id, definition.Notes)
		insertExamples(builder, id, "", definition.Examples)
	}
}
