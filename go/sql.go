package odict

import "github.com/bokwoon95/sq"

type LANGUAGE struct {
	sq.TableStruct
	ID   sq.NumberField `ddl:"type=BIGINT primarykey auto_increment autoincrement identity"`
	CODE sq.StringField `ddl:"notnull unique"`
	FLAG sq.StringField `ddl:"notnull unique"`
}

type DICTIONARY struct {
	sq.TableStruct
	ID   sq.NumberField `ddl:"type=BIGINT primarykey auto_increment autoincrement identity"`
	NAME sq.StringField `ddl:"notnull"`
}

type ENTRY struct {
	sq.TableStruct
	ID            sq.NumberField `ddl:"type=BIGINT primarykey auto_increment autoincrement identity"`
	CREATED_AT    sq.TimeField   `ddl:"default=CURRENT_TIMESTAMP"`
	UPDATED_AT    sq.TimeField
	TERM          sq.StringField `ddl:"notnull"`
	DICTIONARY_ID sq.NumberField `ddl:"type=BIGINT references=dictionary.id"`
}

type ETYMOLOGY struct {
	sq.TableStruct
	ID          sq.StringField `ddl:"primarykey"` // cuid() generation is handled by prisma, not the database.
	DESCRIPTION sq.StringField
	ENTRY_ID    sq.NumberField `ddl:"type=BIGINT notnull references=entry.id"`
}

type USAGE struct {
	sq.TableStruct
	ID           sq.NumberField `ddl:"type=BIGINT primarykey auto_increment autoincrement identity"`
	ETYMOLOGY_ID sq.StringField `ddl:"notnull references=etymology.id"`
}

type GROUP struct {
	sq.TableStruct
	ID          sq.NumberField `ddl:"type=BIGINT primarykey auto_increment autoincrement identity"`
	DESCRIPTION sq.StringField
	USAGE_ID    sq.NumberField `ddl:"type=BIGINT notnull references=usage.id"`
}

type DEFINITION struct {
	sq.TableStruct
	ID       sq.NumberField `ddl:"type=BIGINT primarykey auto_increment autoincrement identity"`
	TEXT     sq.StringField `ddl:"notnull"`
	USAGE_ID sq.NumberField `ddl:"type=BIGINT references=usage.id"`
	GROUP_ID sq.NumberField `ddl:"type=BIGINT references=group.id"`
}

type EXAMPLE struct {
	sq.TableStruct
	ID            sq.NumberField `ddl:"type=BIGINT primarykey auto_increment autoincrement identity"`
	TEXT          sq.StringField `ddl:"notnull"`
	DEFINITION_ID sq.NumberField `ddl:"type=BIGINT notnull references=definition.id"`
}
