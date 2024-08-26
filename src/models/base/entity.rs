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

impl BaseEntity for Base {
	fn id(&self) -> &str {
		&self.id
	}

	fn insert_date_time(&self) -> DateTime<Utc> {
		self.insert_date_time
	}

	fn update_date_time(&self) -> Option<DateTime<Utc>> {
		self.update_date_time
	}

	fn created_by(&self) -> &str {
		&self.created_by
	}

	fn updated_by(&self) -> Option<&str> {
		self.updated_by.as_deref()
	}

	fn is_active(&self) -> bool {
		self.is_active
	}
}
