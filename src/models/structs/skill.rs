use crate::models::base::entity::{impl_base, Base};

pub struct Skill {
	base: Base, // TODO: Figure out the proper way to do this. Possibly with more macros?
	name: String,
	category: SkillCategory,
	proficiency: Option<u8>,
	description: Option<String>,
}

pub enum SkillCategory {
	SoftSkill{ skill: String },
	Technology { technology_name: String, version: String },
	Tool { tool_name: String, version: Option<String> },
}

impl_base!(Skill);
