


use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap};
use near_sdk::{log, near_bindgen, Promise, env, Balance};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk:: AccountId;

pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;
pub const ONE_NEAR: Balance = 1_000_000_000_000_000_000_000_000;

pub type Date = u64;
pub type IdeaId=u64;
pub type InvestmentId=u64;

pub use crate::metadata::*;
pub use crate::calls::*;
pub use crate::internal::*;
pub use crate::views::*;


mod calls;
mod views;
mod internal;
mod metadata;



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

#[near_bindgen]
impl Contract {


//get total invested in idea by project phase
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


// //filter ideas where goal is not reached and phase is not closed
// pub fn get_active_project_phase_for_idea(&self, idea_id: IdeaId)->u8{
//     self.goals
//     .get(&idea_id) //zamijeniti sa get_all
//     .unwrap()
//     .iter()
//     .filter(|goal| goal.goal_reached == false && goal.phase_closed == false)
//     .map(|goal| goal.project_phase)
//     .min()
//     .unwrap()
// }


//get current active goal for idea
pub fn get_active_project_phase(&self, idea_id: IdeaId) -> u8 {
    let goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
    let mut current_phase = ProjectPhaseGoals{idea_id: 0, project_phase: 0, amount: 0, goal_reached: false, phase_start: 0, phase_closed: false, collect_enabled: false};
    for goal in goals{
        if goal.goal_reached == false && goal.phase_closed == false && goal.amount != 0 {
            current_phase = goal;
            break;
        }
    }
    current_phase.project_phase
}

//create function that will make a for loop 4 times with function get_amount_by_project_phase and make sum of results
pub fn get_total_amount_by_idea(&self, idea_id: IdeaId) -> Balance {
    let mut total_amount = 0;
    for i in 1..5{
        total_amount += self.get_amount_by_project_phase(idea_id, i);
    }
    total_amount
}

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




// //this is used to regularly check if the goal has expried
// pub fn check_date_all_pagination(&mut self, from_index: usize, limit: usize) {
//     let ideas_phases = self.get_all_ideas_and_phases();
//     for idea_phase in ideas_phases.iter().skip(from_index).take(limit) {
        
//         let time_passed = self.time_passed(idea_phase.0, idea_phase.1);
//     let goal_reached = self.get_goal_reached(idea_phase.0, idea_phase.1);
//     let phase_closed = self.get_phase_closed(idea_phase.0, idea_phase.1);
//     if time_passed > 259200000000 && goal_reached == true && phase_closed == false{
//         log!("time passed and goal reached and phase not closed");
//         self.transfer_funds(idea_phase.0, idea_phase.1);
//         self.set_phase_closed(idea_phase.0, idea_phase.1);
        
//     }
//     else if time_passed > 259200000000 && goal_reached == false{
//         log!("time passed and goal not reached");
//         self.return_to_investors(idea_phase.0, idea_phase.1);
//         self.set_phase_closed(idea_phase.0, idea_phase.1);
        
//     }else if time_passed < 259200000000{
//         log!("time not passed");
//     }
//     }
// }

//this is used to regularly check if the goal has expried
pub fn check_date_all_pagination(&mut self, from_index: usize, limit: usize) {
    let ideas_phases = self.get_all_ideas_and_phases();
    for idea_phase in ideas_phases.iter().skip(from_index).take(limit) {
        
    let time_passed = self.time_passed(idea_phase.0, idea_phase.1);
    let goal_reached = self.get_goal_reached(idea_phase.0, idea_phase.1);
    let phase_closed = self.get_phase_closed(idea_phase.0, idea_phase.1);
    if time_passed > 259200000000 && goal_reached == false{
        log!("time passed and goal not reached");
        log!("idea id: {} and phase: {} has been closed wihtout reaching goal", idea_phase.0, idea_phase.1);
        self.return_to_investors(idea_phase.0, idea_phase.1);
        self.set_phase_closed(idea_phase.0, idea_phase.1);
        
    }else if time_passed < 259200000000{
        log!("time not passed");
    }
    }
}

//refactor upper function without pagination
pub fn check_date_all(&mut self) {
    let ideas_phases = self.get_all_ideas_and_phases();
    for idea_phase in ideas_phases.iter() {
        
    let time_passed = self.time_passed(idea_phase.0, idea_phase.1);
    let goal_reached = self.get_goal_reached(idea_phase.0, idea_phase.1);
    //let phase_closed = self.get_phase_closed(idea_phase.0, idea_phase.1);
    if time_passed > 259200000000 && goal_reached == false{
        log!("time passed and goal not reached");
        log!("idea id: {} and phase: {} has been closed wihtout reaching goal", idea_phase.0, idea_phase.1);
        self.return_to_investors(idea_phase.0, idea_phase.1);
        self.set_phase_closed(idea_phase.0, idea_phase.1);
        
    }else if time_passed < 259200000000{
        log!("time not passed");
    }
    }
}

//get all ideas and project phases and return them as a vector
pub fn get_all_ideas_and_phases(&self) -> Vec<(IdeaId, u8)>{
    let mut ideas_phases = Vec::new();
    let ideas = self.ideas.keys_as_vector();
    for idea_id in ideas.iter(){
        let idea = self.ideas.get(&idea_id).unwrap();
        let project_phases = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
        for goal in project_phases.iter(){
            ideas_phases.push((idea_id, goal.project_phase));
        }
    }
    ideas_phases
}



    // //get all the ideas for owner id and call get_sum_of_amount for each idea - use pagination
    // pub fn get_sum_of_amount_for_owner(&self, owner_id: AccountId, from_index: u64, limit: u64) -> u128{
    //     let ideas = self.ideas.keys_as_vector();
    //     let mut sum_of_amount = 0;
    //     let mut index = 0;
    //     for idea_id in ideas.iter(){
    //         let idea = self.ideas.get(&idea_id).unwrap();
    //         if idea.owner_id == owner_id{
    //             if index >= from_index && index < from_index + limit{
    //                 sum_of_amount += self.get_sum_of_amount(idea_id);
    //             }
    //             index += 1;
    //         }
    //     }
    //     sum_of_amount
    // }

    //USE THIS INSTEAD ONE ABOVE, it is faster BUT CHECK IF IT WORKS FOR LARGE NUMBERS
    pub fn get_sum_of_amount_for_owner(&self, owner_id: AccountId) -> f64{
        let ideas = self.ideas.keys_as_vector();
        let mut sum_of_amount = 0;
        let mut sum_as_near:f64 = 0.0;
        for idea_id in ideas.iter(){
            let idea = self.ideas.get(&idea_id).unwrap();
            if idea.owner_id == owner_id{
                sum_of_amount += self.get_sum_of_amount(idea_id);
                sum_as_near= self.yocto_to_near(sum_of_amount);
            }
        }
        sum_as_near
    }

    // //get investor count for all ideas for owner id - use pagination
    // pub fn get_investor_count_for_owner(&self, owner_id: AccountId, from_index: u64, limit: u64) -> u64{
    //     let ideas = self.ideas.keys_as_vector();
    //     let mut investor_count = 0;
    //     let mut index = 0;
    //     for idea_id in ideas.iter(){
    //         let idea = self.ideas.get(&idea_id).unwrap();
    //         if idea.owner_id == owner_id{
    //             if index >= from_index && index < from_index + limit{
    //                 investor_count += self.get_investor_count(idea_id);
    //             }
    //             index += 1;
    //         }
    //     }
    //     investor_count
    // }

    //get investor count for all ideas for owner id - WITHOUT pagination
    pub fn get_investor_count_for_owner(&self, owner_id: AccountId) -> u64{
        let ideas = self.ideas.keys_as_vector();
        let mut investor_count = 0;
        for idea_id in ideas.iter(){
            let idea = self.ideas.get(&idea_id).unwrap();
            if idea.owner_id == owner_id{
                investor_count += self.get_investor_count(idea_id);
            }
        }
        investor_count
    }

    pub fn get_investor_count(&self, idea_id: IdeaId) -> u64{
        let investments = self.investment.keys_as_vector();
        let mut investor_count = 0;
        for investment_id in investments.iter(){
            let investment = self.investment.get(&investment_id).unwrap();
            if investment.idea_id == idea_id{
                investor_count += 1;
            }
        }
        investor_count
    }

    //get how much one account invested in ideas
    pub fn get_sum_of_amount_for_investor(&self, investor_id: AccountId) -> f64{
        let investments = self.investment.keys_as_vector();
        let mut sum = 0;
        let mut sum_of_amount = 0.0;
        for investment_id in investments.iter(){
            let investment = self.investment.get(&investment_id).unwrap();
            if investment.investor_id == investor_id{
                sum+= investment.amount;
            }
        }
        sum_of_amount = self.yocto_to_near(sum);
        sum_of_amount
    }

//get sum of ideas that investor invested in
pub fn get_invested_ideas_count(&self, investor_id: AccountId) -> u64{
    let investments = self.investment.keys_as_vector();
    let mut invested_ideas_count = 0;
    for investment_id in investments.iter(){
        let investment = self.investment.get(&investment_id).unwrap();
        if investment.investor_id == investor_id{
            invested_ideas_count += 1;
        }
    }
    invested_ideas_count
}



// //get sum of amount of investments for each prject phase
// pub fn get_sum_of_amounts(&self, idea_id: IdeaId, project_phase: u8)->Balance{
//     self.investment
//     .iter()
//     .filter(|(_, investment)| investment.idea_id == idea_id && investment.project_phase == project_phase)
//     .map(|(_, investment)| investment.amount)
//     .sum()
// }

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





// //collect funds for project phase(replaced by newer function to enable multiple phases)
// pub fn collect_funds_old_working(&mut self, idea_id: IdeaId, project_phase: u8){
    
//     let phase_cloased = self.get_phase_closed(idea_id, project_phase);
//     assert!(
//         phase_cloased == false,
//         "The phase is already closed",
//     );
//     //check if the goal is reached
//     let goal_reached = self.get_goal_reached(idea_id, project_phase);
//     assert!(
//         goal_reached == true,
//         "Goal not reached"
//     );
    
//     let owner_id = env::predecessor_account_id();
//     let idea = self.ideas.get(&idea_id).unwrap();
//     assert!(
//         owner_id == idea.owner_id,
//         "Only the owner of the idea can collect funds",
//     );
//     //transfer funds to owner, call internal function transfer_funds
//     self.transfer_funds(idea_id, project_phase);
//     //enable second phase of project
//     self.set_phase_closed(idea_id, project_phase)
// }

pub fn collect_funds_for_all_phases(&mut self, idea_id: IdeaId) {
    for project_phase in 1..=4 {
        log!("Collecting funds for phase {} and idea_id {}", project_phase, idea_id);
        let result = self.collect_funds(idea_id, project_phase);
        let result_str = String::from_utf8(result).unwrap();
        log!("Result for phase {}: {:?}", project_phase, result_str);
        // check the value of result_str
        // if it's equals to "Funds collected successfully" the funds collection for that phase succeeded
        // otherwise it contains the reason why the collection failed
    }
    log!("Funds collection for all phases completed.");
}


pub fn collect_funds(&mut self, idea_id: IdeaId, project_phase: u8) -> Vec<u8>{
    let phase_cloased = self.get_phase_closed(idea_id, project_phase);
    if phase_cloased {
        return "The phase is already closed".as_bytes().to_vec();
    }
    //check if the goal is reached
    let goal_reached = self.get_goal_reached(idea_id, project_phase);
    if !goal_reached {
        return "Goal not reached".as_bytes().to_vec();
    }
    let owner_id = env::predecessor_account_id();
    let idea = self.ideas.get(&idea_id).unwrap();
    if owner_id != idea.owner_id {
        return "Only the owner of the idea can collect funds".as_bytes().to_vec();
    }
    //transfer funds to owner, call internal function transfer_funds
    self.transfer_funds(idea_id, project_phase);
    //enable second phase of project
    self.set_phase_closed(idea_id, project_phase);
    "Funds collected successfully".as_bytes().to_vec()
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

//if goal exist and it is reached create new goal
// pub fn create_goals(&mut self, idea_id: IdeaId, project_phase: u8, amount: Balance) {
//     let owner_id = env::predecessor_account_id();
//     let idea = self.ideas.get(&idea_id).unwrap();
//     assert!(
//         owner_id == idea.owner_id,
//         "Only the owner of the idea can edit create goals",
//     );
//     let mut goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
//     let mut goal_exist = false;
//     let mut previous_goal_reached=false;
//     for goal in goals.iter_mut(){
//         if goal.idea_id == idea_id && goal.project_phase == project_phase{
//             goal_exist = true;
//         }
//     }
    
//     for goal in goals.iter_mut(){
//         if goal.idea_id == idea_id && goal.project_phase == project_phase-1 && goal.goal_reached == false{
//             previous_goal_reached = false;
//             log! ("Previous goal not reached");
//         }else if goal.idea_id == idea_id && goal.project_phase == project_phase-1 && goal.goal_reached == true{
//             previous_goal_reached = true;
//             log!("Previous goal reached");
//         }
//     }
//     let active_goal = self.get_active_project_phase(idea_id);
//     log!("Goal exist: {}", goal_exist);
//     log!("Previous goal reached: {}", previous_goal_reached);
//     log!("Active goal: {}", active_goal);
//     if !goal_exist && previous_goal_reached || self.get_active_project_phase(idea_id) == 0{
//         goals.push(ProjectPhaseGoals{idea_id, project_phase, amount, goal_reached: false, phase_start: env::block_timestamp(),phase_closed: false, collect_enabled: false});
//     }
//     self.goals.insert(&idea_id, &goals);
// }

//refactor create_goals just to input goal, without any checks
pub fn create_goals(&mut self, idea_id: IdeaId, project_phase: u8, amount: Balance) {
    let mut goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
    goals.push(ProjectPhaseGoals{idea_id, project_phase, amount, goal_reached: false, phase_start: env::block_timestamp(),phase_closed: false, collect_enabled: false});
    self.goals.insert(&idea_id, &goals);
    log!("Goal phase {} created", project_phase);
}


//combine edit_idea_metadata and edit_project_phase_goals into one function
pub fn edit_idea(&mut self, idea_id: IdeaId, metadata: IdeaMetadata, amount1:u128, amount2:u128, amount3:u128, amount4:u128){
    let owner_id = env::predecessor_account_id();
    let idea = self.ideas.get(&idea_id).unwrap();
    assert!(
        owner_id == idea.owner_id,
        "Only the owner of the idea can edit the idea metadata",
    );
    self.ideas.insert(&idea_id, &metadata);
    
    let mut goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
    let ProjectPhaseGoals = [amount1, amount2, amount3, amount4];
    let mut i =0;
   let mut previous_goal_reached = false;
    for goal in goals.iter_mut(){
        if goal.idea_id == idea_id {
            let goal_reached = self.get_goal_reached(idea_id, goal.project_phase);
    if goal_reached == true{
        log!("Goal phase {} reached, cannot edit", goal.project_phase);
        previous_goal_reached = true;
        i=i+1;

        continue;
    }else if previous_goal_reached == true{
            log!("Goal phase {} edited", goal.project_phase);
            log!("New amount: {}", ProjectPhaseGoals[i]);
            goal.amount = ProjectPhaseGoals[i];
            i=i+1; 
            previous_goal_reached = false;
            }else{
             log!("Goal phase {}, cannot edit. Previous goal not reached", goal.project_phase);
            }
    }
        
    }
    self.goals.insert(&idea_id, &goals);
}
}





// pub fn create_project_phase_goal(&mut self, idea_id: IdeaId, amount: Balance){
//     let current_phase = self.get_active_project_phase(idea_id);
//     let mut goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
//     if current_phase == 0{
//         goals.push(ProjectPhaseGoals{idea_id, project_phase: 1, amount, goal_reached: true, phase_start: env::block_timestamp(), phase_closed: false, collect_enabled: false});
//         self.goals.insert(&idea_id, &goals);
//         log!("New project phase goal created beacuse no active goal was found");
//     }else if current_phase >= 1 && goals[current_phase as usize - 1].goal_reached == true{
//         goals.push(ProjectPhaseGoals{idea_id, project_phase: current_phase + 1, amount, goal_reached: false, phase_start: env::block_timestamp(), phase_closed: false, collect_enabled: false});
//         self.goals.insert(&idea_id, &goals);
//         log!("New project phase goal created beacuse current goal was reached");
//     }else if current_phase >= 1 && goals[current_phase as usize -1].goal_reached == false{
//         panic!("Current goal is not reached yet");
//     }
        
    

    
//     }


    








// //calculate time passed from phase_start_time
// //if time passed is greater than 30 days and goal_reached=true then transfer funds to owner and enable next phase,  othwevise if time passed is greater than 30 days and goal_reached=false return to investors
// //check date
// //if date is greater than 30 days, call internal function transfer_funds



// // pub fn check_date(&mut self, idea_id: IdeaId, project_phase: u8){
// //     let time_passed = self.time_passed(idea_id, project_phase);
// //     let goal_reached = self.get_goal_reached(idea_id, project_phase);
// //     let phase_closed = self.get_phase_closed(idea_id, project_phase);
// //     if time_passed > 259200000000 && goal_reached == true && phase_closed == false{
// //         log!("time passed and goal reached and phase not closed");
// //         self.transfer_funds(idea_id, project_phase);
// //         self.set_phase_closed(idea_id, project_phase);
        
// //     }
// //     else if time_passed > 259200000000 && goal_reached == false{
// //         log!("time passed and goal not reached");
// //         self.return_to_investors(idea_id, project_phase);
        
// //     }else if time_passed < 259200000000{
// //         log!("time not passed");
// //     }
// // }




// //check date for project phases
// pub fn check_date_for_project_phases(&mut self, idea_id: IdeaId){
//     let project_phases = self.get_project_phases(idea_id);
//     for project_phase in project_phases.iter(){
//         self.check_date(idea_id, *project_phase);
//     }
// }










        

// // //check_date for all ideas and all phases
// // pub fn check_date_all(&mut self){
// //     let ideas = self.ideas.keys_as_vector();
// //     for idea_id in ideas.iter(){
// //         let idea = self.ideas.get(&idea_id).unwrap();
// //         let project_phases = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
// //         for goal in project_phases.iter(){
// //             self.check_date(idea_id, goal.project_phase);
// //         }
// //     }
// // }

// // pub fn check_date_all(&mut self){
// //     let ideas = self.ideas.keys_as_vector();
// //     for idea_id in ideas.iter(){
// //         let idea = self.ideas.get(&idea_id).unwrap();
// //         let project_phases = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
// //         for goal in project_phases.iter(){
// //             self.check_date(self.get_all_ideas_and_phases());
// //         }
// //     }
// // }

// //get all ideas and project phases and return them as a vector
// pub fn get_all_ideas_and_phases(&self) -> Vec<(IdeaId, u8)>{
//     let mut ideas_phases = Vec::new();
//     let ideas = self.ideas.keys_as_vector();
//     for idea_id in ideas.iter(){
//         let idea = self.ideas.get(&idea_id).unwrap();
//         let project_phases = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
//         for goal in project_phases.iter(){
//             ideas_phases.push((idea_id, goal.project_phase));
//         }
//     }
//     ideas_phases
// }

// //take all ideas and phases and check date
// pub fn check_date_now(&mut self){
//     let ideas_phases = self.get_all_ideas_and_phases();
//     for idea_phase in ideas_phases.iter(){
//         self.check_date(idea_phase.0, idea_phase.1);
//     }
// }

// //function that will iterate check date for all ideas and all phases
// pub fn check_date_all(&mut self){
//     let ideas_phases = self.get_all_ideas_and_phases();
//     for idea_phase in ideas_phases.iter(){
//         self.check_date(idea_phase.0, idea_phase.1);
//     }
// }










// pub fn check_date(&mut self, idea_id: IdeaId, project_phase: u8){
//     let time_passed = self.time_passed(idea_id, project_phase);
//     let goal_reached = self.get_goal_reached(idea_id, project_phase);
//     let phase_closed = self.get_phase_closed(idea_id, project_phase);
//     if time_passed > 259200000000 && goal_reached == true && phase_closed == false{
//         log!("time passed and goal reached and phase not closed");
//         self.transfer_funds(idea_id, project_phase);
//         self.set_phase_closed(idea_id, project_phase);
        
//     }
//     else if time_passed > 259200000000 && goal_reached == false{
//         log!("time passed and goal not reached");
//         self.return_to_investors(idea_id, project_phase);
        
//     }else if time_passed < 259200000000{
//         log!("time not passed");
//     }
// }

// //combine check_date and check_date_all into one function



























 





