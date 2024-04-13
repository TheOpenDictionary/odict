use sea_query::{
    InsertStatement, MysqlQueryBuilder, PostgresQueryBuilder, SqliteQueryBuilder,
    TableCreateStatement,
};

pub struct SQLBuilder {
    dialect: SQLDialect,
    schema: Vec<TableCreateStatement>,
    queries: Vec<InsertStatement>,
}

impl SQLBuilder {
    pub fn new(dialect: SQLDialect) -> Self {
        SQLBuilder {
            dialect,
            schema: Vec::new(),
            queries: Vec::new(),
        }
    }

    pub fn add_table(&mut self, cmd: TableCreateStatement) {
        self.schema.push(cmd);
    }

    pub fn add_insert(&mut self, cmd: InsertStatement) {
        self.queries.push(cmd);
    }

    pub fn build(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut tables = self
            .schema
            .iter()
            .map(|cmd| match self.dialect {
                SQLDialect::Postgres => cmd.to_string(PostgresQueryBuilder::default()),
                SQLDialect::MySQL => cmd.to_string(MysqlQueryBuilder::default()),
                SQLDialect::SQLite => cmd.to_string(SqliteQueryBuilder::default()),
            })
            .collect::<Vec<String>>();

        let mut queries = self
            .queries
            .iter()
            .map(|cmd| match self.dialect {
                SQLDialect::Postgres => cmd.to_string(PostgresQueryBuilder::default()),
                SQLDialect::MySQL => cmd.to_string(MysqlQueryBuilder::default()),
                SQLDialect::SQLite => cmd.to_string(SqliteQueryBuilder::default()),
            })
            .collect::<Vec<String>>();

        tables.append(&mut queries);

        Ok(tables.join(";\n"))
    }
}

pub enum SQLDialect {
    Postgres,
    MySQL,
    SQLite,
}
