
use crate::*;

#[near_bindgen]

impl Contract {

    
// //get all ideas
//     pub fn get_all_ideas(&self, from_index: usize, limit: usize)->Vec<(u64,IdeaMetadata)>{
//         self.ideas
//         .iter()
//         .skip(from_index)
//         .take(limit)
//         .collect()

//     }

    // //ovo je NOVI - optimizirani kod
    // pub fn get_all_ideas_with_active_goals_and_investment_amount_sum(
    //     &self,
    //     from_index: usize,
    //     limit: usize,
    // ) -> Vec<JsonIdea> {
    //     let mut ideas_with_active_goals: Vec<JsonIdea> = Vec::new();
    //     for idea_id in self.get_all_ideas_with_active_goals_new(from_index, limit).iter() {
    //         let idea = self
    //             .ideas
    //             .get(idea_id)
    //             .expect("Idea not found for given idea_id");
    //         let goals = self
    //             .get_active_goal_from_idea_id(idea_id.clone())
    //             .expect("No active goals found for given idea_id");
    //         let active_phase = self
    //             .get_active_project_phase(idea_id.clone());
    //             //.expect("No active project phase found for given idea_id");
            
            
    //         let investment_amount_sum: u128 = self
    //             .get_investments_by_idea_id(*idea_id, active_phase)
    //             .iter()
    //             .map(|(_, investment)| investment.amount)
    //             .sum();
    //         let idea_metadata = idea.clone();
    //         let goals_metadata = goals.clone();
    //         ideas_with_active_goals.push(JsonIdea {
    //             idea_id: *idea_id,
    //             title: idea_metadata.title,
    //             excerpt: idea_metadata.excerpt,
    //             description: idea_metadata.description,
    //             competitors: idea_metadata.competitors,
    //             value_proposition: idea_metadata.value_proposition,
    //             tags: idea_metadata.tags,
    //             team: idea_metadata.team,
    //             picture_url: idea_metadata.picture_url,
    //             owner_id: idea_metadata.owner_id,
    //             project_phase: goals_metadata.project_phase,
    //             amount: goals_metadata.amount,
    //             sum: investment_amount_sum,
    //             goal_reached: goals_metadata.goal_reached,
    //             phase_start: goals_metadata.phase_start,
    //         });
    //     }
    //     ideas_with_active_goals
    // }

    //get all ideas with active goals and investment amount sum and investors count
    pub fn get_all_ideas_homepage(&self, from_index: usize,limit: usize, )->Vec<JsonIdea>{              
        let mut ideas_with_active_goals:Vec<JsonIdea> = Vec::new();
        for idea_id in self.get_all_ideas_with_active_goals_new(from_index, limit).iter(){
            let idea = self.ideas.get(idea_id).expect("Idea not found for given idea_id");
            let goals = self.get_active_goal_from_idea_id(idea_id.clone()).expect("No active goals found for given idea_id");
            let active_phase = self.get_active_project_phase(idea_id.clone());
            let investment_amount_sum:u128 = self.get_investments_by_idea_id(*idea_id, active_phase).iter().map(|(_, investment)| investment.amount).sum();
            let investors_count = self.get_investors_count_by_idea_id2(*idea_id);
            let idea_metadata = idea.clone();
            let goals_metadata = goals.clone();
            ideas_with_active_goals.push(JsonIdea{
                idea_id: *idea_id,
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
                amount: goals_metadata.amount,
                sum: investment_amount_sum,
                goal_reached: goals_metadata.goal_reached,
                phase_start: goals_metadata.phase_start,
                investors_count: investors_count,
            });
        }
        ideas_with_active_goals
    }

    
    // upper function to get ideas for only one owner
    pub fn get_all_ideas_homepage_by_owner_id(&self, owner_id:AccountId)->Vec<JsonIdea>{
        let mut ideas_with_active_goals:Vec<JsonIdea> = Vec::new();
        for idea_id in self.get_all_ideas_and_goals_id_only().iter(){
            let idea = self.ideas.get(idea_id).expect("Idea not found for given idea_id");
            if idea.owner_id == owner_id {
                let goals = self.get_active_goal_from_idea_id(idea_id.clone()).expect("No active goals found for given idea_id");
                let active_phase = self.get_active_project_phase(idea_id.clone());
                let investment_amount_sum:u128 = self.get_investments_by_idea_id(*idea_id, active_phase).iter().map(|(_, investment)| investment.amount).sum();
                let investors_count = self.get_investors_count_by_idea_id2(*idea_id);
                let idea_metadata = idea.clone();
                let goals_metadata = goals.clone();
                ideas_with_active_goals.push(JsonIdea{
                    idea_id: *idea_id,
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
                    amount: goals_metadata.amount,
                    sum: investment_amount_sum,
                    goal_reached: goals_metadata.goal_reached,
                    phase_start: goals_metadata.phase_start,
                    investors_count: investors_count,
                });
            }
        }
        ideas_with_active_goals
    }



    //get all ideas and goals and return just idea_id
    pub fn get_all_ideas_and_goals_id_only(&self)->Vec<u64>{
        let mut all_ideas:Vec<u64> = Vec::new();
        for (key, goal) in self.goals.iter(){
            for goal in goal.iter(){
               
                    all_ideas.push(key);
               
            }
        }
        all_ideas
    }






        //get all idea_ids with active goals
        pub fn get_all_ideas_with_active_goals_new(&self, from_index: usize, limit: usize)->Vec<u64>{
            let mut ideas_with_active_goals:Vec<u64> = Vec::new();
            let mut index = 0;
            for (key, goal) in self.goals.iter(){
                if index >= from_index && index < from_index + limit {
                    for goal in goal.iter(){
                        if goal.goal_reached == false && goal.phase_closed == false {
                            ideas_with_active_goals.push(key);
                        }
                    }
                }
                index = index + 1;
            }
            ideas_with_active_goals
        }

        //get other ideas that are not in upper result
        pub fn get_all_ideas_without_active_goals(&self, from_index: usize, limit: usize)->Vec<u64>{
            let mut ideas_without_active_goals:Vec<u64> = Vec::new();
            let mut index = 0;
            for (key, goal) in self.goals.iter(){
                if index >= from_index && index < from_index + limit {
                    let mut goal_reached = false;
                    for goal in goal.iter(){
                        if goal.goal_reached == false && goal.phase_closed == false {
                            goal_reached = true;
                        }
                    }
                    if goal_reached == false {
                        ideas_without_active_goals.push(key);
                    }
                }
                index = index + 1;
            }
            ideas_without_active_goals
        }

    // //get all idea_ids without active goals
    // pub fn get_all_ideas_without_active_goals(&self, from_index: usize, limit: usize)->Vec<u64>{
    //     let mut ideas_without_active_goals:Vec<u64> = Vec::new();
    //     let mut index = 0;
    //     for (key, goal) in self.goals.iter(){
    //         if index >= from_index && index < from_index + limit {
    //             let mut goal_reached = false;
    //             for goal in goal.iter(){
    //                 if goal.goal_reached == true {
    //                     goal_reached = true;
    //                 }
    //             }
    //             if goal_reached == false {
    //                 ideas_without_active_goals.push(key);
    //             }
    //         }
    //         index = index + 1;
    //     }
    //     //iterate trough result from get_all_ideas_with_active_goals_new and remove all ids from ideas_without_active_goals
    //     let ideas_with_active_goals = self.get_all_ideas_with_active_goals_new(from_index, limit);
    //     for idea_id in ideas_with_active_goals.iter(){
    //         ideas_without_active_goals.retain(|&x| &x != idea_id);
    //     }
    //     ideas_without_active_goals
    // }

       
    //get all investments for an idea by project_phase
    pub fn get_investments_by_idea_id(&self, idea_id: IdeaId, project_phase: u8)->Vec<(u64,InvestmentMetadata)>{
        self.investment
        .iter()
        .filter(|(_, investment)| investment.idea_id == idea_id && investment.project_phase == project_phase)
        .collect()

    }


    //get active goal from idea_id
    pub fn get_active_goal_from_idea_id(&self, idea_id: IdeaId)->Option<ProjectPhaseGoals>{
        let goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
        for goal in goals.iter(){
            if goal.goal_reached == false && goal.phase_closed == false {
                return Some(goal.clone());
            }
        }
        None
    }



    //same as upper function but for whole idea, not by phases and not using get_investments_by_idea_id
    pub fn get_investors_count_by_idea_id2(&self, idea_id: IdeaId)->u64{
        let mut investors_count:u64 = 0;
        let mut investors:Vec<String> = Vec::new();
        let investments = self.investment
        .iter()
        .filter(|(_, investment)| investment.idea_id == idea_id)
        .collect::<Vec<(u64, InvestmentMetadata)>>();
        for (_, investment) in investments.iter(){
            if !investors.contains(&investment.investor_id.to_string()){
                log!("investor_id: {}", investment.investor_id);
                investors_count = investors_count + 1;
                investors.push(investment.investor_id.to_string());
            }
        }
        investors_count
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

//     //view idea
//     pub fn get_idea(&self, idea_id: IdeaId)->Option<IdeaMetadata>{
//         self.ideas.get(&idea_id)
//     }

//     //see if idea has active goals
//     pub fn get_idea_has_active_goals(&self, idea_id: IdeaId)->bool{
//         let goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
//         for goal in goals.iter(){
//             if goal.idea_id == idea_id && goal.goal_reached == false {
//                 return true;
//             }
//         }
//         false
//     }

//     //iter trough all the idea and return the ones that have active goals
//     pub fn get_all_ideas_with_active_goals(&self, from_index: usize, limit: usize)->Vec<(u64,IdeaMetadata)>{
//         let mut ideas_with_active_goals:Vec<(u64,IdeaMetadata)> = Vec::new();
//         let mut index = 0;
//         for (key, idea) in self.ideas.iter(){
//             if index >= from_index && index < from_index + limit {
//                 if self.get_idea_has_active_goals(key) == true {
//                     ideas_with_active_goals.push((key, idea));
//                 }
//             }
//             index = index + 1;
//         }
//         ideas_with_active_goals

       

//     }

//     pub fn check_all_results(&mut self, idea_metadata:&Vec<(u64,IdeaMetadata)>){
//         for idea in idea_metadata.iter(){
//             let idea_id = idea.0;
//             let projectphase = self.get_active_project_phase(idea_id);
//             self.check_date(idea_id, projectphase);
//         }
//     }

//     pub fn check_all(&mut self, from_index: usize, limit: usize){
//         let ideaswithactivegoals = self.get_all_ideas_with_active_goals(from_index, limit);
       
//         for idea in ideaswithactivegoals.iter(){
//             let idea_id = idea.0;
//             let projectphase = self.get_active_project_phase(idea_id);
//             self.check_date(idea_id, projectphase);
//         }
//     }



//     pub fn get_idea_and_goals(&self, idea_id: IdeaId)->Option<(IdeaMetadata,Vec<ProjectPhaseGoals>)>{
//         let idea = self.ideas.get(&idea_id);
//         let goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
//         for goal in goals.iter(){
//             if goal.idea_id == idea_id && goal.goal_reached == false {
//                 return Some((idea.unwrap(), goals));
//             }
//         }
//         return None;
//         }
    


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

    //get all ideas and goals by owner
    pub fn get_all_ideas_and_goals_by_owner(&self, owner_id: AccountId, from_index: usize, limit: usize)->Vec<(u64,IdeaMetadata,Vec<ProjectPhaseGoals>)>{
        let mut ideas_and_goals:Vec<(u64,IdeaMetadata,Vec<ProjectPhaseGoals>)> = Vec::new();
        let mut index = 0;
        for (key, idea) in self.ideas.iter(){
            if index >= from_index && index < from_index + limit {
                if idea.owner_id == owner_id {
                    let goals = self.goals.get(&key).unwrap_or_else(||Vec::new());
                    ideas_and_goals.push((key, idea, goals));
                }
            }
            index = index + 1;
        }
        ideas_and_goals
    }

    //refactor function get_all_ideas_and_goals_by_owner to return JsonIdeaWithGoals
    pub fn get_all_ideas_and_goals_by_owner_json(
        &self,
        owner_id: AccountId,
        from_index: usize,
        limit: usize,
    ) -> Vec<JsonIdeaWithGoals> {
        let mut ideas_and_goals: Vec<JsonIdeaWithGoals> = Vec::new();
        let mut index = 0;
        let mut all_goals: Vec<ProjectPhaseGoals> = Vec::new();
        let mut project_phase: u8;
        for (key, idea) in self.ideas.iter() {
            if index >= from_index && index < from_index + limit {
                if idea.owner_id == owner_id {
                    let goals = self.goals.get(&key).unwrap_or_else(|| Vec::new());
                    for goal in goals.iter() {
                        project_phase = goal.project_phase;
                        all_goals.push(*goal);
                    }
                    let json_idea = JsonIdeaWithGoals {
                        idea_id: key,
                        title: idea.title,
                        excerpt: idea.excerpt,
                        description: idea.description,
                        owner_id: idea.owner_id,
                        competitors: idea.competitors,
                        value_proposition: idea.value_proposition,
                        tags: idea.tags,
                        team: idea.team,
                        picture_url: idea.picture_url,

                        goals: all_goals.clone(),
                    };
                    ideas_and_goals.push(json_idea);
                }
            }
            index = index + 1;
        }
        ideas_and_goals
    }

    //filter upper function by closed goals
    




    // //refactor upper function to return jsonidea
    // pub fn get_all_ideas_and_goals_by_owner_json(
    //     &self,
    //     owner_id: AccountId,
    //     from_index: usize,
    //     limit: usize,
    // ) -> Vec<JsonIdea> {
    //     let mut ideas_and_goals: Vec<JsonIdea> = Vec::new();
    //     let mut index = 0;
    //     let mut all_goals: Vec<ProjectPhaseGoals> = Vec::new();
    //     let mut project_phase: u8;
    //     for (key, idea) in self.ideas.iter() {
    //         if index >= from_index && index < from_index + limit {
    //             if idea.owner_id == owner_id {
    //                 let goals = self.goals.get(&key).unwrap_or_else(|| Vec::new());
    //                 for goal in goals.iter() {
    //                     project_phase = goal.project_phase;
    //                     all_goals.push(ProjectPhaseGoals {
    //                         idea_id: goal.idea_id,
    //                         project_phase: goal.project_phase,
    //                         goal_reached: goal.goal_reached,
    //                         amount: goal.amount,
    //                         phase_start: goal.phase_start,
    //                         phase_closed: goal.phase_closed,
    //                         collect_enabled: goal.collect_enabled,
    //                     });
    
    //                     ideas_and_goals.push(JsonIdea {
    //                         idea_id: key,
    //                         title: idea.title,
    //                         excerpt: idea.excerpt,
    //                         description: idea.description,
    //                         competitors: idea.competitors,
    //                         value_proposition: idea.value_proposition,
    //                         tags: idea.tags,
    //                         team: idea.team,
    //                         picture_url: idea.picture_url,
    //                         owner_id: idea.owner_id,
    //                         project_phase: project_phase,
    //                         amount: all_goals
    //                             .iter()
    //                             .filter(|x| x.project_phase == project_phase)
    //                             .map(|x| x.amount)
    //                             .sum(),
    //                         goal_reached: all_goals
    //                             .iter()
    //                             .filter(|x| x.project_phase == project_phase)
    //                             .map(|x| x.goal_reached)
    //                             .sum(),
    //                         phase_start: all_goals
    //                             .iter()
    //                             .filter(|x| x.project_phase == project_phase)
    //                             .map(|x| x.phase_start)
    //                             .sum(),
                            
    //                         // investors_count: all_goals
    //                         //     .iter()
    //                         //     .filter(|x| x.project_phase == project_phase)
    //                         //     .map(|x| x.investors_count)
    //                         //     .sum(),
    //                     });
    //                 }
    //             }
    //         }
    //         index = index + 1;
    //     }
    //     ideas_and_goals
    // }
    
    

  
//     //get all ideas and goals with active goals
//     pub fn get_all_ideas_and_goals_with_active_goals(&self, from_index: usize, limit: usize)->Vec<(u64,IdeaMetadata,Vec<ProjectPhaseGoals>)>{
//         let mut ideas_and_goals:Vec<(u64,IdeaMetadata,Vec<ProjectPhaseGoals>)> = Vec::new();
//         let mut index = 0;
//         for (key, idea) in self.ideas.iter(){
//             if index >= from_index && index < from_index + limit {
//                 let goals = self.goals.get(&key).unwrap_or_else(||Vec::new());
//                 ideas_and_goals.push((key, idea, goals));
//             }
//             index = index + 1;
//         }
//         ideas_and_goals
//     }

//     //use get_all_ideas_with_active_goals and return JsonIdeaWithGoals
//     pub fn get_all_ideas_with_active_goals_json(&self, from_index: usize, limit: usize)->Vec<JsonIdeaWithGoals>{
//         let mut ideas_with_active_goals:Vec<JsonIdeaWithGoals> = Vec::new();
//         let mut index = 0;
//         for (key, idea) in self.ideas.iter(){
//             if index >= from_index && index < from_index + limit {
//                 if self.get_idea_has_active_goals(key) == true {
//                     let goals = self.goals.get(&key).unwrap_or_else(||Vec::new());
//                     let jsonidea = JsonIdeaWithGoals{
//                         idea_id: key,
//                         title: idea.title,
//                         excerpt: idea.excerpt,
//                         description: idea.description,
//                         competitors: idea.competitors,
//                         value_proposition: idea.value_proposition,
//                         tags: idea.tags,
//                         team: idea.team,
//                         picture_url: idea.picture_url,
//                         owner_id: idea.owner_id,
//                         goals: goals,
//                     };
//                     ideas_with_active_goals.push(jsonidea);
//                 }
//             }
//             index = index + 1;
//         }
//         ideas_with_active_goals
//     }

  



//     //delete project_phase
//     pub fn delete_project_phase(&mut self, idea_id: IdeaId, project_phase: u8){
//         //assert that the owner of the idea is the caller
//         let idea = self.ideas.get(&idea_id).unwrap();
//         assert_eq!(env::predecessor_account_id(), idea.owner_id, "Only the owner of the idea can delete a project phase");
//         let mut goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
//         let mut index = 0;
//         let mut goal_index = 0;
//         for goal in goals.iter(){
//             if goal.idea_id == idea_id && goal.project_phase == project_phase {
//                 goal_index = index;
//             }
//             index = index + 1;
//         }
//         goals.remove(goal_index);
//         self.goals.insert(&idea_id, &goals);
//     }







//     // //get goals for idea with active goals
//     // pub fn get_goals_for_idea(&self, idea_id: IdeaId)->Vec<Goal>{
//     //     let goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
//     //     let mut active_goals:Vec<Goal> = Vec::new();
//     //     for goal in goals.iter(){
//     //         if goal.idea_id == idea_id && goal.goal_reached == false {
//     //             active_goals.push(goal.clone());
//     //         }
//     //     }
//     //     active_goals
//     // }

//     // //get all goals for idea
//     // pub fn get_all_goals_for_idea(&self, idea_id: IdeaId)->Vec<Goal>{
//     //     let goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
//     //     goals
//     // }



//     //get all investments
//     pub fn get_all_investments(&self, from_index: usize, limit: usize)->Vec<(u64,InvestmentMetadata)>{
//         self.investment
//         .iter()
//         .skip(from_index)
//         .take(limit)
//         .collect()

//     }


//     //get all investments for an investor
//     pub fn get_investments_by_investor_id(&self, investor_id: AccountId)->Vec<(u64,InvestmentMetadata)>{
//         self.investment
//         .iter()
//         .filter(|(_, investment)| investment.investor_id == investor_id)
//         .collect()

//     }

//     //get all ideas by owner_id
//     pub fn get_ideas_by_owner_id(&self, owner_id: AccountId)->Vec<(u64,IdeaMetadata)>{
//         self.ideas
//         .iter()
//         .filter(|(_, idea)| idea.owner_id == owner_id)
//         .collect()

//     }



  



//      //filter all idea_ids with active goals and return idea_id and project_phase
//         pub fn get_all_ideas_with_active_goals_and_project_phase(&self, from_index: usize, limit: usize)->Vec<(u64,u8)>{
//             let mut ideas_with_active_goals:Vec<(u64,u8)> = Vec::new();
//             let mut index = 0;
//             for (key, goal) in self.goals.iter(){
//                 if index >= from_index && index < from_index + limit {
//                     for goal in goal.iter(){
//                         if goal.goal_reached == false && goal.phase_closed == false {
//                             ideas_with_active_goals.push((key,goal.project_phase));
//                         }
//                     }
//                 }
//                 index = index + 1;
//             }
//             ideas_with_active_goals
//         }




//     //call the get_all_ideas_with_active_goals_new and use result as input for get_idea_and_goals
//     pub fn get_all_ideas_with_active_goals_and_goals(&self, from_index: usize, limit: usize)->Vec<(u64,IdeaMetadata,Vec<ProjectPhaseGoals>)>{
//         let mut ideas_with_active_goals:Vec<(u64,IdeaMetadata,Vec<ProjectPhaseGoals>)> = Vec::new();
//         for idea_id in self.get_all_ideas_with_active_goals_new(from_index, limit).iter(){
//             let idea = self.ideas.get(&idea_id);
//             let goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
//             ideas_with_active_goals.push((*idea_id, idea.unwrap(), goals));
//         }
//         ideas_with_active_goals
//     }

//     //call the get_all_ideas_with_active_goals_new and use result to sum all the investments by project_phase
//     pub fn get_all_ideas_with_active_goals_and_investments(&self, from_index: usize, limit: usize)->Vec<(u64,IdeaMetadata,Vec<(u8,u128)>)>{
//         let mut ideas_with_active_goals:Vec<(u64,IdeaMetadata,Vec<(u8,u128)>)> = Vec::new();
//         for idea_id in self.get_all_ideas_with_active_goals_new(from_index, limit).iter(){
//             let idea = self.ideas.get(&idea_id);
//             let mut investments:Vec<(u8,u128)> = Vec::new();
//             let active_phase= self.get_active_project_phase_for_idea(*idea_id);
//             for project_phase in 1..=active_phase {
//                 let mut sum:u128 = 0;
//                 let investments_by_idea_id = self.get_investments_by_idea_id(*idea_id, project_phase);
//                 for (_, investment) in investments_by_idea_id.iter(){
//                     sum = sum + investment.amount;
//                 }
//                 investments.push((project_phase,sum));
//             }
//             ideas_with_active_goals.push((*idea_id, idea.unwrap(), investments));
//         }
//         ideas_with_active_goals
//     }

//     //refactor get_all_ideas_with_active_goals_and_investments to return  JsonIdeaWithInvestments
//     // pub fn get_all_ideas_with_active_goals_and_investments_json(&self, from_index: usize, limit: usize)->Vec<JsonIdeaWithInvestments>{
//     //     let mut ideas_with_active_goals:Vec<JsonIdeaWithInvestments> = Vec::new();
//     //     let mut index = 0;
//     //     for idea_id in self.get_all_ideas_with_active_goals_new(from_index, limit).iter(){
//     //         let idea = self.ideas.get(&idea_id);
//     //         let mut investments:Vec<(u8,u128)> = Vec::new();
//     //         let active_phase= self.get_active_project_phase_for_idea(*idea_id);
//     //         for project_phase in 1..=active_phase {
//     //             let mut sum:u128 = 0;
//     //             let investments_by_idea_id = self.get_investments_by_idea_id(*idea_id, project_phase);
//     //             for (_, investment) in investments_by_idea_id.iter(){
//     //                 sum = sum + investment.amount;
//     //             }
//     //             investments.push((project_phase,sum));
//     //         }
//     //         ideas_with_active_goals.push(JsonIdeaWithInvestments{
//     //             idea_id: *idea_id,
//     //             idea: idea.unwrap(),
//     //             investments: investments
//     //         });
//     //     }
//     //     ideas_with_active_goals
//     // }

//     pub fn get_all_ideas_that_have_active_goals_and_investments_json(&self, from_index: usize, limit: usize) -> Vec<JsonIdeaWithInvestments> {
//         let mut ideas_with_active_goals: Vec<JsonIdeaWithInvestments> = Vec::new();
    
//         for idea_id in self.get_all_ideas_with_active_goals_new(from_index, limit).iter() {
//             let idea = self.ideas.get(&idea_id);
//             let idea_metadata = idea.unwrap();
//             let project_phase_goals = self.goals.get(&idea_id).unwrap_or_else(|| Vec::new());
//             let active_phase = self.get_active_project_phase_for_idea(*idea_id);
    
//             let mut investments: Vec<Investment> = Vec::new();
//             for project_phase in 1..=active_phase {
//                 let mut sum: u128 = 0;
//                 let investments_by_idea_id = self.get_investments_by_idea_id(*idea_id, project_phase);
//                 for (_, investment) in investments_by_idea_id.iter() {
//                     sum = sum + investment.amount;
//                 }
    
//                 investments.push(Investment {
//                     project_phase,
//                     goal: project_phase_goals[project_phase as usize - 1].amount,
//                     sum,
//                 });
//             }
    
//             ideas_with_active_goals.push(JsonIdeaWithInvestments {
//                 idea_id: *idea_id,
//                 title: idea_metadata.title,
//                 excerpt: idea_metadata.excerpt,
//                 description: idea_metadata.description,
//                 competitors: idea_metadata.competitors,
//                 value_proposition: idea_metadata.value_proposition,
//                 tags: idea_metadata.tags,
//                 team: idea_metadata.team,
//                 picture_url: idea_metadata.picture_url,
//                 owner_id: idea_metadata.owner_id,
//                 investments,
//             });
//         }
    
//         ideas_with_active_goals
//     }
    
    // pub fn get_idea_with_active_goals_and_investments_for_idea_json(&self, idea_id: IdeaId) -> Option<JsonIdeaWithInvestments> {
    //     let idea = self.ideas.get(&idea_id);
    //     let idea_metadata = idea.unwrap();
    //     let project_phase_goals = self.goals.get(&idea_id).unwrap_or_else(|| Vec::new());
    //     let active_phase = self.get_active_project_phase(idea_id);
    
    //     let mut investments: Vec<Investment> = Vec::new();
    //     for project_phase in 1..=active_phase {
    //         let mut sum: u128 = 0;
    //         let investments_by_idea_id = self.get_investments_by_idea_id(idea_id, project_phase);
    //         for (_, investment) in investments_by_idea_id.iter() {
    //             sum = sum + investment.amount;
    //         }
    
    //         investments.push(Investment {
    //             project_phase,
    //             goal: project_phase_goals[project_phase as usize - 1].amount,
    //             sum,
    //         });
    //     }
    
    //     Some(JsonIdeaWithInvestments {
    //         idea_id: idea_id,
    //         title: idea_metadata.title,
    //         excerpt: idea_metadata.excerpt,
    //         description: idea_metadata.description,
    //         competitors: idea_metadata.competitors,
    //         value_proposition: idea_metadata.value_proposition,
    //         tags: idea_metadata.tags,
    //         team: idea_metadata.team,
    //         picture_url: idea_metadata.picture_url,
    //         owner_id: idea_metadata.owner_id,
    //         investments,
    //     })
    // }

    //refactor upper idea with investor count for idea
    pub fn get_idea_for_single(&self, idea_id: IdeaId) -> Option<JsonIdeaWithInvestments> {
        let idea = self.ideas.get(&idea_id);
        let idea_metadata = idea.unwrap();
        let project_phase_goals = self.goals.get(&idea_id).unwrap_or_else(|| Vec::new());
        let active_phase = self.get_active_project_phase(idea_id);
    
        let mut investments: Vec<Investment> = Vec::new();
        for project_phase in 1..=active_phase {
            let mut sum: u128 = 0;
            let investments_by_idea_id = self.get_investments_by_idea_id(idea_id, project_phase);
            for (_, investment) in investments_by_idea_id.iter() {
                sum = sum + investment.amount;
            }
    
            investments.push(Investment {
                project_phase,
                goal: project_phase_goals[project_phase as usize - 1].amount,
                sum,
            });
        }
    
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
            owner_id: idea_metadata.owner_id,
            investments,
            investors_count: self.get_investors_count_by_idea_id2(idea_id),
        })
    }
    

//     //get active goals from idea_id
//     pub fn get_active_goals_from_idea_id(&self, idea_id: IdeaId)->Vec<ProjectPhaseGoals>{
//         let mut active_goals:Vec<ProjectPhaseGoals> = Vec::new();
//         let goals = self.goals.get(&idea_id).unwrap_or_else(||Vec::new());
//         for goal in goals.iter(){
//             if goal.goal_reached == false && goal.phase_closed == false {
//                 active_goals.push(goal.clone());
//             }
//         }
//         active_goals
//     }


  
// //ovo je STARI - neoptimizirani kod
//     // pub fn get_all_ideas_with_active_goals_and_goals_and_investment_amount_sum(&self, from_index: usize, limit: usize)->Vec<JsonIdea>{
//     //     let mut ideas_with_active_goals:Vec<JsonIdea> = Vec::new();
//     //     let mut index = 0;
//     //     for idea_id in self.get_all_ideas_with_active_goals_new(from_index, limit).iter(){
//     //         let idea = self.ideas.get(&idea_id);
//     //         let goals = self.get_active_goal_from_idea_id(idea_id.clone());
//     //         let mut investment_amount_sum:u128 = 0;
//     //         let active_phase=self.get_active_project_phase(idea_id.clone());
//     //         for investment in self.get_investments_by_idea_id(*idea_id, active_phase).iter(){
//     //             investment_amount_sum = investment_amount_sum + investment.1.amount;
//     //         }
//     //         let idea_metadata = idea.unwrap();
//     //         let goals_metadata = goals.unwrap();
//     //         ideas_with_active_goals
//     //             .push(JsonIdea{idea_id:*idea_id, title:Some(idea_metadata.title).unwrap(), excerpt:Some(idea_metadata.excerpt).unwrap(), description:Some(idea_metadata.description).unwrap(), 
//     //                     competitors:Some(idea_metadata.competitors).unwrap(), value_proposition:Some(idea_metadata.value_proposition).unwrap(), tags:Some(idea_metadata.tags).unwrap(), 
//     //                     team:Some(idea_metadata.team).unwrap(),picture_url:Some(idea_metadata.picture_url).unwrap(), owner_id:Some(idea_metadata.owner_id).unwrap(),  
//     //                     project_phase:goals_metadata.project_phase, amount:goals_metadata.amount, sum:investment_amount_sum});
//     //     }
//     //     ideas_with_active_goals
//     // }





//     // pub fn get_investments(&self, from_index: usize, limit: usize ) -> Vec<(u64,u128)>{ {
//     //     let results=self.get_all_ideas_with_active_goals_and_goals(from_index, limit);
//     //     let mut active_ideas:Vec<u64> = Vec::new();
//     //     for result in results.iter(){
//     //         let idea_id = result.0;
//     //         active_ideas.push(idea_id);
//     //         }
//     //     let mut investments:Vec<(u64,InvestmentMetadata)> = Vec::new();
//     //     for idea_id in active_ideas.iter(){
//     //         self.investment
//     //         .iter()
//     //         .filter(|(_, investment)| investment.idea_id == *idea_id)
//     //         .for_each(|(key, investment)| investments.push((key, investment)));
//     //     }
//     //     log!("is investments empty: {}", investments.is_empty());
//     //     for i in investments.iter(){
//     //         log!("usao je u petlju investments");
//     //         let idea_id = i.1.idea_id;
//     //         let amount = i.1.amount;
//     //         let mut total_amount:u128 = 0;
//     //             for i in investments.iter(){
//     //                 if i.1.idea_id == idea_id {
//     //                     total_amount = total_amount + i.1.amount;
                        
//     //                 }
//     //             }
//     //     }

//     //     let mut sum_investments:Vec<(u64, u128)> = Vec::new();
//     //     for idea_id in active_ideas.iter(){
            
//     //         let mut sum:u128 = 0;
//     //         for investment in investments.iter(){
//     //             log!("usao u petlju");
//     //             if investment.1.idea_id == *idea_id {
                   
//     //                 sum = sum + investment.1.amount;
//     //                 log!("sum: {}", sum);
                    
//     //             }else
//     //             {
//     //                 log!("no match");
//     //             }
//     //         }
//     //         sum_investments.push((*idea_id, sum));
//     //     }
//     //     //return together results and sum_investments
//     //     sum_investments

//     // }}

//     //  //get investments and return together result and sum investments
//     //  pub fn get_investments_and_sum(&self, from_index: usize, limit: usize ) -> Vec<(u64,u128,Vec<(u64, InvestmentMetadata)>)>{ {
//     //     let results=self.get_all_ideas_with_active_goals_and_goals(from_index, limit);
//     //     let mut active_ideas:Vec<u64> = Vec::new();
//     //     let mut investments:Vec<(u64, InvestmentMetadata)> = Vec::new();
//     //     for result in results.iter(){
//     //         let idea_id = result.0;
//     //         active_ideas.push(idea_id);
//     //         }
//     //     for idea_id in active_ideas.iter(){
//     //         self.investment
//     //         .iter()
//     //         .filter(|(_, investment)| investment.idea_id == *idea_id)
//     //         .for_each(|(key, investment)| investments.push((key, investment)));
//     //     }
//     //     //get sum of all investments for each idea

//     //     let mut sum_investments:Vec<(u64, u128)> = Vec::new();
//     //     for idea_id in active_ideas.iter(){
//     //         let mut sum:u128 = 0;
//     //         for investment in investments.iter(){
//     //             if investment.1.idea_id == *idea_id {
//     //                 sum = sum + investment.1.amount;
//     //             }
//     //         }
//     //         sum_investments.push((*idea_id, sum));
//     //     }

//     //     let mut result:Vec<(u64,u128,Vec<(u64, InvestmentMetadata)>)> = Vec::new();
//     //     for idea_id in active_ideas.iter(){
//     //         let mut investments_for_idea:Vec<(u64, InvestmentMetadata)> = Vec::new();
//     //         for investment in investments.iter(){
//     //             if investment.1.idea_id == *idea_id {
//     //                 investments_for_idea.push(investment.clone());
//     //             }
//     //         }
//     //         for sum in sum_investments.iter(){
//     //             if sum.0 == *idea_id {
//     //                 result.push((*idea_id, sum.1, investments_for_idea.clone()));
//     //             }
//     //         }
//     //     }
        
//     //     result
//     //     }
//     // }}
    
//     //get investments and return together result and sum investments
//      //pub fn get_investments_and_sum(&self, from_index: usize, limit: usize ) -> Vec<(u64,u128,Vec<(u64, InvestmentMetadata)>)>{ {
//         pub fn get_investments_and_sum(&self, from_index: usize, limit: usize ){ {
//             let results=self.get_all_ideas_with_active_goals_and_goals(from_index, limit);
//             let mut active_ideas:Vec<u64> = Vec::new();
//             let mut investments:Vec<(u64, InvestmentMetadata)> = Vec::new();
//             for result in results.iter(){
//                 let idea_id = result.0;
//                 active_ideas.push(idea_id);
//                 }
//             for idea_id in active_ideas.iter(){
//                 self.investment
//                 .iter()
//                 .filter(|(_, investment)| investment.idea_id == *idea_id)
//                 .for_each(|(key, investment)| investments.push((key, investment)));
//             }
//             //get sum of all investments for each idea
    
//             let mut sum_investments:Vec<(u64, u128)> = Vec::new();
//             for idea_id in active_ideas.iter(){
//                 let mut sum:u128 = 0;
//                 for investment in investments.iter(){
//                     if investment.1.idea_id == *idea_id {
//                         sum = sum + investment.1.amount;
//                     }
//                 }
//                 sum_investments.push((*idea_id, sum));
//             }
    
//             let mut result:Vec<(u64,u128,Vec<(u64, InvestmentMetadata)>)> = Vec::new();
//             for idea_id in active_ideas.iter(){
//                 let mut investments_for_idea:Vec<(u64, InvestmentMetadata)> = Vec::new();
//                 for investment in investments.iter(){
//                     if investment.1.idea_id == *idea_id {
//                         investments_for_idea.push(investment.clone());
//                     }
//                 }
//                 for sum in sum_investments.iter(){
//                     if sum.0 == *idea_id {
//                         result.push((*idea_id, sum.1, investments_for_idea.clone()));
//                     }
//                 }
//             }
            
//             for i in result.iter(){
//                 for j in i.2.iter(){
//                     log!("idea_id: {}, sum: {}, investment_id: {}, amount: {}, investor:{}", i.0, i.1, j.0, j.1.amount, j.1.investor_id);
//                 }
                
//             //result
//             }
//         }}

//         //fetch all ideas with active goals and goals
//     // pub fn get_all_ideas_with_active_goals_and_all(&self, from_index: usize, limit: usize) -> Vec<(u64, IdeaMetadata, Vec<ProjectPhaseGoals>, u64, u128)> {
//     //     let mut results: Vec<(u64, IdeaMetadata, Vec<ProjectPhaseGoals>, u64, u128)> = Vec::new();
//     //     let ideas=self.get_all_ideas_with_active_goals_and_goals(from_index, limit);
//     //     let sum_investments=self.get_investments(from_index, limit);
//     //     //connect ideas with sum_investments in result
//     //     for idea in ideas.iter(){
//     //         for sum in sum_investments.iter(){
//     //             if idea.0 == sum.0 {
//     //                 results.push((idea.0, idea.1.clone(), idea.2.clone(), sum.0, sum.1));
//     //             }
//     //         }
            
//     //     }
//     //     for i in results.iter(){
//     //             for j in i.2.iter(){
//     //                 log!("idea_id: {}, idea_metadata: {:?}, amount: {}, goal reached: {}", i.0, i.1, j.amount, j.goal_reached);
//     //             }
            
//     //     }
        
//     //     results
//     // }


//     // //get idea_metadata using result of get_all_active_project_phase_goals
//     // pub fn get_idea_metadata_from_active_project_phase_goals(&self, active_project_phase_goals:Vec<(u64,ProjectPhaseGoals)>)->Vec<(u64,IdeaMetadata)>{
//     //     let mut ideas:Vec<(u64,IdeaMetadata)> = Vec::new();
//     //     for goal in active_project_phase_goals.iter(){
//     //         let idea_id = goal.1.idea_id;
//     //         let idea = self.ideas.get(&idea_id);
//     //         ideas.push((idea_id, idea.unwrap()));
//     //     }
//     //     ideas
//     // }



}
