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
	let field_names = User::field_names();
	let field_types = User::field_types();

	let users = Storage::new(
		"/home/geoolekom/rust/rust-db/storage/storage.sqlite3", 
		"Losers", 
		&field_names[..]);

	users.open();

	let users_got = users.get::<User>("username='geoolekom'").unwrap();

	println!("{:?}", users_got);
}

