use crate::*;

#[near_bindgen]

impl Contract {

    
//get all ideas for
    pub fn get_all_ideas(&self, from_index: usize, limit: usize)->Vec<(u64,IdeaMetadata)>{
        self.ideas
        .iter()
        .skip(from_index)
        .take(limit)
        .collect()

    }

       
    

    //view idea
    pub fn get_idea(&self, idea_id: IdeaId)->Option<IdeaMetadata>{
        self.ideas.get(&idea_id)
    }

    //see if idea has active goals
    pub fn get_idea_has_active_goals(&self, idea_id: IdeaId)->bool{
        let goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
        for goal in goals.iter(){
            if goal.idea_id == idea_id && goal.goal_reached == false {
                return true;
            }
        }
        false
    }

    //iter trough all the idea and return the ones that have active goals
    pub fn get_all_ideas_with_active_goals(&self, from_index: usize, limit: usize)->Vec<(u64,IdeaMetadata)>{
        let mut ideas_with_active_goals:Vec<(u64,IdeaMetadata)> = Vec::new();
        let mut index = 0;
        for (key, idea) in self.ideas.iter(){
            if index >= from_index && index < from_index + limit {
                if self.get_idea_has_active_goals(key) == true {
                    ideas_with_active_goals.push((key, idea));
                }
            }
            index = index + 1;
        }
        ideas_with_active_goals

    }

    //get projectphasegoals for idea where goal_reached is false
    pub fn get_active_project_phase_goals(&self, idea_id: IdeaId)->Vec<ProjectPhaseGoals>{
        let mut active_project_phase_goals:Vec<ProjectPhaseGoals> = Vec::new();
        let goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
        for goal in goals.iter(){
            if goal.idea_id == idea_id && goal.goal_reached == false {
                active_project_phase_goals.push(goal.clone());
            }
        }
        active_project_phase_goals
    }   
    


    pub fn get_idea_and_goals(&self, idea_id: IdeaId)->Option<(IdeaMetadata,Vec<ProjectPhaseGoals>)>{
        let idea = self.ideas.get(&idea_id);
        let goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
        for goal in goals.iter(){
            if goal.idea_id == idea_id && goal.goal_reached == false {
                return Some((idea.unwrap(), goals));
            }
        }
        return None;
        }
    


    //get all ideas and goals
    pub fn get_all_ideas_and_goals(&self, from_index: usize, limit: usize)->Vec<(u64,IdeaMetadata,Vec<ProjectPhaseGoals>)>{
        let mut ideas_and_goals:Vec<(u64,IdeaMetadata,Vec<ProjectPhaseGoals>)> = Vec::new();
        let mut index = 0;
        for (key, idea) in self.ideas.iter(){
            if index >= from_index && index < from_index + limit {
                let goals = self.goals.get(&key).unwrap_or_else(||Vec::new());
                ideas_and_goals.push((key, idea, goals));
            }
            index = index + 1;
        }
        ideas_and_goals

    }


   


    //get all investments
    pub fn get_all_investments(&self, from_index: usize, limit: usize)->Vec<(u64,InvestmentMetadata)>{
        self.investment
        .iter()
        .skip(from_index)
        .take(limit)
        .collect()

    }
    //get all investments for an idea by project_phase
    pub fn get_investments_by_idea_id(&self, idea_id: IdeaId, project_phase: u8)->Vec<(u64,InvestmentMetadata)>{
        self.investment
        .iter()
        .filter(|(_, investment)| investment.idea_id == idea_id && investment.project_phase == project_phase)
        .collect()

    }

    //get all investments for an investor
    pub fn get_investments_by_investor_id(&self, investor_id: AccountId)->Vec<(u64,InvestmentMetadata)>{
        self.investment
        .iter()
        .filter(|(_, investment)| investment.investor_id == investor_id)
        .collect()

    }

    //get all ideas by owner_id
    pub fn get_ideas_by_owner_id(&self, owner_id: AccountId)->Vec<(u64,IdeaMetadata)>{
        self.ideas
        .iter()
        .filter(|(_, idea)| idea.owner_id == owner_id)
        .collect()

    }

    // //get all project phase goals
    // pub fn get_all_project_phase_goals(&self, from_index: usize, limit: usize)->Vec<(u64,ProjectPhaseGoals)>{
    //     self.goals
    //     .iter()
    //     .skip(from_index)
    //     .take(limit)
    //     .collect()

    // }
   

}