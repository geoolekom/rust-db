extern crate sqlite3;

use self::sqlite3::{
	DatabaseConnection,
	Query,
	ResultRow,
	ResultRowAccess,
	SqliteResult,
	StatementUpdate,
	PreparedStatement,
};

use self::sqlite3::access::{
	ByFilename,
};

use self::sqlite3::access::flags::{
	OPEN_READWRITE,
};

pub trait Model {
	fn new(row: &mut ResultRow) -> Self;

	fn column_names() -> Vec<&'static str>;
}

pub struct Storage<'a> {

	connection: DatabaseConnection,
	table: &'a str,
	columns: &'a [&'a str],

}

impl<'a> Storage<'a> {

	pub fn new(filename: &'a str, table: &'a str, columns: &'a [&'a str]) -> Storage<'a> {
		let mut connection = Storage::connect(filename);
		let mut columns_as_sql = "(".to_string();
		columns_as_sql += &columns
			.iter()
			.skip(1)
			.fold(columns[0].to_string(), |sql, column| sql + ", " + column);
		columns_as_sql += ")";

		let sql = "create table ".to_string() + table + " " + &columns_as_sql + ";";

		match connection.exec(&sql) {
			Ok(_) => println!("Table successfully created."),
			Err(err) => panic!("Table creation: {}", err),
		};

		Storage {
			connection: connection,
			table: table,
			columns: columns,
		}

	}

	pub fn open(filename: &'a str, table: &'a str, columns: &'a [&'a str]) -> Storage<'a> {
		let mut connection = Storage::connect(filename);
		let mut columns_as_sql = "".to_string();
		columns_as_sql += &columns
			.iter()
			.skip(1)
			.fold(columns[0].to_string(), |sql, column| sql + ", " + column);

		let sql = "select ".to_string() + &columns_as_sql + " from " + table + ";";

		match connection.exec(&sql) {
			Ok(_) => println!("Successfully connected."),
			Err(err) => panic!("Table with such signature doesn't exist: {}", err),
		};

		Storage {
			connection: connection,
			table: table,
			columns: columns,
		}
	}

	pub fn get_all<T: Model>(&self) -> SqliteResult<Vec<T>> {
		let sql = "select * from ".to_string() + self.table + ";";
		let mut stmt = self.prepare_statement(&sql);

		Storage::process_statement::<T>(&mut stmt)
	}

	pub fn get<T: Model>(&self, query: &'a str) -> SqliteResult<Vec<T>> {
		let sql = "select * from ".to_string() + self.table + " where " + query + ";";
		let mut stmt = self.prepare_statement(&sql);

		Storage::process_statement::<T>(&mut stmt)
	}

	pub fn connect(filename: &'a str) -> DatabaseConnection {
		let filename_access = ByFilename {
			filename: filename,
			flags: OPEN_READWRITE,
		};

		match DatabaseConnection::new(filename_access) {
		    Ok(conn) => conn,
		    Err(err) => panic!("Connection to DB: {}", err),
		}
	}

	pub fn prepare_statement(&self, sql: &'a str) -> PreparedStatement {
		match self.connection.prepare(&sql) {
			Ok(stmt) => stmt,
			Err(err) => panic!("Creating a statement: {}", err),
		}
	}

	pub fn process_statement<T: Model>(stmt: &'a mut PreparedStatement) -> SqliteResult<Vec<T>> {
		let to_model = |row: &mut ResultRow| Ok(T::new(row));

		let models = match stmt.query(&[], to_model) {
			Ok(models) => models.collect(),
			Err(err) => panic!("Query execution: {}", err),
		};
		models
	}

}