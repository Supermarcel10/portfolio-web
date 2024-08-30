use super::company::Company;
use crate::{
	impl_achievable, impl_asset, impl_attachable, impl_describable, impl_duration_dateable,
	impl_gradeable,
};

pub struct Education {
	pub institution: Company,
	pub degree: String,
	pub field_of_study: String,
	pub modules: Vec<Module>,
}

pub struct Module {
	pub code: Option<String>,
}

impl_asset!(Education);
impl_duration_dateable!(Education);
impl_describable!(Education);
impl_achievable!(Education);
impl_attachable!(Education);
impl_gradeable!(Education);

impl_asset!(Module);
impl_duration_dateable!(Module);
impl_describable!(Module);
impl_gradeable!(Module);
