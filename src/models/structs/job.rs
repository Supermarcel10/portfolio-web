use crate::models::traits::{impl_achievable, impl_attachable, impl_describable, impl_duration_dateable};
use super::company::Company;

pub struct Job {
	pub company: Company,
	pub responsibilities: Vec<String>
}

impl_duration_dateable!(Job);
impl_describable!(Job);
impl_achievable!(Job);
impl_attachable!(Job);
