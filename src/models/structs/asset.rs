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

macro_rules! impl_asset {
	($type:ty) => {
		impl AssetEntity for $type {
			fn title(&self) -> &str {
				todo!()
			}

			fn skills(&self) -> &[Skill] {
				todo!()
			}

			fn related_assets(&self) -> &[Box<dyn AssetEntity>] {
				todo!()
			}
		}

		impl_base!($type);
	}
}

pub (crate) use {
	impl_asset
};
