#[macro_use(dao)]
extern crate rustdb;

use rustdb::storage::{Model, Storage};

dao! {
	struct User {
	    id: i32,
	    username: String,
	    password: String,
	    salt: String,
	}
}

fn main() {

	//let columns = ["id serial primary key", "username varchar", "password varchar", "salt varchar"];
	let columns = User::column_names();

	let users = Storage::open(
		"/home/geoolekom/rust/rust-db/storage/storage.sqlite3", 
		"Users", 
		&columns[..]);

	let users_got = users.get::<User>("username='geoolekom'").unwrap();

	println!("{:?}", users_got);
}

