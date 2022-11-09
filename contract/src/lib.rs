use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, LookupMap};
use near_sdk::{log, near_bindgen, Promise, env, Balance};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk:: AccountId;

pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;
pub const ONE_NEAR: Balance = 1_000_000_000_000_000_000_000_000;


pub type IdeaId=u64;
pub type InvestmentId=u64;

pub use crate::calls::*;
use crate::internal::*;
pub use crate::views::*;

mod calls;
mod views;
mod internal;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub ideas: UnorderedMap<IdeaId, IdeaMetadata>,
    pub investment: UnorderedMap<InvestmentId, InvestmentMetadata>,
    pub goals:UnorderedMap<IdeaId,Vec<ProjectPhaseGoals>>,

   
}

// Define the default, which automatically initializes the contract
impl Default for Contract{
    fn default() -> Self{
        Self{
            ideas: UnorderedMap::new(b"s".to_vec()),
            investment: UnorderedMap::new(b"i".to_vec()),
            goals: UnorderedMap::new(b"g".to_vec()),
          
        }
    }
}

    

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct IdeaMetadata {
    pub title: Option<String>, // ex. "Teleportal" or "Future bank"
    pub excerpt: Option<String>,
    pub description: Option<String>, // free-form description
    pub competitors: Option<Vec<String>>,
    pub value_proposition: Option<String>, // unique value proposition
    pub tags:Option<Vec<String>>,
    pub team:Option<Team>,
    pub picture_url: Option<String>,
    pub owner_id: AccountId,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct Team {
    pub name: String,
    pub cv: String,

}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
//struct for project with phase goals
pub struct ProjectPhaseGoals{
    pub idea_id: IdeaId,
    pub project_phase: u8,
    pub amount: Balance,
    pub goal_reached: bool,
}

//struct

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct InvestmentMetadata{
    pub idea_id: IdeaId,
    pub project_phase: u8,
    pub investor_id: AccountId,
    pub amount: Balance,

}

#[near_bindgen]
impl Contract {

//get total invested in idea by project phase  (idea_id, project_phase)
pub fn get_total_invested_by_idea_id(&self, idea_id: IdeaId, project_phase: u8)->Balance{
    self.investment
    .iter()
    .filter(|(_, investment)| investment.idea_id == idea_id && investment.project_phase == project_phase)
    .map(|(_, investment)| investment.amount)
    .sum()
}

//get toal invested in idea
pub fn get_total_invested_by_idea(&self, idea_id: IdeaId)->Balance{
    self.investment
    .iter()
    .filter(|(_, investment)| investment.idea_id == idea_id)
    .map(|(_, investment)| investment.amount)
    .sum()
}

// //create new goals
// pub fn new_goals(&mut self, idea_id: IdeaId, project_phase: u8, amount: Balance) {
//     let mut goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
//     goals.push(ProjectPhaseGoals{idea_id, project_phase, amount, goal_reached: false});
//     self.goals.insert(&idea_id, &goals);
// }



//get goals
pub fn get_goals(&self, idea_id: IdeaId) -> Vec<ProjectPhaseGoals> {
    self.goals.get(&idea_id).unwrap_or_else(||Vec::new())
} 

//if goal exist update goal othervise create new goal
pub fn update_or_create_goals(&mut self, idea_id: IdeaId, project_phase: u8, amount: Balance) {
    let mut goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
    let mut goal_exist = false;
    for goal in goals.iter_mut(){
        if goal.idea_id == idea_id && goal.project_phase == project_phase{
            goal_exist = true;
        }
    }
    if !goal_exist{
        goals.push(ProjectPhaseGoals{idea_id, project_phase, amount, goal_reached: false});
    }
    self.goals.insert(&idea_id, &goals);
}

 //get amount for project phase by idea id and project phase
 pub fn get_amount_by_project_phase(&self, idea_id: IdeaId, project_phase: u8) -> Balance {
    let goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
    let mut amount = 0;
    for goal in goals.iter(){
        if goal.idea_id == idea_id && goal.project_phase == project_phase{
            amount = goal.amount;
        }
    }
    amount
}


// //get amount for project phase goal by idea_id and project_phase
// pub fn get_goal_amount_by_idea_id(&self, idea_id: IdeaId, project_phase: u8)->Balance{
//     self.goals
//     .iter()
//     .filter(|(_, goal)| goal.idea_id == idea_id && goal.project_phase == project_phase)
//     .map(|(_, goal)| goal.amount)
//     .sum()
// }

#[payable]
//invest in idea by project phase
pub fn invest_in_idea(&mut self, idea_id: IdeaId, project_phase: u8){
   
    let investor_id = env::predecessor_account_id();
    //random number for investment id
    let investment_id = env::random_seed().iter().fold(0u64, |a, &b| (a << 8) | (b as u64));
    //get atached deposit
    let amount = env::attached_deposit();
     //assert if get_total_invested_by_idea_id + amount <= get_goal_amount_by_idea_id
    //assert that projectphase for that idea is not null

    //razdvojiti total invested i invested so far
    let invested=self.get_total_invested_by_idea_id(idea_id, project_phase);
    let to_be_invested=self.get_total_invested_by_idea_id(idea_id, project_phase) + amount;
    let goal=self.get_amount_by_project_phase(idea_id, project_phase)*ONE_NEAR;
    let mut still_to_invest:f32=0.0;


   
 
    assert!(
        self.get_amount_by_project_phase(idea_id, project_phase) != 0,
        "The project phase for that idea is not set",
    );
    

        still_to_invest=(goal as f32 - invested as f32) as f32;
        
        
    assert!(
        to_be_invested <= goal,
        // self.get_total_invested_by_idea_id(idea_id, project_phase) + amount <= self.get_amount_by_project_phase(idea_id, project_phase) *ONE_NEAR,
        "The amount of the investment is greater than the goal amount, left to invest:{}", still_to_invest,
    );


    //create investment
    let investment = InvestmentMetadata{
        idea_id,
        project_phase,
        investor_id,
        amount,
    };
    self.investment.insert(&investment_id, &investment);

//check if goal is reached and call function set_goal_reached
log!("to be invested:{}", to_be_invested);
log!("goal:{}", goal);
if to_be_invested == goal{
    self.set_goal_reached(idea_id, project_phase);
    log!("Goal reached");
    //TODO; enable button to collect funds and enable second phase of project
}



    // //if total invested is equal to goal amount, set goal_reached to true
    // if self.get_total_invested_by_idea_id(idea_id, project_phase) == self.get_goal_amount_by_idea_id(idea_id, project_phase) *ONE_NEAR{
    //     let mut goal = self.goals.get(&idea_id).unwrap();
    //     goal.goal_reached = true;
    //     self.goals.insert(&idea_id, &goal);
    // }
    
}

//once the buton is clicked, the funds are collected and the second phase of the project is enabled
//see if the one calling the function is the owner of the idea
//if yes, collect funds and enable second phase of project
//if no, return error
pub fn collect_funds(&mut self, idea_id: IdeaId, project_phase: u8){
    //check if the goal is reached
    let goal_reached = self.get_goal_reached(idea_id, project_phase);
    assert!(
        goal_reached == true,
        "Goal not reached"
    );
    let owner_id = env::predecessor_account_id();
    let idea = self.ideas.get(&idea_id).unwrap();
    assert!(
        owner_id == idea.owner_id,
        "Only the owner of the idea can collect funds",
    );
    //transfer funds to owner, call internal function transfer_funds
    self.transfer_funds(idea_id, project_phase);
    //enable second phase of project
}

}




