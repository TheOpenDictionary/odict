package dump

import (
	"bytes"
	"embed"
	"fmt"

	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/bokwoon95/sq"
	"github.com/bokwoon95/sqddl/ddl"
)

//go:embed sql.go
var fsys embed.FS

var sqlDictionaryId int = 1
var sqlEntryId int = 1
var sqlEtymologyId int = 1
var sqlUsageId int = 1
var sqlGroupId int = 1
var sqlDefinitionId int = 1
var sqlExampleId int = 1

func sql(dict types.DictionaryRepresentable, sqlDialect SqlDialect) string {
	var sqlCmds string

	sqlCmds += sqlCreate(sqlDialect)
	sqlCmds += sqlInsert(sqlDialect, dict)

	return sqlCmds
}

func sqlCreate(sqlDialect SqlDialect) string {
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

func sqlInsert(sqlDialect SqlDialect, dict types.DictionaryRepresentable) string {
	var sqlCmds string

	// Entry point that will capture every insert statement
	// Dictionary, entries, etymologies, usages, groups, definitions, examples
	sqlCmds += sqlInsertDictionary(sqlDialect, dict)
	return sqlCmds
}

func sqlInsertDictionary(sqlDialect SqlDialect, dict types.DictionaryRepresentable) string {
	var sqlCmds string

	// Insert dictionary w/ PK of 1
	d := sq.New[DICTIONARIES]("")
	insertQuery := sq.
		InsertInto(d).
		Columns(d.NAME, d.ID).
		Values(sq.Literal(dict.Name), sq.Literal(sqlDictionaryId))

	dict_query, _, err := sq.ToSQL(sqlDialect, insertQuery, nil)
	if err != nil {
		fmt.Println(err)
	}

	sqlCmds += dict_query + ";\n"
	sqlCmds += sqlInsertEntries(sqlDialect, dict.Entries)

	return sqlCmds
}

func sqlInsertEntries(sqlDialect SqlDialect, entries types.KVMap[string, types.EntryRepresentable]) string {
	var sqlCmds string

	// Insert entries w/ relation to dictionary with PK of 1
	for _, entry := range entries {
		e := sq.New[ENTRIES]("")
		insertQuery := sq.
			InsertInto(e).
			Columns(e.ID, e.TERM, e.DICTIONARY_ID).
			Values(sq.Literal(sqlEntryId), sq.Literal(entry.Term), sq.Literal(1))
		e_query, _, err := sq.ToSQL(sqlDialect, insertQuery, nil)
		if err != nil {
			fmt.Println(err)
			continue
		}

		sqlCmds += e_query + ";\n"
		sqlCmds += sqlInsertEtymologies(sqlDialect, entry.Etymologies)

		sqlEntryId++
	}

	return sqlCmds
}

func sqlInsertEtymologies(sqlDialect SqlDialect, etymologies []types.EtymologyRepresentable) string {
	var sqlCmds string

	// Insert etymologies w/ relation to current entry
	for _, etymology := range etymologies {
		ety := sq.New[ETYMOLOGIES]("")
		insertQuery := sq.
			InsertInto(ety).
			Columns(ety.ID, ety.DESCRIPTION, ety.ENTRY_ID).
			Values(sq.Literal(sqlEtymologyId), sq.Literal(etymology.Description), sq.Literal(sqlEntryId))
		ety_query, _, err := sq.ToSQL(sqlDialect, insertQuery, nil)
		if err != nil {
			fmt.Println(err)
			continue
		}

		sqlCmds += ety_query + ";\n"
		sqlCmds += sqlInsertUsages(sqlDialect, etymology.Usages)

		sqlEtymologyId++
	}

	return sqlCmds
}

func sqlInsertUsages(sqlDialect SqlDialect, usages types.KVMap[types.PartOfSpeech, types.UsageRepresentable]) string {
	var sqlCmds string

	// Insert usages w/ relation to current ety
	for _, usage := range usages {
		usg := sq.New[USAGES]("")
		insertQuery := sq.
			InsertInto(usg).
			Columns(usg.ID, usg.ETYMOLOGY_ID).
			Values(sq.Literal(sqlUsageId), sq.Literal(sqlEtymologyId))
		usg_query, _, err := sq.ToSQL(sqlDialect, insertQuery, nil)
		if err != nil {
			fmt.Println(err)
			continue
		}

		sqlCmds += usg_query + ";\n"
		sqlCmds += sqlInsertGroups(sqlDialect, usage.Groups)

		sqlUsageId++
	}

	return sqlCmds
}

func sqlInsertGroups(sqlDialect SqlDialect, groups []types.GroupRepresentable) string {
	var sqlCmds string

	// Insert groups w/ relation to current usage
	for _, group := range groups {
		grp := sq.New[GROUPS]("")
		insertQuery := sq.
			InsertInto(grp).
			Columns(grp.ID, grp.DESCRIPTION, grp.USAGE_ID).
			Values(sq.Literal(sqlGroupId), sq.Literal(group.Description), sq.Literal(sqlUsageId))
		grp_query, _, err := sq.ToSQL(sqlDialect, insertQuery, nil)
		if err != nil {
			fmt.Println(err)
			continue
		}

		sqlCmds += grp_query + ";\n"
		sqlCmds += sqlInsertDefinitions(sqlDialect, group.Definitions)

		sqlGroupId++
	}
	return sqlCmds
}

func sqlInsertDefinitions(sqlDialect SqlDialect, definitions []types.DefinitionRepresentable) string {
	var sqlCmds string

	// Insert definitions w/ relation to current usage/group
	for _, definition := range definitions {
		def := sq.New[DEFINITIONS]("")
		insertQuery := sq.
			InsertInto(def).
			Columns(def.ID, def.TEXT, def.USAGE_ID, def.GROUP_ID).
			Values(sq.Literal(sqlDefinitionId), sq.Literal(definition.Value), sq.Literal(sqlUsageId), sq.Literal(sqlGroupId))
		def_query, _, err := sq.ToSQL(sqlDialect, insertQuery, nil)
		if err != nil {
			fmt.Println(err)
			continue
		}

		sqlCmds += def_query + ";\n"
		sqlCmds += sqlInsertExamples(sqlDialect, definition.Examples)

		sqlDefinitionId++
	}
	return sqlCmds
}

func sqlInsertExamples(sqlDialect SqlDialect, examples []string) string {
	var sqlCmds string

	// Insert examples w/ relation to current definition
	for _, example := range examples {
		ex := sq.New[EXAMPLES]("")
		insertQuery := sq.
			InsertInto(ex).
			Columns(ex.ID, ex.TEXT, ex.DEFINITION_ID).
			Values(sq.Literal(sqlExampleId), sq.Literal(example), sq.Literal(sqlDefinitionId))
		ex_query, _, err := sq.ToSQL(sqlDialect, insertQuery, nil)
		if err != nil {
			fmt.Println(err)
			continue
		}

		sqlCmds += ex_query + ";\n"

		sqlExampleId++
	}

	return sqlCmds
}
