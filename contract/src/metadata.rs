
use crate::*;
 
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug, Clone)]
pub struct IdeaMetadata {
    pub title: Option<String>, // ex. "Teleportal" or "Future bank"
    pub excerpt: Option<String>,
    pub description: Option<String>, // free-form description
    pub competitors: Option<Vec<String>>,
    pub value_proposition: Option<String>, // unique value proposition
    pub tags:Option<Vec<String>>,
    pub team:Vec<Option<Team>>,
    pub picture_url: Option<String>,
    pub owner_id: AccountId,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug, Clone)]
pub struct Team {
    pub name: String,
    pub cv: String,

}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
#[derive(Clone, Copy)]
//struct for project with phase goals
pub struct ProjectPhaseGoals{
    pub idea_id: IdeaId,
    pub project_phase: u8,
    pub amount: Balance,
    pub goal_reached: bool,
    pub phase_start: u64,
    pub phase_closed: bool,
    pub collect_enabled: bool,
}

//struct

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug, Clone)]
pub struct InvestmentMetadata{
    pub idea_id: IdeaId,
    pub project_phase: u8,
    pub investor_id: AccountId,
    pub amount: Balance,

}

//The Json idea is what will be returned from view calls. 
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct JsonIdea {
    pub idea_id: IdeaId,
    pub title: Option<String>, // ex. "Teleportal" or "Future bank"
    pub excerpt: Option<String>,
    pub description: Option<String>, // free-form description
    pub competitors: Option<Vec<String>>,
    pub value_proposition: Option<String>, // unique value proposition
    pub tags:Option<Vec<String>>,
    pub team:Vec<Option<Team>>,
    pub picture_url: Option<String>,
    pub owner_id: AccountId,
    pub project_phase: u8,
    pub amount: Balance,
    pub sum: f64,
    pub goal_reached: bool,
    pub phase_start: u64,
    pub investors_count: u64,
    
}

//The Json idea with all the data from goals that will be returned from view calls, goals should be reurned as a vector
#[derive(Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")]
#[derive(Debug)]
pub struct JsonIdeaWithGoals{
    pub idea_id: IdeaId,
    pub title: Option<String>, // ex. "Teleportal" or "Future bank"
    pub excerpt: Option<String>,
    pub description: Option<String>, // free-form description
    pub competitors: Option<Vec<String>>,
    pub value_proposition: Option<String>, // unique value proposition
    pub tags:Option<Vec<String>>,
    pub team:Vec<Option<Team>>,
    pub picture_url: Option<String>,
    pub owner_id: AccountId,
    pub goals: Vec<ProjectPhaseGoals>,
   
}

//The Json idea with all the data from investments that will be returned from view calls, investments should be reurned as a sum for each phase
#[derive(Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")]
#[derive(Debug)]

pub struct JsonIdeaWithInvestments{
    pub idea_id: IdeaId,
    pub title: Option<String>, // ex. "Teleportal" or "Future bank"
    pub excerpt: Option<String>,
    pub description: Option<String>, // free-form description
    pub competitors: Option<Vec<String>>,
    pub value_proposition: Option<String>, // unique value proposition
    pub tags:Option<Vec<String>>,
    pub team:Vec<Option<Team>>,
    pub picture_url: Option<String>,
    pub owner_id: AccountId,
    pub investments: Vec<Investment>,
    pub investors_count: u64,
   
}

#[derive(Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")]
#[derive(Debug)]
pub struct Investment {
    pub project_phase: u8,
    pub goal: Balance,
    pub sum: f64,
}