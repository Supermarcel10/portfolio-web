use crate::models::base::entity::impl_base;

pub struct Skill {
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
