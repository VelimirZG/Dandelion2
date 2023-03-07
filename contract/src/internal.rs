use crate::*;




impl Contract {
    //transfer funds to the idea owner
pub(crate) fn transfer_funds(&mut self, idea_id: IdeaId, project_phase: u8) {
    let owner_id = env::predecessor_account_id();
    let idea = self.ideas.get(&idea_id).unwrap();
    assert!(owner_id == idea.owner_id, "Only the owner of the idea can collect funds");

    let idea = self.ideas.get(&idea_id).unwrap();
    let invested_so_far = self.get_investments_by_idea_id_and_project_phase(idea_id, project_phase);
    let total_invested: Balance = invested_so_far.iter().map(|(_, investment)| investment.amount).sum();
   
    //get 0.25% of the total invested
    let fee = total_invested * 25 / 10000;
    //log!("fee: {}", fee);
    //get the total amount to be transferred to the owner
    let total_to_transfer = total_invested - fee;
    //log!("total_to_transfer: {}", total_to_transfer);
    //transfer the total amount to the owner
    Promise::new(idea.owner_id).transfer(total_to_transfer);
    //log!("Transferred {} yoctoNEAR to owner of idea", total_to_transfer);
   
}


}
