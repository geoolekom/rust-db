extern crate sqlite3;

use self::sqlite3::{
	DatabaseConnection,
	Query,
	ResultRow,
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

	fn field_names() -> Vec<&'static str>;

	fn field_types() -> Vec<&'static str>;
}

pub struct Storage<'a> {

	connection: DatabaseConnection,
	table: &'a str,
	column_names: &'a [&'a str],

}

impl<'a> Storage<'a> {

	pub fn new(filename: &'a str, table: &'a str, field_names: &'a [&'a str]) -> Storage<'a> {
		let connection = Storage::connect(filename);

		Storage {
			connection: connection,
			table: table,
			column_names: field_names,
		}

	}

	pub fn init(&self, field_types:  &'a [&'a str]) {

		let column_types = field_types.iter().map(|&class| {
			match class {
				"i32" | "u32" => "integer",
				"i64" | "isize" | "u64" | "usize" => "bigint",
				"f32" | "f64" => "float",
				"bool" => "boolean",
				"String" | "&str" => "varchar",
				_ => "blob",
			}
		}).collect::<Vec<_>>();

		let mut columns_as_sql = "(".to_string();
		columns_as_sql += &(0..self.column_names.len())
			.skip(1)
			.fold(
				self.column_names[0].to_string() + " " + column_types[0] + " primary key autoincrement", 
				|sql, i| sql + ", " + self.column_names[i] + " " + column_types[i]
			);
		columns_as_sql += ")";

		let sql = "create table ".to_string() + self.table + " " + &columns_as_sql + ";";

		let mut stmt = Storage::prepare_statement(&self.connection, &sql);

		match stmt.update(&[]) {
			Ok(_) => println!("Table successfully created."),
			Err(err) => panic!("Table creation: {}", err),
		};

	}

	pub fn open(&self) {
		let mut columns_as_sql = "".to_string();
		columns_as_sql += &self.column_names
			.iter()
			.skip(1)
			.fold(self.column_names[0].to_string(), |sql, column| sql + ", " + column);

		let sql = "select ".to_string() + &columns_as_sql + " from " + self.table + " order by " + self.column_names[0] + " limit 1;";

		let mut stmt = Storage::prepare_statement(&self.connection, &sql);

		match stmt.query(&[], |row: &mut ResultRow| Ok(())) {
			Ok(_) => println!("Successfully connected."),
			Err(err) => panic!("Table with such signature doesn't exist: {}", err),
		};
	}

	pub fn get_all<T: Model>(&self) -> SqliteResult<Vec<T>> {
		let sql = "select * from ".to_string() + self.table + ";";
		let mut stmt = Storage::prepare_statement(&self.connection, &sql);

		Storage::process_query::<T>(&mut stmt)
	}

	pub fn get<T: Model>(&self, query: &'a str) -> SqliteResult<Vec<T>> {
		let sql = "select * from ".to_string() + self.table + " where " + query + ";";
		let mut stmt = Storage::prepare_statement(&self.connection, &sql);

		Storage::process_query::<T>(&mut stmt)
	}

	pub fn add<T: Model>(&self, model: T) {
		let sql = "insert into ".to_string() + self.table + " () values " + ";";
	}

	fn connect(filename: &'a str) -> DatabaseConnection {
		let filename_access = ByFilename {
			filename: filename,
			flags: OPEN_READWRITE,
		};

		match DatabaseConnection::new(filename_access) {
		    Ok(conn) => conn,
		    Err(err) => panic!("Connection to DB: {}", err),
		}
	}

	fn prepare_statement(connection: &'a DatabaseConnection, sql: &'a str) -> PreparedStatement {
		match connection.prepare(&sql) {
			Ok(stmt) => stmt,
			Err(err) => panic!("Creating a statement: {}", err),
		}
	}

	fn process_query<T: Model>(stmt: &'a mut PreparedStatement) -> SqliteResult<Vec<T>> {
		let to_model = |row: &mut ResultRow| Ok(T::new(row));

		let models = match stmt.query(&[], to_model) {
			Ok(models) => models.collect(),
			Err(err) => panic!("Query execution: {}", err),
		};
		models
	}

	fn process_update() {

	}

}