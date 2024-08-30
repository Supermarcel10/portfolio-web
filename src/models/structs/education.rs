use crate::models::structs::asset::impl_asset;
use crate::models::traits::{impl_duration_dateable, impl_describable, impl_achievable, impl_attachable, impl_gradeable};
use super::company::Company;

pub struct Education {
	institution: Company,
	degree: String,
	field_of_study: String,
	modules: Vec<Module>
}

pub struct Module {
	code: Option<String>
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
