#[macro_export]
macro_rules! dao {
	(struct $name:ident {
		$($field:ident: $class:ty,)*
	}) => {
		extern crate sqlite3;
		use sqlite3::{
			ResultRow,
			ResultRowAccess,
		};
		#[derive(Debug)]
		struct $name {
		    $($field: $class,)*
		}

		impl Model for $name {
		    fn column_names() -> Vec<&'static str> {
		    	vec![$(stringify!($field)),*]
		    }

			fn new(row: &mut ResultRow) -> Self {
				$name {
					$($field: row.get(stringify!($field)),)*
				}
			}
		}
	}
}