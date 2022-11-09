use crate::*;


//refund the initial deposit based on the amount of the storage that was used by the contract
pub(crate) fn refund_deposit(storage_used: u64) {
    //get how much it would cost to store the information
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    //get the attached deposit
    let attached_deposit = env::attached_deposit();

    //make sure that the attached deposit is greater than or equal to the required cost
    assert!(
        required_cost <= attached_deposit,
        "Must attach {} yoctoNEAR to cover storage",
        required_cost,
    );

    //get the refund amount from the attached deposit - required cost
    let refund = attached_deposit - required_cost;

    //if the refund is greater than 1 yocto NEAR, we refund the predecessor that amount
    if refund > 1 {
        Promise::new(env::predecessor_account_id()).transfer(refund);
        log!("Refunded {} yoctoNEAR", refund);
    }
}

impl Contract {
    



//set goal reached to true
pub(crate) fn set_goal_reached(&mut self, idea_id: IdeaId, project_phase: u8) {
    let mut goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
    for goal in goals.iter_mut(){
        if goal.idea_id == idea_id && goal.project_phase == project_phase {
            goal.goal_reached = true;
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
    log!("Transferred {} yoctoNEAR", total_invested);
}
}
