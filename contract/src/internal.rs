use crate::*;


// //refund the initial deposit based on the amount of the storage that was used by the contract
// pub(crate) fn refund_deposit(storage_used: u64) {
//     //get how much it would cost to store the information
//     let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
//     //get the attached deposit
//     let attached_deposit = env::attached_deposit();

//     //make sure that the attached deposit is greater than or equal to the required cost
//     assert!(
//         required_cost <= attached_deposit,
//         "Must attach {} yoctoNEAR to cover storage",
//         required_cost,
//     );

//     //get the refund amount from the attached deposit - required cost
//     let refund = attached_deposit - required_cost;

//     //if the refund is greater than 1 yocto NEAR, we refund the predecessor that amount
//     if refund > 1 {
//         Promise::new(env::predecessor_account_id()).transfer(refund);
//         log!("Refunded {} yoctoNEAR", refund);
//     }
// }

impl Contract {
    



//set goal reached to true
pub(crate) fn set_goal_reached(&mut self, idea_id: IdeaId, project_phase: u8) {
    let mut goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
    for goal in goals.iter_mut(){
        if goal.idea_id == idea_id && goal.project_phase == project_phase {
            goal.goal_reached = true;
            goal.collect_enabled = true;
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

//transfer funds to the idea owner
pub(crate) fn transfer_funds(&mut self, idea_id: IdeaId, project_phase: u8) {
  
    let idea = self.ideas.get(&idea_id).unwrap();
    let invested_so_far = self.get_investments_by_idea_id(idea_id, project_phase);
    let total_invested: Balance = invested_so_far.iter().map(|(_, investment)| investment.amount).sum();
    Promise::new(idea.owner_id).transfer(total_invested);
    log!("Transferred {} yoctoNEAR to owner of idea", total_invested);
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

//get time passed
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

}
