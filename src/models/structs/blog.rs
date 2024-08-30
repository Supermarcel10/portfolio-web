use crate::models::base::content::Element;
use crate::models::structs::asset::impl_asset;
use crate::models::traits::{impl_attachable, impl_describable, impl_featurable};

pub struct Blog {
	pub slug: String,
	pub tags: Vec<String>,
	pub content: Vec<Element>,
	pub is_draft: bool
}

impl_asset!(Blog);
impl_attachable!(Blog);
impl_describable!(Blog);
impl_featurable!(Blog);
