
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
   // log!("This is the amount before checking: {}", amount);
   // assert!(amount % 10000000000000000000000 == 0, "The amount cannot have more than two decimal places");



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

   //delete idea
    pub fn delete_idea(&mut self, idea_id: IdeaId) {
        //check if caller is the owner of the idea or the owner of the contract
        let caller = env::predecessor_account_id();
        let idea = self.get_idea_by_id(idea_id);
        assert!(caller == idea.owner_id || caller == env::current_account_id(), "Only the owner of the idea or the owner of the contract can delete the idea");
        self.delete_goals(idea_id);
        self.delete_investments(idea_id);
        
        assert!(
            self.ideas.remove(&idea_id).is_some(),
            "Idea does not exist"
        );
    }

    //delete all goals for the idea
    pub fn delete_goals(&mut self, idea_id: IdeaId) {
        let caller = env::predecessor_account_id();
        // let idea = self.get_idea_by_id(idea_id);
        // assert!(caller == idea.owner_id || caller == env::current_account_id(), "Only the owner of the idea or the owner of the contract can delete the idea");
        assert!(
            self.goals.remove(&idea_id).is_some(),
            "Goals do not exist"
        );

        }
//delete all investments where get_investment_id_by_idea_id
    pub fn delete_investments(&mut self, idea_id: IdeaId) {
        let caller = env::predecessor_account_id();
        // let idea = self.get_idea_by_id(idea_id);
        // assert!(caller == idea.owner_id || caller == env::current_account_id(), "Only the owner of the idea or the owner of the contract can delete the idea");
        let investment_ids = self.get_investment_id_by_idea_id(idea_id);
        for investment_id in investment_ids {
            assert!(
                self.investment.remove(&investment_id).is_some(),
                "Investment does not exist"
            );
        }
    }

    //get invetment_id from investment where idea_id in ideametadata = idea_id passed in function
    pub fn get_investment_id_by_idea_id(&self, idea_id: IdeaId) -> Vec<u64> {
        let mut investment_ids: Vec<u64> = vec![];
        for (investment_id, investment) in self.investment.iter() {
            if investment.idea_id == idea_id {
                investment_ids.push(investment_id);
            }
        }
        investment_ids  
    }
    
    }

   