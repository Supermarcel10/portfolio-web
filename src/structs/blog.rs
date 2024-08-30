use crate::{impl_asset, impl_attachable, impl_describable, impl_featurable};
use crate::base::content::Element;

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
