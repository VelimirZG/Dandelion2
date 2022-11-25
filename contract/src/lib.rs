use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, LookupMap};
use near_sdk::{log, near_bindgen, Promise, env, Balance};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk:: AccountId;

pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;
pub const ONE_NEAR: Balance = 1_000_000_000_000_000_000_000_000;

pub type Date = u64;
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


//get goals
pub fn get_goals(&self, idea_id: IdeaId) -> Vec<ProjectPhaseGoals> {
    self.goals.get(&idea_id).unwrap_or_else(||Vec::new())
} 

//get current active goal for idea
pub fn get_active_project_phase(&self, idea_id: IdeaId) -> u8 {
    let goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
    let mut current_phase = ProjectPhaseGoals{idea_id: 0, project_phase: 0, amount: 0, goal_reached: false, phase_start: 0, phase_closed: false, collect_enabled: false};
    for goal in goals{
        if goal.goal_reached == false && goal.phase_closed == false{
            current_phase = goal;
            break;
        }
    }
    current_phase.project_phase
}

pub fn create_project_phase_goal(&mut self, idea_id: IdeaId, amount: Balance){
    let current_phase = self.get_active_project_phase(idea_id);
    let mut goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
    if current_phase == 0{
        goals.push(ProjectPhaseGoals{idea_id, project_phase: 1, amount, goal_reached: true, phase_start: env::block_timestamp(), phase_closed: false, collect_enabled: false});
        self.goals.insert(&idea_id, &goals);
        log!("New project phase goal created beacuse no active goal was found");
    }else if current_phase >= 1 && goals[current_phase as usize - 1].goal_reached == true{
        goals.push(ProjectPhaseGoals{idea_id, project_phase: current_phase + 1, amount, goal_reached: false, phase_start: env::block_timestamp(), phase_closed: false, collect_enabled: false});
        self.goals.insert(&idea_id, &goals);
        log!("New project phase goal created beacuse current goal was reached");
    }else if current_phase >= 1 && goals[current_phase as usize -1].goal_reached == false{
        panic!("Current goal is not reached yet");
    }
        
    

    
    }

//if goal exist and it is reached create new goal
pub fn create_goals(&mut self, idea_id: IdeaId, project_phase: u8, amount: Balance) {
    let mut goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
    let mut goal_exist = false;
    let mut previous_goal_reached=false;
    for goal in goals.iter_mut(){
        if goal.idea_id == idea_id && goal.project_phase == project_phase{
            goal_exist = true;
        }
    }
    
    for goal in goals.iter_mut(){
        if goal.idea_id == idea_id && goal.project_phase == project_phase-1 && goal.goal_reached == false{
            previous_goal_reached = false;
            log! ("Previous goal not reached");
        }else if goal.idea_id == idea_id && goal.project_phase == project_phase-1 && goal.goal_reached == true{
            previous_goal_reached = true;
            log!("Previous goal reached");
        }
    }
    let mut active_goal = self.get_active_project_phase(idea_id);
    log!("Goal exist: {}", goal_exist);
    log!("Previous goal reached: {}", previous_goal_reached);
    log!("Active goal: {}", active_goal);
    if !goal_exist && previous_goal_reached || self.get_active_project_phase(idea_id) == 0{
        goals.push(ProjectPhaseGoals{idea_id, project_phase, amount, goal_reached: false, phase_start: env::block_timestamp(),phase_closed: false, collect_enabled: false});
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


//once the buton is clicked, the funds are collected and the second phase of the project is enabled
//see if the one calling the function is the owner of the idea
//if yes, collect funds and enable second phase of project
//if no, return error
pub fn collect_funds(&mut self, idea_id: IdeaId, project_phase: u8){
    
    let phase_cloased = self.get_phase_closed(idea_id, project_phase);
    assert!(
        phase_cloased == false,
        "The phase is already closed",
    );
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
    self.set_phase_closed(idea_id, project_phase)
}

//check if phase is closed
pub fn get_phase_closed(&self, idea_id: IdeaId, project_phase: u8) -> bool{
    let goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
    let mut phase_closed = false;
    for goal in goals.iter(){
        if goal.idea_id == idea_id && goal.project_phase == project_phase{
            phase_closed = goal.phase_closed;
        }
    }
    phase_closed
}

//set project phase to closed
pub fn set_phase_closed(&mut self, idea_id: IdeaId, project_phase: u8){
    let mut goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
    for goal in goals.iter_mut(){
        if goal.idea_id == idea_id && goal.project_phase == project_phase{
            goal.phase_closed = true;
            goal.collect_enabled=false;
        }
    }
    self.goals.insert(&idea_id, &goals);
}


//calculate time passed from phase_start_time
//if time passed is greater than 30 days and goal_reached=true then transfer funds to owner and enable next phase,  othwevise if time passed is greater than 30 days and goal_reached=false return to investors
//check date
//if date is greater than 30 days, call internal function transfer_funds

pub fn time_passed(&self, idea_id: IdeaId, project_phase: u8) -> u64{
    let goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
    let mut time_passed = 0;
    for goal in goals.iter(){
        if goal.idea_id == idea_id && goal.project_phase == project_phase{
            time_passed = env::block_timestamp() - goal.phase_start;
        }
    }
    time_passed
}

pub fn check_date(&mut self, idea_id: IdeaId, project_phase: u8){
    let time_passed = self.time_passed(idea_id, project_phase);
    let goal_reached = self.get_goal_reached(idea_id, project_phase);
    let phase_closed = self.get_phase_closed(idea_id, project_phase);
    if time_passed > 259200000000 && goal_reached == true && phase_closed == false{
        log!("time passed and goal reached and phase not closed");
        self.transfer_funds(idea_id, project_phase);
        self.set_phase_closed(idea_id, project_phase);
        
    }
    else if time_passed > 259200000000 && goal_reached == false{
        log!("time passed and goal not reached");
        self.return_to_investors(idea_id, project_phase);
        
    }else if time_passed < 259200000000{
        log!("time not passed");
    }
}

//check date for project phases
pub fn check_date_for_project_phases(&mut self, idea_id: IdeaId){
    let project_phases = self.get_project_phases(idea_id);
    for project_phase in project_phases.iter(){
        self.check_date(idea_id, *project_phase);
    }
}

//get project phases
pub fn get_project_phases(&self, idea_id: IdeaId) -> Vec<u8>{
    let goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
    let mut project_phases = Vec::new();
    for goal in goals.iter(){
        if goal.idea_id == idea_id{
            project_phases.push(goal.project_phase);
        }
    }
    project_phases
}

//edit ideametadata if the one calling the function is the owner of the idea
pub fn edit_idea_metadata(&mut self, idea_id: IdeaId, metadata: IdeaMetadata){
    let owner_id = env::predecessor_account_id();
    let idea = self.ideas.get(&idea_id).unwrap();
    assert!(
        owner_id == idea.owner_id,
        "Only the owner of the idea can edit the idea metadata",
    );
    self.ideas.insert(&idea_id, &metadata);
}


//edit project_phase_goals if goal is not reached
pub fn edit_project_phase_goals(&mut self, idea_id: IdeaId, project_phase: u8, amount: u128){
    let owner_id = env::predecessor_account_id();
    let idea = self.ideas.get(&idea_id).unwrap();
    assert!(
        owner_id == idea.owner_id,
        "Only the owner of the idea can edit the idea metadata",
    );
    let goal_reached = self.get_goal_reached(idea_id, project_phase);
    assert!(
        goal_reached == false,
        "Goal reached, cannot edit project phase goals",
    );
    let mut goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
    for goal in goals.iter_mut(){
        if goal.idea_id == idea_id && goal.project_phase == project_phase{
            goal.amount = amount;
        
        }
    }
    self.goals.insert(&idea_id, &goals);
}



// //check_date for all ideas and all phases
// pub fn check_date_all(&mut self){
//     let ideas = self.ideas.keys_as_vector();
//     for idea_id in ideas.iter(){
//         let idea = self.ideas.get(&idea_id).unwrap();
//         let project_phases = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
//         for goal in project_phases.iter(){
//             self.check_date(idea_id, goal.project_phase);
//         }
//     }
// }

//return money to investors
pub fn return_to_investors(&mut self, idea_id: IdeaId, project_phase: u8){
    let investments = self.investment.keys_as_vector();
    for investment_id in investments.iter(){
        let investment = self.investment.get(&investment_id).unwrap();
        if investment.idea_id == idea_id && investment.project_phase == project_phase{
            Promise::new(investment.investor_id).transfer(investment.amount);
            log!("Funds returned to investor");
        }
    }
}

pub fn get_sum_of_amount(&self, idea_id: IdeaId) -> u128{
    let investments = self.investment.keys_as_vector();
    let mut sum_of_amount = 0;
    for investment_id in investments.iter(){
        let investment = self.investment.get(&investment_id).unwrap();
        if investment.idea_id == idea_id{
            sum_of_amount += investment.amount;
        }
    }
    sum_of_amount
}

    //get all the ideas for owner id and call get_sum_of_amount for each idea
    pub fn get_sum_of_amount_for_owner(&self, owner_id: AccountId) -> u128{
        let ideas = self.ideas.keys_as_vector();
        let mut sum_of_amount = 0;
        for idea_id in ideas.iter(){
            let idea = self.ideas.get(&idea_id).unwrap();
            if idea.owner_id == owner_id{
                sum_of_amount += self.get_sum_of_amount(idea_id);
            }
        }
        sum_of_amount
    }
    //get all the ideas for owner id and call get_sum_of_amount for each idea - use pagination
    pub fn get_sum_of_amount_for_owner_pagination(&self, owner_id: AccountId, from_index: u64, limit: u64) -> u128{
        let ideas = self.ideas.keys_as_vector();
        let mut sum_of_amount = 0;
        let mut index = 0;
        for idea_id in ideas.iter(){
            let idea = self.ideas.get(&idea_id).unwrap();
            if idea.owner_id == owner_id{
                if index >= from_index && index < from_index + limit{
                    sum_of_amount += self.get_sum_of_amount(idea_id);
                }
                index += 1;
            }
        }
        sum_of_amount
    }

}




