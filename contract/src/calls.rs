
use crate::*;
use near_sdk::{near_bindgen, AccountId, env};


#[near_bindgen]
impl Contract {
    pub fn create_idea(&mut self, idea_id:IdeaId, metadata:IdeaMetadata, amount1:u128, amount2:u128, amount3:u128, amount4:u128) {
        assert!(self.ideas.insert(&idea_id, &metadata).is_none(), "Idea already exists");

        let project_phase_goals = [amount1, amount2, amount3, amount4];
        self.ideas.insert(&idea_id, &metadata);
        project_phase_goals.iter().enumerate().for_each(|(i, amount)| {
            // log!("Creating goal for idea {} phase {} amount {}", idea_id, i, amount) ;
            let project_phase = (i + 1) as u8;
            match project_phase {
                1 => self.create_goals(idea_id, project_phase, *amount, true),
                _ => self.create_goals(idea_id, project_phase, *amount, false),
            }
        });
    }

    pub fn create_goals(&mut self, idea_id: IdeaId, project_phase: u8, goal_amount: Balance, active: bool) {
        // log!("Creating goal for idea {} phase {} amount {}", idea_id, project_phase, goal_amount) ;  
        let mut goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
        goals.push(ProjectPhaseGoals{idea_id, project_phase, goal_amount, goal_reached: false, phase_start: env::block_timestamp(),phase_paid: false, collect_enabled: false, active });
        self.goals.insert(&idea_id, &goals);
        // log!("Goal phase {} created", project_phase);
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
            token_received: false,
        };
        self.investment.insert(&investment_id, &investment);
    
        if goal - to_be_invested <= 1000000000 {
            self.set_goal_reached(idea_id, project_phase);
            // log!("Goal reached");
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
        // log!("Goal phase {} reached, cannot edit", goal.project_phase);
        previous_goal_reached = true;
        i=i+1;

        continue;
    }else if previous_goal_reached == true{
        //assert that project phase goal is more then 0
        assert!(project_phase_goals[i] > 0, "The goal amount for the project phase must be greater than 0");
            // log!("Goal phase {} edited", goal.project_phase);
            // log!("New amount: {}", project_phase_goals[i]);
            goal.goal_amount = project_phase_goals[i];
            i=i+1; 
            previous_goal_reached = false;
            //set goal to active
            goal.active = true;
            }else{
            //  log!("Goal phase {}, cannot edit. Previous goal not reached", goal.project_phase);
            }
    }
        
    }
    self.goals.insert(&idea_id, &goals);
}


pub fn collect_funds_for_all_phases(&mut self, idea_id: IdeaId) {
    for project_phase in 1..=4 {
        // log!("Collecting funds for phase {} and idea_id {}", project_phase, idea_id);
        let result = self.collect_funds(idea_id, project_phase);
        let result_str = String::from_utf8(result).unwrap();
        // log!("Result for phase {}: {:?}", project_phase, result_str);
        // check the value of result_str
        // if it's equals to "Funds collected successfully" the funds collection for that phase succeeded
        // otherwise it contains the reason why the collection failed
    }
    // log!("Funds collection for all phases completed.");
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
    if project_phase==3{

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




//refactor upper function without pagination
pub fn check_date_all(&mut self) {
    let ideas_phases = self.get_all_ideas_and_phases();
    for idea_phase in ideas_phases.iter() {
        
        let time_passed = self.time_passed(idea_phase.0, idea_phase.1);
        let goal_reached = self.get_goal_reached(idea_phase.0, idea_phase.1);
        let phase_paid = self.get_phase_paid(idea_phase.0, idea_phase.1);
        let active = self.get_active_phase(idea_phase.0, idea_phase.1);
        if (time_passed as u64) > (7.776e15f64.round() as u64)  && goal_reached == false && active == true && phase_paid == false{
            // log!("time passed and goal not reached");
            log!("idea id: {} and phase: {} has been closed wihtout reaching goal", idea_phase.0, idea_phase.1);
            self.return_to_investors(idea_phase.0, idea_phase.1);
            self.set_phase_paid(idea_phase.0, idea_phase.1);
            
        }else if (time_passed as u64) > (7.776e15f64.round() as u64){
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
            // log!("Funds returned to investor");
        }
    }
}



//get all the investors and how much they invested by each phase in idea
pub fn get_investors(&self, idea_id: IdeaId, project_phase: u8) -> Vec<(AccountId, u128)>{
    let mut investors: Vec<(AccountId, u128)> = Vec::new();
    let investments = self.investment.keys_as_vector();
    for investment_id in investments.iter(){
        let investment = self.investment.get(&investment_id).unwrap();
        if investment.idea_id == idea_id && investment.project_phase == project_phase{
            investors.push((investment.investor_id, (investment.amount) as u128));
        }
    }
    investors
}

//get all the investors and how much they invested by each phase in idea
pub fn get_investors_sum(&self, idea_id: IdeaId, project_phase: u8) -> Vec<(AccountId, u128)>{
    let mut investors: Vec<(AccountId, u128)> = Vec::new();
    let investments = self.investment.keys_as_vector();
    for investment_id in investments.iter(){
        let investment = self.investment.get(&investment_id).unwrap();
        if investment.idea_id == idea_id && investment.project_phase == project_phase{
            let mut found = false;
            for investor in investors.iter_mut(){
                if investor.0 == investment.investor_id{
                    investor.1 += investment.amount;
                    found = true;
                }
            }
            if !found{
                investors.push((investment.investor_id, (investment.amount) as u128));
            }
        }
    }
    investors

        }

  //get sum of investemnts for idea and phase for one investor
    pub fn get_investor_sum(&self, idea_id: IdeaId, project_phase: u8, investor_id: AccountId) -> u128{
        let mut sum: u128 = 0;
        let investments = self.investment.keys_as_vector();
        for investment_id in investments.iter(){
            let investment = self.investment.get(&investment_id).unwrap();
            if investment.idea_id == idea_id && investment.project_phase == project_phase && investment.investor_id == investor_id && investment.token_received == false{
                sum += investment.amount;
            }
        }
        sum

    }



//calculate which percentage is result from get_investor_sum and get_goal, retun percentage from 0-100
    pub fn get_investor_percentage(&self, idea_id: IdeaId, project_phase: u8, investor_id: AccountId) -> f32{
        let goal = self.get_goal(idea_id, project_phase);
        let investor_sum = self.get_investor_sum(idea_id, project_phase, investor_id) as f32;
        let percentage = (investor_sum * 100.00) / ((goal*ONE_NEAR) as f32);
        percentage
    }

    // //collect token from only one investor
    // pub fn collect_token(&mut self, idea_id: IdeaId, project_phase: u8, investor_id: AccountId){
    //     //assert that caller of this function is contract owner
    //     assert_eq!(env::predecessor_account_id(), env::current_account_id(), "Only contract owner can call this function.");
    //     let percentage = self.get_investor_percentage(idea_id, project_phase, investor_id.clone());
    //     let tokens = (percentage * (self.get_goal(idea_id, project_phase) as f32)) as u128;
    //     //set all investments from this investor to token_received = true
    //     let investments = self.investment.keys_as_vector();
    //     for investment_id in investments.iter(){
    //         let mut investment = self.investment.get(&investment_id).unwrap();
    //         if investment.idea_id == idea_id && investment.project_phase == project_phase && investment.investor_id == investor_id{
    //             investment.token_received = true;
    //             self.investment.insert(&investment_id, &investment);
    //         }
    //     }
    //     Promise::new(investor_id).transfer(tokens);
    // //     log!("Tokens collected from investor");
    // }

    pub fn collect_token(&mut self, idea_id: IdeaId, project_phase: u8, investor_id: AccountId){
        //assert that caller of this function is contract owner
        assert_eq!(env::predecessor_account_id(), env::current_account_id(), "Only contract owner can call this function.");
        let percentage = self.get_investor_percentage(idea_id, project_phase, investor_id.clone());
        let tokens = (percentage * (self.get_goal(idea_id, project_phase) as f32)) as u128;
        // Collect investments that need to be updated
        let mut investments_to_update = Vec::new();
        for investment_id in self.investment.keys_as_vector().iter() {
            let investment = self.investment.get(&investment_id).unwrap();
            if investment.idea_id == idea_id && investment.project_phase == project_phase && investment.investor_id == investor_id && investment.token_received == false {
                let mut investment = investment.clone();
                investment.token_received = true;
                investments_to_update.push((investment_id, investment));
            }
        }
        // Update investments
        for (investment_id, investment) in investments_to_update.iter() {
            self.investment.insert(investment_id, investment);
        }
        //get ft metadata and transfer tokens to investor
        let contract_id = self.get_token_contract(idea_id).unwrap();
    
        let receiver_id = investor_id;
        let amount = tokens;
        // log!("contract_id: {}, receiver_id{}, amount{}", contract_id, receiver_id, amount);
        // self.transfer(contract_id, receiver_id, near_sdk::json_types::U128(amount));
        // // log!("Tokens collected from investor");
    }

  //get token_contract for idea
    pub fn get_token_contract(&self, idea_id: IdeaId) -> Option<AccountId>{
        let idea = self.ideas.get(&idea_id).unwrap();
        idea.token_contract
       
    }



    //refactor upper function to collect token for all phases
    pub fn collect_token_all_phases(&mut self, idea_id: IdeaId, investor_id: AccountId){
        //assert that caller of this function is contract owner
        assert_eq!(env::predecessor_account_id(), env::current_account_id(), "Only contract owner can call this function.");
        let mut tokens: u128 = 0;
        for phase in 1..=3{
            let percentage = self.get_investor_percentage(idea_id, phase, investor_id.clone());
            tokens += (percentage * (self.get_goal(idea_id, phase) as f32)) as u128;
            // Collect investments that need to be updated
            let mut investments_to_update = Vec::new();
            for investment_id in self.investment.keys_as_vector().iter() {
                let investment = self.investment.get(&investment_id).unwrap();
                if investment.idea_id == idea_id && investment.project_phase == phase && investment.investor_id == investor_id && investment.token_received == false {
                    let mut investment = investment.clone();
                    investment.token_received = true;
                    investments_to_update.push((investment_id, investment));
                }
            }
            // Update investments
            for (investment_id, investment) in investments_to_update.iter() {
                self.investment.insert(investment_id, investment);
            }
        }
        let contract_id = self.get_token_contract(idea_id).unwrap();
    
        let receiver_id = investor_id;
        let amount = tokens;
        // log!("contract_id: {}, receiver_id{}, amount{}", contract_id, receiver_id, amount);
        // self.transfer(contract_id, receiver_id, near_sdk::json_types::U128(amount));
        // // log!("Tokens collected from investor");
    }
    

// //collect tokens from investors
//     pub fn collect_tokens(&mut self, idea_id: IdeaId, project_phase: u8){
//         //assert that caller of this function is contract owner
//         assert_eq!(env::predecessor_account_id(), env::current_account_id(), "Only contract owner can call this function.");
//         let investors = self.get_investors(idea_id, project_phase);
//         for investor in investors.iter(){
//             let percentage = self.get_investor_percentage(idea_id, project_phase, investor.0);
//             let tokens = (percentage * (self.get_goal(idea_id, project_phase) as f32)) as u128;
//             Promise::new(investor.0).transfer(tokens);
// //             log!("Tokens collected from investor");
//         }
//     }

        

      

        pub fn get_goal(&self, idea_id: IdeaId, project_phase: u8) -> u128{
            let goals = self.goals.get(&idea_id).unwrap();
            for goal in goals.iter(){
                if goal.idea_id == idea_id && goal.project_phase == project_phase{
                    return goal.goal_amount;
                }
            }
            0
        }

        //delete idea and all related data but only if you are owner of idea
        pub fn delete_idea(&mut self, idea_id: IdeaId){
            //assert that caller of this function is contract owner
            //assert_eq!(env::predecessor_account_id(), env::current_account_id(), "Only contract owner can call this function.");
            let idea = self.ideas.get(&idea_id).unwrap();
            assert_eq!(env::predecessor_account_id(), idea.owner_id, "Only owner of idea can delete idea.");
            self.ideas.remove(&idea_id);
            self.goals.remove(&idea_id);
            self.investment.remove(&idea_id);
            // log!("Idea deleted");
        }
    }


    
    

   