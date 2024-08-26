use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::models::base::object_store_ref::ObjRef;
use crate::models::structs::company::Company;

pub struct Testimonial {
	author: TestimonialAuthor,
	content: Vec<String>,
	date: DateTime<Utc>
}

pub enum TestimonialAuthor {
	Individual {
		name: String,
		picture: ObjRef,
		contact_information: HashMap<String, String>
	},

	CompanyRepresentative {
		name : String,
		picture: ObjRef,
		contact_information: HashMap<String, String>,
		company: Company,
		role: String
	}
}
