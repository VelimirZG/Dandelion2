use crate::*;





#[near_bindgen]
impl Contract {
//create idea
    #[payable]
    pub fn create_idea(&mut self, idea_id:IdeaId, metadata:IdeaMetadata){
        assert!(
            self.ideas.insert(&idea_id, &metadata).is_none(),
            "Idea already exists"
        );

        //measuring the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        self.ideas.insert(&idea_id, &metadata);

        //calculate the required storage which was used - initial storage
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes);

    }
    
  

// //invest in idea
//     pub fn invest_in_idea(&mut self, investment_id: InvestmentId, metadata: InvestmentMetadata){
//         // Get who is calling the method and how much $NEAR they attached
//         let investor: AccountId = env::predecessor_account_id();
//         let investment_amount: Balance = env::attached_deposit();

//         //get from investments by metadata.idea_id how much is invested in that idea by project phase
//         let invested_so_far=self.get_investments_by_idea_id(metadata.idea_id, metadata.project_phase);

//         //get the total amount invested in that idea by project phase
//         let total_invested: Balance = invested_so_far.iter().map(|(_, investment)| investment.amount).sum();

//         //get ProjectPhaseGoals for that idea by project phase
//         let project_phase_goals = self.get_project_phase_goals_by_idea_id(metadata.idea_id, metadata.project_phase);

//         //check if invested so faR + INVESTMENT AMOUNT IS less or equal to the project_phase_goal
//         assert!(
//             total_invested + investment_amount <= project_phase_goals,
//             "Investment amount exceeds project phase goal"
//         );

//             assert!(
//                 self.investment.insert(&investment_id, &metadata).is_none(),
//                 "Investment already exists"
//             );

//         self.investment.insert(&investment_id, &metadata);
//     }

//     //set goals for project phase
//     pub fn set_project_phase_goals(&mut self, idea_id: IdeaId, project_phase: u8, goal: Balance){
//         assert!(
//             self.goals.insert(&(idea_id, project_phase), &goal).is_none(),
//             "Goals already exist"
//         );

//         self.goals.insert(&(idea_id, project_phase), &goal);
//     }

    
}