package sql

import (
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
	"github.com/bokwoon95/sq"
)

type GROUPS struct {
	sq.TableStruct
	ID          sq.UUIDField `ddl:"primarykey"`
	DESCRIPTION sq.StringField
	SENSE_ID    sq.UUIDField `ddl:"notnull references=senses.id"`
}

// Insert groups w/ relation to current sense
func insertGroups(builder *SQLBuilder, senseID string, groups []types.GroupRepresentable) {
	for _, group := range groups {
		grp := sq.New[GROUPS]("")
		id := utils.CreateUUID()

		builder.AddCommand(
			sq.
				InsertInto(grp).
				Columns(grp.ID, grp.DESCRIPTION, grp.SENSE_ID).
				Values(
					sq.Literal(id),
					sq.Literal(group.Description.String()),
					sq.Literal(senseID),
				),
		)

		insertDefinitions(builder, "", group.ID, group.Definitions)
	}
}
