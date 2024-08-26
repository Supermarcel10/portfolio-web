use chrono::{DateTime, Utc};
use crate::models::base::entity::{Base, BaseEntity};
use super::skill::Skill;

pub trait AssetEntity: BaseEntity {
	fn title(&self) -> &str;
	fn skills(&self) -> &[Skill];
	fn related_assets(&self) -> &[Box<dyn AssetEntity>];
}

pub struct Asset {
	pub base: Base,
	pub title: String,
	pub skills: Vec<Skill>,
	pub related_assets: Vec<Box<dyn AssetEntity>>,
}

impl BaseEntity for Asset {
	fn id(&self) -> &str {
		self.base.id()
	}

	fn insert_date_time(&self) -> DateTime<Utc> {
		self.base.insert_date_time()
	}

	fn update_date_time(&self) -> Option<DateTime<Utc>> {
		self.base.update_date_time()
	}

	fn created_by(&self) -> &str {
		self.base.created_by()
	}

	fn updated_by(&self) -> Option<&str> {
		self.base.updated_by()
	}

	fn is_active(&self) -> bool {
		self.base.is_active()
	}
}

impl AssetEntity for Asset {
	fn title(&self) -> &str {
		&self.title
	}

	fn skills(&self) -> &[Skill] {
		&self.skills
	}

	fn related_assets(&self) -> &[Box<dyn AssetEntity>] {
		&self.related_assets
	}
}
