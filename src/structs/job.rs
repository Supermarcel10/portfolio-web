use crate::{impl_achievable, impl_asset, impl_attachable, impl_describable, impl_duration_dateable};
use super::company::Company;

pub struct Job {
	pub company: Company,
	pub responsibilities: Vec<String>
}

impl_asset!(Job);
impl_duration_dateable!(Job);
impl_describable!(Job);
impl_achievable!(Job);
impl_attachable!(Job);
