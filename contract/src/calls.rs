
use crate::*;
use near_sdk::{near_bindgen, AccountId, env};


#[near_bindgen]
impl Contract {
    pub fn create_idea(&mut self, idea_id:IdeaId, metadata:IdeaMetadata, amount1:u128, amount2:u128, amount3:u128, amount4:u128) {
        assert!(self.ideas.insert(&idea_id, &metadata).is_none(), "Idea already exists");

        let project_phase_goals = [amount1, amount2, amount3, amount4];
        self.ideas.insert(&idea_id, &metadata);
        project_phase_goals.iter().enumerate().for_each(|(i, amount)| {
            log!("Creating goal for idea {} phase {} amount {}", idea_id, i, amount) ;
            let project_phase = (i + 1) as u8;
            match project_phase {
                1 => self.create_goals(idea_id, project_phase, *amount, true),
                _ => self.create_goals(idea_id, project_phase, *amount, false),
            }
        });
    }

    pub fn create_goals(&mut self, idea_id: IdeaId, project_phase: u8, goal_amount: Balance, active: bool) {
        log!("Creating goal for idea {} phase {} amount {}", idea_id, project_phase, goal_amount) ;  
        let mut goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
        goals.push(ProjectPhaseGoals{idea_id, project_phase, goal_amount, goal_reached: false, phase_start: env::block_timestamp(),phase_paid: false, collect_enabled: false, active });
        self.goals.insert(&idea_id, &goals);
        log!("Goal phase {} created", project_phase);
    }

    //invest active phase of idea
    #[payable]
    pub fn invest_in_idea(&mut self, idea_id: IdeaId) {
        let project_phase = self.get_active_project_phase(idea_id);
        let investor_id = env::predecessor_account_id();
        let investment_id = env::random_seed().iter().fold(0u64, |a, &b| (a << 8) | (b as u64));
        let amount = env::attached_deposit();

        assert!(amount >= 10000000000000000, "The minimum amount to invest is 0.01 NEAR");

        let invested = self.get_total_invested_by_idea_id(idea_id, project_phase);
        let to_be_invested = invested + amount;
        let goal = self.get_goal_by_project_phase(idea_id, project_phase) * ONE_NEAR;
        let still_to_invest = (goal - invested) as f32;
    
        assert!(self.get_goal_by_project_phase(idea_id, project_phase) != 0,
                "The project phase for that idea is not set");
    
        assert!(to_be_invested <= goal,
                "The amount of the investment is greater than the goal amount, left to invest: {}",
                still_to_invest / ONE_NEAR as f32);
    
        let investment = InvestmentMetadata{
            idea_id,
            project_phase,
            investor_id,
            amount,
        };
        self.investment.insert(&investment_id, &investment);
    
        if goal - to_be_invested <= 1000000000 {
            self.set_goal_reached(idea_id, project_phase);
            log!("Goal reached");
        }

    }


pub fn set_goal_reached(&mut self, idea_id: IdeaId, project_phase: u8) {
    let mut goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
    for goal in goals.iter_mut(){
        if goal.idea_id == idea_id && goal.project_phase == project_phase {
            goal.goal_reached = true;
            goal.collect_enabled = true;
            goal.active = false;
        }
    }
    self.goals.insert(&idea_id, &goals);
}

//get_goal_reached
pub(crate) fn get_goal_reached(&self, idea_id: IdeaId, project_phase: u8) -> bool {
    let goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
    for goal in goals.iter(){
        if goal.idea_id == idea_id && goal.project_phase == project_phase && goal.goal_reached == true {
            return goal.goal_reached;
        }
    }
    false
}

//edit idea and phase amount but only if phase is active
pub fn edit_idea(&mut self, idea_id: IdeaId, metadata: IdeaMetadata, amount1:u128, amount2:u128, amount3:u128, amount4:u128){
    let owner_id = env::predecessor_account_id();
    let idea = self.ideas.get(&idea_id).unwrap();
    assert!(
        owner_id == idea.owner_id,
        "Only the owner of the idea can edit the idea metadata",
    );
    self.ideas.insert(&idea_id, &metadata);
    let mut goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
    let project_phase_goals = [amount1, amount2, amount3, amount4];
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
        //assert that project phase goal is more then 0
        assert!(project_phase_goals[i] > 0, "The goal amount for the project phase must be greater than 0");
            log!("Goal phase {} edited", goal.project_phase);
            log!("New amount: {}", project_phase_goals[i]);
            goal.goal_amount = project_phase_goals[i];
            i=i+1; 
            previous_goal_reached = false;
            //set goal to active
            goal.active = true;
            }else{
             log!("Goal phase {}, cannot edit. Previous goal not reached", goal.project_phase);
            }
    }
        
    }
    self.goals.insert(&idea_id, &goals);
}

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
    let phase_paid = self.get_phase_paid(idea_id, project_phase);
    if phase_paid {
        return "The phase is already paid".as_bytes().to_vec();
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
    self.set_phase_paid(idea_id, project_phase);
    "Funds collected successfully".as_bytes().to_vec()
}

//set project phase to closed
pub fn set_phase_paid(&mut self, idea_id: IdeaId, project_phase: u8){
    let mut goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
    for goal in goals.iter_mut(){
        if goal.idea_id == idea_id && goal.project_phase == project_phase{
            goal.phase_paid = true;
            goal.collect_enabled=false;
        }
    }
    self.goals.insert(&idea_id, &goals);
}


// //this is used to regularly check if the goal has expried -TODO DODATI AKO JE ACTIVE
// pub fn check_date_all_pagination(&mut self, from_index: usize, limit: usize) {
//     let ideas_phases = self.get_all_ideas_and_phases();
//     for idea_phase in ideas_phases.iter().skip(from_index).take(limit) {
        
//     let time_passed = self.time_passed(idea_phase.0, idea_phase.1);
//     let goal_reached = self.get_goal_reached(idea_phase.0, idea_phase.1);
//     let phase_closed = self.get_phase_closed(idea_phase.0, idea_phase.1);
//     if time_passed > 259200000000 && goal_reached == false && active == true{
//         log!("time passed and goal not reached");
//         log!("idea id: {} and phase: {} has been closed wihtout reaching goal", idea_phase.0, idea_phase.1);
//         self.return_to_investors(idea_phase.0, idea_phase.1);
//         self.set_phase_closed(idea_phase.0, idea_phase.1);
        
//     }else if time_passed < 259200000000{
//         log!("time not passed");
//     }
//     }
// }

//refactor upper function without pagination
pub fn check_date_all(&mut self) {
    let ideas_phases = self.get_all_ideas_and_phases();
    for idea_phase in ideas_phases.iter() {
        
    let time_passed = self.time_passed(idea_phase.0, idea_phase.1);
    let goal_reached = self.get_goal_reached(idea_phase.0, idea_phase.1);
    let phase_paid = self.get_phase_paid(idea_phase.0, idea_phase.1);
    let active = self.get_active_phase(idea_phase.0, idea_phase.1);
    if time_passed > 259200000000 && goal_reached == false && active == true && phase_paid == false{
        log!("time passed and goal not reached");
        log!("idea id: {} and phase: {} has been closed wihtout reaching goal", idea_phase.0, idea_phase.1);
        self.return_to_investors(idea_phase.0, idea_phase.1);
        self.set_phase_paid(idea_phase.0, idea_phase.1);
        
    }else if time_passed < 259200000000{
        log!("time not passed");
    }
    }
}




//return money to investors
pub fn return_to_investors(&mut self, idea_id: IdeaId, project_phase: u8){
    //assert that caller of this function is contract owner
    assert_eq!(env::predecessor_account_id(), env::current_account_id(), "Only contract owner can call this function.");
    let investments = self.investment.keys_as_vector();
    for investment_id in investments.iter(){
        let investment = self.investment.get(&investment_id).unwrap();
        if investment.idea_id == idea_id && investment.project_phase == project_phase{
            Promise::new(investment.investor_id).transfer(investment.amount);
            log!("Funds returned to investor");
        }
    }
}






    }
    

   