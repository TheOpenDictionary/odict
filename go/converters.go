package odict

import (
	"bytes"
	"embed"
	"encoding/json"
	"encoding/xml"
	"fmt"

	"github.com/bokwoon95/sq"
	"github.com/bokwoon95/sqddl/ddl"
	flatbuffers "github.com/google/flatbuffers/go"
)

//go:embed sql.go
var fsys embed.FS

func strToPartOfSpeech(str string) PartOfSpeech {
	if val, ok := posTagPartOfSpeechMap[str]; ok {
		return val
	}

	return Unknown
}

func serialize(b Serializable) []byte {
	builder := flatbuffers.NewBuilder(0)
	buffer := b.AsBuffer(builder)

	builder.Finish(buffer)

	return builder.FinishedBytes()
}

func JSON(any interface{}) string {
	b, err := json.MarshalIndent(&any, "", " ")

	Check(err)

	return string(b)
}

func XML(any interface{}) string {
	str, err := xml.MarshalIndent(&any, "", " ")

	Check(err)

	return string(str)
}

func sql(dict DictionaryRepresentable, sqlDialect string) string {
	var sqlCmds string

	sqlCmds += sqlCreate(sqlDialect)
	sqlCmds += sqlInsert(sqlDialect, dict)

	return sqlCmds
}

func sqlCreate(sqlDialect string) string {
	var buf bytes.Buffer

	// Generate SQL create statements and constraints
	generateCmd := &ddl.GenerateCmd{
		Dialect:   sqlDialect,
		DirFS:     fsys,
		Filenames: []string{"sql.go"},
		DryRun:    true,
		Stdout:    &buf,
	}
	err := generateCmd.Run()
	if err != nil {
		fmt.Println(err)
	}
	return buf.String()
}

func sqlInsert(sqlDialect string, dict DictionaryRepresentable) string {
	var sqlCmds string

	// Entry point that will capture every insert statement
	// Dictionary, entries, etymologies, usages, groups, definitions, examples
	sqlCmds += sqlInsertDictionary(sqlDialect, dict)
	return sqlCmds
}

func sqlInsertDictionary(sqlDialect string, dict DictionaryRepresentable) string {
	var sqlCmds string
	dictId := 1

	// Insert dictionary w/ PK of 1
	d := sq.New[DICTIONARIES]("")
	insertQuery := sq.
		InsertInto(d).
		Columns(d.NAME, d.ID).
		Values(sq.Literal(dict.Name), sq.Literal(dictId))
	dict_query, _, err := sq.ToSQL(sqlDialect, insertQuery, nil)
	if err != nil {
		fmt.Println(err)
	}

	sqlCmds += dict_query + ";\n"
	sqlCmds += sqlInsertEntries(sqlDialect, dict.Entries, dictId)

	return sqlCmds
}

func sqlInsertEntries(sqlDialect string, entries KVMap[string, EntryRepresentable], dictId int) string {
	var sqlCmds string
	entryId := 1

	// Insert entries w/ relation to dictionary with PK of 1
	for _, entry := range entries {
		e := sq.New[ENTRIES]("")
		insertQuery := sq.
			InsertInto(e).
			Columns(e.ID, e.TERM, e.DICTIONARY_ID).
			Values(sq.Literal(entryId), sq.Literal(entry.Term), sq.Literal(1))
		e_query, _, err := sq.ToSQL(sqlDialect, insertQuery, nil)
		if err != nil {
			fmt.Println(err)
			continue
		}

		sqlCmds += e_query + ";\n"
		sqlCmds += sqlInsertEtymologies(sqlCmds, entry.Etymologies, entryId)

		entryId++
	}

	return sqlCmds
}

func sqlInsertEtymologies(sqlDialect string, etymologies []EtymologyRepresentable, entryId int) string {
	var sqlCmds string
	etyId := 1

	// Insert etymologies w/ relation to current entry
	for _, etymology := range etymologies {
		ety := sq.New[ETYMOLOGIES]("")
		insertQuery := sq.
			InsertInto(ety).
			Columns(ety.ID, ety.DESCRIPTION, ety.ENTRY_ID).
			Values(sq.Literal(etyId), sq.Literal(etymology.Description), sq.Literal(entryId))
		ety_query, _, err := sq.ToSQL(sqlDialect, insertQuery, nil)
		if err != nil {
			fmt.Println(err)
			continue
		}

		sqlCmds += ety_query + ";\n"
		sqlInsertUsages(sqlDialect, etymology.Usages, etyId)

		etyId++

	}

	return sqlCmds
}

func sqlInsertUsages(sqlDialect string, usages KVMap[PartOfSpeech, UsageRepresentable], etyId int) string {
	var sqlCmds string
	usageId := 1

	// Insert usages w/ relation to current ety
	for _, usage := range usages {
		usg := sq.New[USAGES]("")
		insertQuery := sq.
			InsertInto(usg).
			Columns(usg.ID, usg.ETYMOLOGY_ID).
			Values(sq.Literal(usageId), sq.Literal(etyId))
		usg_query, _, err := sq.ToSQL(sqlDialect, insertQuery, nil)
		if err != nil {
			fmt.Println(err)
			continue
		}

		sqlCmds += usg_query + ";\n"
		sqlCmds += sqlInsertGroups(sqlDialect, usage.Groups, usageId)

		usageId++
	}

	return sqlCmds
}

func sqlInsertGroups(sqlDialect string, groups []GroupRepresentable, usageId int) string {
	var sqlCmds string
	groupId := 1

	// Insert groups w/ relation to current usage
	for _, group := range groups {
		grp := sq.New[GROUPS]("")
		insertQuery := sq.
			InsertInto(grp).
			Columns(grp.ID, grp.DESCRIPTION, grp.USAGE_ID).
			Values(sq.Literal(groupId), sq.Literal(group.Description), sq.Literal(usageId))
		grp_query, _, err := sq.ToSQL(sqlDialect, insertQuery, nil)
		if err != nil {
			fmt.Println(err)
			continue
		}

		sqlCmds += grp_query + ";\n"
		sqlCmds += sqlInsertDefinitions(sqlDialect, group.Definitions, usageId, groupId)

		groupId++
	}
	return sqlCmds
}

func sqlInsertDefinitions(sqlDialect string, definitions []DefinitionRepresentable, usageId int, groupId int) string {
	var sqlCmds string
	defId := 1

	// Insert definitions w/ relation to current usage/group
	for _, definition := range definitions {
		def := sq.New[DEFINITIONS]("")
		insertQuery := sq.
			InsertInto(def).
			Columns(def.ID, def.TEXT, def.USAGE_ID, def.GROUP_ID).
			Values(sq.Literal(defId), sq.Literal(definition.Value), sq.Literal(usageId), sq.Literal(groupId))
		def_query, _, err := sq.ToSQL(sqlDialect, insertQuery, nil)
		if err != nil {
			fmt.Println(err)
			continue
		}

		sqlCmds += def_query + ";\n"
		sqlInsertExamples(sqlDialect, definition.Examples, defId)

		defId++
	}
	return sqlCmds
}

func sqlInsertExamples(sqlDialect string, examples []string, defId int) string {
	var sqlCmds string
	exId := 1

	// Insert examples w/ relation to current definition
	for _, example := range examples {
		ex := sq.New[EXAMPLES]("")
		insertQuery := sq.
			InsertInto(ex).
			Columns(ex.ID, ex.TEXT, ex.DEFINITION_ID).
			Values(sq.Literal(exId), sq.Literal(example), sq.Literal(defId))
		ex_query, _, err := sq.ToSQL(sqlDialect, insertQuery, nil)
		if err != nil {
			fmt.Println(err)
			continue
		}

		sqlCmds += ex_query + ";\n"

		exId++
	}

	return sqlCmds
}
