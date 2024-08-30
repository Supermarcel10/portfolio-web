use crate::{impl_attachable, impl_base};

pub struct Company {
	name : String,
	location : Option<String>,
	industry : Option<String>,
	company_type : CompanyType
}

pub enum CompanyType {
	EducationalInstitution,
	Corporation,
	Startup,
	Government,
	NonProfit,
	Client
}

impl_base!(Company);
impl_attachable!(Company);
