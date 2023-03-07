
use crate::*;
 
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug, Clone)]
pub struct IdeaMetadata {
    pub title: Option<String>, 
    pub excerpt: Option<String>,
    pub description: Option<String>, 
    pub competitors: Option<Vec<String>>,
    pub value_proposition: Option<String>, 
    pub tags:Option<Vec<String>>,
    pub team:Option<String>, 
    pub picture_url: Option<String>,
    pub owner_id: AccountId,
    pub website: Option<String>,
    pub token_contract: Option<AccountId>,
    pub airdrop: Option<String>,
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
    pub goal_amount: Balance,
    pub goal_reached: bool,
    pub phase_start: u64,
    pub phase_paid: bool, //paid
    pub collect_enabled: bool,
    pub active: bool,
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
    pub token_received: bool,

}

//The Json idea is what will be returned from view calls. 
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct JsonIdea {
    pub idea_id: IdeaId,
    pub title: Option<String>, 
    pub excerpt: Option<String>,
    pub description: Option<String>, // free-form description
    pub competitors: Option<Vec<String>>,
    pub value_proposition: Option<String>, // unique value proposition
    pub tags:Option<Vec<String>>,
    pub team:Option<String>, 
    pub picture_url: Option<String>,
    pub owner_id: AccountId,
    pub website: Option<String>,
    pub project_phase: u8,
    pub goal_amount: Balance,
    pub sum: f64,
    pub goal_reached: bool,
    pub phase_start: u64,
    pub investors_count: u64,
    pub collect_enabled: bool,
    pub active: bool,
    
}

//The Json idea with all the data from goals that will be returned from view calls, goals should be returned as a vector
#[derive(Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")]
#[derive(Debug)]
pub struct JsonIdeaWithGoals{
    pub idea_id: IdeaId,
    pub title: Option<String>, 
    pub excerpt: Option<String>,
    pub description: Option<String>, // free-form description
    pub competitors: Option<Vec<String>>,
    pub value_proposition: Option<String>, // unique value proposition
    pub tags:Option<Vec<String>>,
    pub team:Option<String>, 
    pub picture_url: Option<String>,
    pub owner_id: AccountId,
    pub website: Option<String>,
    pub goals: Vec<ProjectPhaseGoals>,
   
}

//The Json idea with all the data from investments that will be returned from view calls, investments should be returned as a sum for each phase
#[derive(Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")]
#[derive(Debug)]

pub struct JsonIdeaWithInvestments{
    pub idea_id: IdeaId,
    pub title: Option<String>, 
    pub excerpt: Option<String>,
    pub description: Option<String>, // free-form description
    pub competitors: Option<Vec<String>>,
    pub value_proposition: Option<String>, // unique value proposition
    pub tags:Option<Vec<String>>,
    pub team:Option<String>, 
    pub picture_url: Option<String>,
    pub owner_id: AccountId,
    pub website: Option<String>,
    pub investments: Vec<Investment>,
    pub investors_count: u64,
    pub active_phase: Option<u8>,
    pub token_contract: Option<AccountId>,
    pub airdrop: Option<String>,
   
}

#[derive(Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")]
#[derive(Debug)]
pub struct Investment {
    pub project_phase: u8,
    pub goal: Balance,
    pub sum: f64,
    pub goal_reached: bool,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug, Clone)]
pub struct FTmetadata{
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: Option<u8>,
    pub icon: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<String>,
    pub contract_spec: Option<String>,
    pub total_supply: Option<u128>,
    pub contract_supply: Option<u128>,
}


