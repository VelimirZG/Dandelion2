use crate::*;

#[near_bindgen]

impl Contract {

//get all ideas
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