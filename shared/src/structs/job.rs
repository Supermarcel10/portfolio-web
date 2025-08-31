use super::company::Company;
use crate::{
	impl_achievable, impl_asset, impl_attachable, impl_describable, impl_duration_dateable,
};

pub struct Job {
	pub company: Company,
	pub responsibilities: Vec<String>,
}

impl_asset!(Job);
impl_duration_dateable!(Job);
impl_describable!(Job);
impl_achievable!(Job);
impl_attachable!(Job);
