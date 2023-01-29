


use crate::*;
use near_sdk::{Balance, serde_json};
use near_sdk::{near_bindgen, AccountId, Promise, env};




#[near_bindgen]

impl Contract {
    pub fn get_all_ideas_and_goals(&self, from_index: usize, limit: usize)->Vec<(u64,IdeaMetadata,Vec<ProjectPhaseGoals>)>{
        self.ideas.iter().skip(from_index).take(limit)
            .map(|(key, value)| {
                let goals = self.goals.get(&key).unwrap_or_else(||Vec::new());
                (key, value, goals)
            }).collect()
    }

    //get total invested in idea by project phase
pub fn get_total_invested_by_idea_id(&self, idea_id: IdeaId, project_phase: u8)->Balance{
    self.investment
    .iter()
    .filter(|(_, investment)| investment.idea_id == idea_id && investment.project_phase == project_phase)
    .map(|(_, investment)| investment.amount)
    .sum()
}

//get goal by idea id and project phase
pub fn get_goal_by_project_phase(&self, idea_id: IdeaId, project_phase: u8) -> Balance {
    let goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
    let mut amount = 0;
    for goal in goals.iter(){
        if goal.idea_id == idea_id && goal.project_phase == project_phase{
            amount = goal.goal_amount;
        }
    }
    amount
}

pub fn get_active_project_phase(&self, idea_id: IdeaId) -> u8 {
    let goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
    goals.iter().find(|goal| goal.active).map(|goal| goal.project_phase).unwrap_or(0)
}

//get all ideas that have active goals and return them as a vector of JsonIdea
pub fn get_all_ideas_homepage(&self, from_index: usize, limit: usize)->Vec<JsonIdea>{
    let mut ideas_with_active_goals: Vec<JsonIdea> = Vec::new();
    let mut index = 0;

    for (key, goal) in self.goals.iter() {
        for goal in goal.iter() {
            if goal.goal_reached == false && goal.phase_paid == false && goal.active {
                if index >= from_index && index < from_index + limit {
                    log!("idea_id: {}", key);
                    let idea = self.ideas.get(&key).expect("Idea not found for given idea_id");
                    // let active_phase = self.get_active_project_phase(key.clone());
                    let investment_amount_sum: u128 = self.get_investments_by_idea_id_and_project_phase(key, goal.project_phase).iter().map(|(_, investment)| investment.amount).sum();
                    let investors_count = self.get_investors_count_by_idea_id(key);
                    let idea_metadata = idea.clone();
                    let goals_metadata = goal.clone();
                    let near = self.yocto_to_near(investment_amount_sum);
                    ideas_with_active_goals.push(JsonIdea {
                        idea_id: key,
                        title: idea_metadata.title,
                        excerpt: idea_metadata.excerpt,
                        description: idea_metadata.description,
                        competitors: idea_metadata.competitors,
                        value_proposition: idea_metadata.value_proposition,
                        tags: idea_metadata.tags,
                        team: idea_metadata.team,
                        picture_url: idea_metadata.picture_url,
                        owner_id: idea_metadata.owner_id,
                        project_phase: goals_metadata.project_phase,
                        website: idea_metadata.website,
                        goal_amount: goals_metadata.goal_amount,
                        sum: near,
                        collect_enabled: goals_metadata.collect_enabled,
                        // sum: f64::trunc((investment_amount_sum as f64/ONE_NEAR as f64)*100.0)/100.0,
                        goal_reached: goals_metadata.goal_reached,
                        phase_start: goals_metadata.phase_start,
                        investors_count: investors_count,
                    });
                }
                index += 1;
            }
        }
    }
    ideas_with_active_goals
    }

    //sort result of get_all_ideas_homepage by sum and investors count
    pub fn get_all_ideas_homepage_sorted(&self, from_index: usize, limit: usize) -> Vec<JsonIdea> {
        let mut ideas_with_active_goals: Vec<JsonIdea> = Vec::new();
        let mut index = 0;
        
        for (key, goal) in self.goals.iter() {
            for goal in goal.iter() {
                if goal.goal_reached == false && goal.phase_paid == false && goal.active {
                    if index >= from_index && index < from_index + limit {
                        let idea = self.ideas.get(&key).expect("Idea not found for given idea_id");
                        let active_phase = self.get_active_project_phase(key.clone());
                        let investment_amount_sum: u128 = self.get_investments_by_idea_id_and_project_phase(key, active_phase).iter().map(|(_, investment)| investment.amount).sum();
                        let investors_count = self.get_investors_count_by_idea_id(key);
                        let idea_metadata = idea.clone();
                        let goals_metadata = goal.clone();
                        let near = self.yocto_to_near(investment_amount_sum);
                        let new_idea = JsonIdea {
                            idea_id: key,
                            title: idea_metadata.title,
                            excerpt: idea_metadata.excerpt,
                            description: idea_metadata.description,
                            competitors: idea_metadata.competitors,
                            value_proposition: idea_metadata.value_proposition,
                            tags: idea_metadata.tags,
                            team: idea_metadata.team,
                            picture_url: idea_metadata.picture_url,
                            owner_id: idea_metadata.owner_id,
                            project_phase: goals_metadata.project_phase,
                            website: idea_metadata.website,
                            goal_amount: goals_metadata.goal_amount,
                            sum: near,
                            collect_enabled: goals_metadata.collect_enabled,
                            goal_reached: goals_metadata.goal_reached,
                            phase_start: goals_metadata.phase_start,
                            investors_count: investors_count,
                        };
                        if !ideas_with_active_goals.iter().any(|json_idea| json_idea.idea_id == new_idea.idea_id) {
                            ideas_with_active_goals.push(new_idea);
                        }
                    }
                    index += 1;
                }
            }
        }
        ideas_with_active_goals.sort_by(|a, b| b.sum.partial_cmp(&a.sum).unwrap());
        ideas_with_active_goals
        
    }

    pub fn get_all_ideas_homepage_by_investor_id2(&self, investor_id:AccountId, from_index: usize, limit: usize)->Vec<JsonIdea>{
        let mut ideas:Vec<JsonIdea> = Vec::new();
        let idea_ids = self.get_idea_ids_by_investor_id(investor_id);
        let mut index = 0;
        for idea_id in idea_ids.iter(){
            if index >= from_index && index < from_index + limit {
                let idea = self.get_idea_by_id(*idea_id);
                let goals = self.get_goals_from_idea_id(idea_id.clone()).expect("No goals found for given idea_id");
                let investors_count = self.get_investors_count_by_idea_id(*idea_id);
                let investment_amount_sum: u128 = self.get_investments_by_idea_id(*idea_id).iter().map(|(_, investment)| investment.amount).sum();
                let sum_amount = self.get_total_amount_by_idea(idea_id.clone());
                let near = self.yocto_to_near(investment_amount_sum);
                ideas.push(JsonIdea{
                    idea_id: *idea_id,
                    title: idea.title,
                    excerpt: idea.excerpt,
                    description: idea.description,
                    competitors: idea.competitors,
                    value_proposition: idea.value_proposition,
                    tags: idea.tags,
                    team: idea.team,
                    picture_url: idea.picture_url,
                    owner_id: idea.owner_id,
                    project_phase: goals.project_phase,
                    website: idea.website,
                    goal_amount: sum_amount,
                    sum: near,
                    goal_reached: goals.goal_reached,
                    phase_start: goals.phase_start,
                    investors_count: investors_count,
                    collect_enabled: goals.collect_enabled,
                });
            }
            index += 1;
        }
        ideas
    }

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
                amount = goal.goal_amount;
            }
        }
        amount
    }


      //get investments by idea_id
      pub fn get_investments_by_idea_id(&self, idea_id: IdeaId)->Vec<(u64,InvestmentMetadata)>{
        self.investment
        .iter()
        .filter(|(_, investment)| investment.idea_id == idea_id)
        .collect()

    }

       //get all the idea_ids from investments where investor_id == investor_id, results should not repeat
       pub fn get_idea_ids_by_investor_id(&self, investor_id:AccountId)->Vec<u64>{
        let mut idea_ids:Vec<u64> = Vec::new();
        for (key, investment) in self.investment.iter(){
            if investment.investor_id == investor_id {
                if idea_ids.iter().any(|i| i == &investment.idea_id){
                    continue;
                }
                idea_ids.push(investment.idea_id);
            }
        }
        idea_ids
    }

     //get goals from idea_id an return ProjectPhaseGoals
     pub fn get_goals_from_idea_id(&self, idea_id:u64)->Option<ProjectPhaseGoals>{
        let mut goals:ProjectPhaseGoals = ProjectPhaseGoals{
            idea_id: idea_id,
            project_phase: 0,
            goal_amount: 0,
            goal_reached: false,
            phase_start: 0,
            phase_paid: false,
            collect_enabled: false,
            active: false,
        };
        for (key, goal) in self.goals.iter(){
            if key == idea_id {
                for goal in goal.iter(){
                    goals = goal.clone();
                }
            }
        }
        Some(goals)
    }

    pub fn get_idea_by_id(&self, idea_id:u64)->IdeaMetadata{
        self.ideas.get(&idea_id).unwrap().clone()

    }

//get project phase goals from idea_id that are active
pub fn get_active_project_phase_goals(&self, idea_id: IdeaId) -> Vec<ProjectPhaseGoals>{
    let goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
    goals.iter().filter(|goal| goal.active).map(|goal| goal.clone()).collect()
}

 //get all investments for an idea by project_phase
 pub fn get_investments_by_idea_id_and_project_phase(&self, idea_id: IdeaId, project_phase: u8)->Vec<(u64,InvestmentMetadata)>{
    self.investment
    .iter()
    .filter(|(_, investment)| investment.idea_id == idea_id && investment.project_phase == project_phase)
    .collect()

}

pub fn get_investors_count_by_idea_id(&self, idea_id: IdeaId)->u64{
    let mut investors_count:u64 = 0;
    let mut investors:Vec<String> = Vec::new();
    let investments = self.investment
    .iter()
    .filter(|(_, investment)| investment.idea_id == idea_id)
    .collect::<Vec<(u64, InvestmentMetadata)>>();
    for (_, investment) in investments.iter(){
        if !investors.contains(&investment.investor_id.to_string()){
            investors_count = investors_count + 1;
            investors.push(investment.investor_id.to_string());
        }
    }
    investors_count
}

pub fn yocto_to_near(&self, yocto: u128) -> f64 {
    let yocto = yocto as f64;
    let near = yocto / ONE_NEAR as f64;
    f64::trunc(near * 100.0) / 100.0

}

pub fn get_idea_for_single(&self, idea_id: IdeaId) -> Option<JsonIdeaWithInvestments> {
    let idea = self.ideas.get(&idea_id);
    let idea_metadata = idea.unwrap();
    let project_phase_goals = self.goals.get(&idea_id).unwrap_or_else(|| Vec::new());
    let mut active_phase=0;

    let mut investments: Vec<Investment> = Vec::new();
    for project_phase in 1..=4 {
        let mut sum: u128 = 0;
        let mut near_sum:f64 = 0.0;
        let goal_reached = self.get_goal_reached(idea_id, project_phase);
        let investments_by_idea_id = self.get_investments_by_idea_id_and_project_phase(idea_id, project_phase);
        for (_, investment) in investments_by_idea_id.iter() {
            sum = sum + investment.amount;
            near_sum= self.yocto_to_near(sum);
        }
        let active=project_phase_goals[project_phase as usize - 1].active;
        if active==true{
            active_phase= project_phase;
        }
        investments.push(Investment {
            project_phase,
            goal: project_phase_goals[project_phase as usize - 1].goal_amount,
            sum:near_sum,
            goal_reached,
        });
    }
    log!("investments: {:?}", investments);

    Some(JsonIdeaWithInvestments {
        idea_id: idea_id,
        title: idea_metadata.title,
        excerpt: idea_metadata.excerpt,
        description: idea_metadata.description,
        competitors: idea_metadata.competitors,
        value_proposition: idea_metadata.value_proposition,
        tags: idea_metadata.tags,
        team: idea_metadata.team,
        picture_url: idea_metadata.picture_url,
        website: idea_metadata.website,
        owner_id: idea_metadata.owner_id,
        investments,
        investors_count: self.get_investors_count_by_idea_id(idea_id),
        active_phase:Some(active_phase),
    })
}

//check if phase is closed
pub fn get_phase_paid(&self, idea_id: IdeaId, project_phase: u8) -> bool{
    let goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
    let mut phase_closed = false;
    for goal in goals.iter(){
        if goal.idea_id == idea_id && goal.project_phase == project_phase{
            phase_closed = goal.phase_paid;
        }
    }
    phase_closed
}

//testing goes from here

//check balance of contract
pub fn get_balance(&self) -> u128 {
    env::account_balance()
}

//count all ideas
pub fn count_all_ideas(&self) -> u64 {
    self.ideas.len() as u64
}

//sum of ideas that investor invested in
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

//count ideas and phases that investor invested in and that reached goal
pub fn count_phases_and_ideas_by_investor_id(&self, investor_id: AccountId)->(u64, u64){
    let mut ideas_count:u64 = 0;
    let mut project_phases_count:u64 = 0;
    let mut ideas:Vec<IdeaId> = Vec::new();
    for (key, goal) in self.goals.iter(){
        for goal in goal.iter(){
            log!("Checking goal: {:?}", goal);
            if goal.goal_reached == true {
                let idea = self.ideas.get(&key).expect("Idea not found for given idea_id");
                log!("Found idea: {:?} for goal: {:?}", idea, goal);
                if !ideas.contains(&key) {
                    
                    ideas.push(key);
                }
            }
        }
    }
    log!("Ideas: {:?}", ideas);
    for idea_id in ideas.iter(){
        let investments = self.investment
        .iter()
        .filter(|(_, investment)| investment.idea_id == *idea_id)
        .collect::<Vec<(u64, InvestmentMetadata)>>();
        log!("Investments for idea: {:?} are: {:?}", idea_id, investments);
        let mut found_match = false;
        for (_, investment) in investments.iter(){
            if investment.investor_id == investor_id {
                log!("Found matching investment: {:?} for investor_id: {}", investment, investor_id);
                found_match = true;
                project_phases_count = project_phases_count + 1;
            }
            }
            if found_match {
            ideas_count = ideas_count + 1;
            }
            }
            (project_phases_count, ideas_count)
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

        //get all ideas by owner that have at least one project_reached and return number of ideas and project_phases that meet the criteria
        pub fn count_phases_and_ideas_by_owner_id(&self, owner_id: AccountId)->(u64, u64){
            let mut ideas_count:u64 = 0;
            let mut project_phases_count:u64 = 0;
            for (key, goal) in self.goals.iter(){
                for goal in goal.iter(){
                    if goal.goal_reached == true {
                        let idea = self.ideas.get(&key).expect("Idea not found for given idea_id");
                        if idea.owner_id == owner_id {
                            ideas_count = ideas_count + 1;
                            project_phases_count = project_phases_count + 1;
                        }
                    }
                }
            }
            (project_phases_count, ideas_count)
        }

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

    pub fn get_all_ideas_homepage_by_owner_id(&self, owner_id: AccountId, from_index: usize, limit: usize) -> Vec<JsonIdea> {
        let mut ideas_with_active_goals: Vec<JsonIdea> = Vec::new();
        let mut index = 0;
    
        for (key, idea) in self.ideas.iter() {
            if idea.owner_id == owner_id {
                let goals = self.get_goals_from_idea_id(key.clone());
                let investment_amount_sum: u128 = self.get_investments_by_idea_id(key).iter().map(|(_, investment)| investment.amount).sum();
                let sum_amount = self.get_total_amount_by_idea(key);

                let investors_count = self.get_investors_count_by_idea_id(key);
                let idea_metadata = idea.clone();
                if let Some(goals_metadata) = goals {
                    if index >= from_index && index < from_index + limit {
                        ideas_with_active_goals.push(JsonIdea {
                            idea_id: key,
                            title: idea_metadata.title,
                            excerpt: idea_metadata.excerpt,
                            description: idea_metadata.description,
                            competitors: idea_metadata.competitors,
                            value_proposition: idea_metadata.value_proposition,
                            tags: idea_metadata.tags,
                            team: idea_metadata.team,
                            picture_url: idea_metadata.picture_url,
                            owner_id: idea_metadata.owner_id,
                            website: idea_metadata.website,
                            project_phase: goals_metadata.project_phase,
                            goal_amount: sum_amount,
                            sum: self.yocto_to_near(investment_amount_sum),
                            goal_reached: goals_metadata.goal_reached,
                            phase_start: goals_metadata.phase_start,
                            investors_count: investors_count,
                            collect_enabled: goals_metadata.collect_enabled,
                        });
                    }
                    index += 1;
                }
            }
        }
        ideas_with_active_goals
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

//check if phase is active, inputs idea_id and phase, output bool
pub fn get_active_phase(&self, idea_id: IdeaId, project_phase: u8) -> bool{
    let goals = self.goals.get(&idea_id).unwrap();
    for goal in goals.iter(){
        if goal.idea_id == idea_id && goal.project_phase == project_phase{
            return goal.active;
        }
    }
    false

}

}
