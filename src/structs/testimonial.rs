use crate::base::object_store_ref::ObjRef;
use crate::structs::asset::impl_asset;
use crate::structs::company::Company;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

pub struct Testimonial {
	pub author: TestimonialAuthor,
	pub content: Vec<String>,
	pub date: DateTime<Utc>,
}

pub enum TestimonialAuthor {
	Individual {
		name: String,
		picture: ObjRef,
		contact_information: HashMap<String, String>,
	},

	CompanyRepresentative {
		name: String,
		picture: ObjRef,
		contact_information: HashMap<String, String>,
		company: Company,
		role: String,
	},
}

impl_asset!(Testimonial);
