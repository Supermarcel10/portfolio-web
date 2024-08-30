use chrono::{DateTime, Utc};

pub trait BaseEntity {
	fn id(&self) -> &str;
	fn insert_date_time(&self) -> DateTime<Utc>;
	fn update_date_time(&self) -> Option<DateTime<Utc>>;
	fn created_by(&self) -> &str;
	fn updated_by(&self) -> Option<&str>;
	fn is_active(&self) -> bool;
}

pub struct Base {
	pub id: String,
	pub insert_date_time: DateTime<Utc>,
	pub update_date_time: Option<DateTime<Utc>>,
	pub created_by: String,
	pub updated_by: Option<String>,
	pub is_active: bool,
}

macro_rules! impl_base {
	($type:ty) => {
		impl BaseEntity for $type {
			fn id(&self) -> &str {
				todo!()
			}

			fn insert_date_time(&self) -> DateTime<Utc> {
				todo!()
			}

			fn update_date_time(&self) -> Option<DateTime<Utc>> {
				todo!()
			}

			fn created_by(&self) -> &str {
				todo!()
			}

			fn updated_by(&self) -> Option<&str> {
				todo!()
			}

			fn is_active(&self) -> bool {
				todo!()
			}
		}
	}
}

pub (crate) use {
	impl_base
};
