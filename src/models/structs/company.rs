use crate::models::base::entity::impl_base;
use crate::models::traits::{impl_attachable};

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
