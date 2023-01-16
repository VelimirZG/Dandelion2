
use crate::*;






#[near_bindgen]
impl Contract {
//create idea
    // #[payable]
    // pub fn create_idea(&mut self, idea_id:IdeaId, metadata:IdeaMetadata, amount:u128) {
    //     log!("displaying metadata: {:?}", metadata);
    //     assert!(
    //         self.ideas.insert(&idea_id, &metadata).is_none(),
    //         "Idea already exists"
    //     );

    //     self.ideas.insert(&idea_id, &metadata);
    //     let projectphase=1;
    //     self.create_goals(idea_id, projectphase, amount)
    // }

    #[payable]
    pub fn create_idea(&mut self, idea_id:IdeaId, metadata:IdeaMetadata, amount1:u128, amount2:u128, amount3:u128, amount4:u128) {
        log!("displaying metadata: {:?}", metadata);
        assert!(
            self.ideas.insert(&idea_id, &metadata).is_none(),
            "Idea already exists"
        );

        let ProjectPhaseGoals = [amount1, amount2, amount3, amount4];
        self.ideas.insert(&idea_id, &metadata);
        for i in 0..ProjectPhaseGoals.len() {
            let projectphase:u8 = (i+1).try_into().unwrap();
            self.create_goals(idea_id, projectphase, ProjectPhaseGoals[i])
        }
    }
    

 

    // //invest in idea
    // #[payable]
    // pub fn invest_in_idea(&mut self, idea_id: IdeaId){
    //     let project_phase = self.get_active_project_phase(idea_id);
    //     let investor_id = env::predecessor_account_id();
    //     let investment_id = env::random_seed().iter().fold(0u64, |a, &b| (a << 8) | (b as u64));
    //     let amount = env::attached_deposit();
    //     assert!(amount >=10000000000000000000000, "The minimum amount to invest is 0.01 NEAR");
    //     let invested = self.get_total_invested_by_idea_id(idea_id, project_phase);
    //     let to_be_invested = invested + amount;
    //     let goal = self.get_amount_by_project_phase(idea_id, project_phase) * ONE_NEAR;
    //     let still_to_invest = (goal - invested) as f32;
    
    //     assert!(self.get_amount_by_project_phase(idea_id, project_phase) != 0,
    //             "The project phase for that idea is not set");
    
    //     assert!(to_be_invested <= goal,
    //             "The amount of the investment is greater than the goal amount, left to invest: {}",
    //             still_to_invest / ONE_NEAR as f32);
    
    //     let investment = InvestmentMetadata{
    //         idea_id,
    //         project_phase,
    //         investor_id,
    //         amount,
    //     };
    //     self.investment.insert(&investment_id, &investment);
    
    //     if goal - to_be_invested <= 1000000000 {
    //         self.set_goal_reached(idea_id, project_phase);
    //         log!("Goal reached");
    //     }
    // }

    #[payable]
    pub fn invest_in_idea(&mut self, idea_id: IdeaId){
    let project_phase = self.get_active_project_phase(idea_id);
    let investor_id = env::predecessor_account_id();
    let investment_id = env::random_seed().iter().fold(0u64, |a, &b| (a << 8) | (b as u64));
    let amount = env::attached_deposit();
    //assert that everything after first three digits is 0
    assert!(amount % 10000000000000000000000 == 0, "The amount cannot have more than two decimal places");



    assert!(amount >= 10000000000000000, "The minimum amount to invest is 0.01 NEAR");
   
    let invested = self.get_total_invested_by_idea_id(idea_id, project_phase);
    let to_be_invested = invested + amount;
    let goal = self.get_amount_by_project_phase(idea_id, project_phase) * ONE_NEAR;
    let still_to_invest = (goal - invested) as f32;

    assert!(self.get_amount_by_project_phase(idea_id, project_phase) != 0,
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
    log!("displaying investment: {:?}", investment);
    self.investment.insert(&investment_id, &investment);

    if goal - to_be_invested <= 1000000000 {
        self.set_goal_reached(idea_id, project_phase);
        log!("Goal reached");
    }
}

    
  
}

