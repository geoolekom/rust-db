extern crate sqlite3;

use sqlite3::{
	DatabaseConnection,
    Query,
    ResultRow,
    ResultRowAccess,
    SqliteResult,
    StatementUpdate,
};

use sqlite3::access::{
	ByFilename,
};

use sqlite3::access::flags::{
	OPEN_READWRITE,
};

#[derive(Debug)]
struct User {
    id: i32,
    username: String,
    password: String,
    salt: String,
}

fn main() {

	let user = User {
		id: 1,
		username: "geoolekom".to_string(),
		password: "qwerty".to_string(),
		salt: "1234".to_string(),
	};

	let ref connection = connect("storage/storage.sqlite3");
	update(connection, user);
	let users = get_all(connection).unwrap();

	println!("{:?}", users);
}

fn connect(filename: &str) -> DatabaseConnection {
	let file_name_access = ByFilename {
		filename: filename,
		flags: OPEN_READWRITE,
	};
	let connection = DatabaseConnection::new(file_name_access).unwrap();
	connection
}

fn init(connection: &DatabaseConnection) {
	let mut stmt = connection.prepare(
		"create table Users (
		id serial primary key,
		username varchar,
		password varchar,
		salt varchar);"
	).unwrap();
	println!("{:?}", stmt.update(&[]).unwrap());
}

fn update(connection: &DatabaseConnection, user: User) {
	let mut tx = connection.prepare(
		"insert into Users (username, password, salt)
		values ($1, $2, $3)"
	).unwrap();
	let changes = tx.update(&[&user.username, &user.password, &user.salt]).unwrap();
	assert_eq!(changes, 1);
}

fn get_all(connection: &DatabaseConnection) -> SqliteResult<Vec<User>> {
    let mut stmt = connection.prepare("select id, username, password, salt from Users").unwrap();

    let to_user = |row: &mut ResultRow| Ok(
        User {
            id: row.get("id"),
            username: row.get("username"),
            password: row.get("password"),
            salt: row.get("salt")
        });
    let users = stmt.query(&[], to_user).unwrap();
    users.collect()
}


