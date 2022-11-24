use crate::*;





#[near_bindgen]
impl Contract {
//create idea
    #[payable]
    pub fn create_idea(&mut self, idea_id:IdeaId, metadata:IdeaMetadata, amount:u128) {
        assert!(
            self.ideas.insert(&idea_id, &metadata).is_none(),
            "Idea already exists"
        );

        self.ideas.insert(&idea_id, &metadata);
        let projectphase=1;
        self.create_goals(idea_id, projectphase, amount)
    }
    
  
    #[payable]
    //invest in idea by project phase
    pub fn invest_in_idea(&mut self, idea_id: IdeaId){
       let project_phase= self.get_active_project_phase(idea_id);
       
        let investor_id = env::predecessor_account_id();
        //random number for investment id
        let investment_id = env::random_seed().iter().fold(0u64, |a, &b| (a << 8) | (b as u64));
        //get atached deposit
        let amount = env::attached_deposit();
        
        let invested=self.get_total_invested_by_idea_id(idea_id, project_phase);
        let to_be_invested=self.get_total_invested_by_idea_id(idea_id, project_phase) + amount;
        let goal=self.get_amount_by_project_phase(idea_id, project_phase)*ONE_NEAR;
        let mut still_to_invest:f32=0.0;
   
     log!("project phase: {}", project_phase);
        assert!(
            self.get_amount_by_project_phase(idea_id, project_phase) != 0,
            "The project phase for that idea is not set",
        );
        
    
            still_to_invest=(goal as f32 - invested as f32) as f32;
            
            
        assert!(
            to_be_invested <= goal,
            "The amount of the investment is greater than the goal amount, left to invest:{}", still_to_invest/ONE_NEAR as f32);
        
    
    
        //create investment
        let investment = InvestmentMetadata{
            idea_id,
            project_phase,
            investor_id,
            amount,
        };
        self.investment.insert(&investment_id, &investment);

      
    
    if ((goal as f64)/(ONE_NEAR as f64) - (invested as f64)/(ONE_NEAR as f64)) as f32 <= 0.00001 {
        self.set_goal_reached(idea_id, project_phase);
        log!("Goal reached");
    
        //TODO; enable button to collect funds and enable second phase of project
    }
    }
    
    
}