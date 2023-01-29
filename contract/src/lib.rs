


use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap};
use near_sdk::{log, near_bindgen, Promise, env, Balance};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk:: AccountId;

pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;
pub const ONE_NEAR: Balance = 1_000_000_000_000_000_000_000_000;

pub type Date = u64;
pub type IdeaId=u64;
pub type InvestmentId=u64;

pub use crate::metadata::*;
pub use crate::calls::*;
pub use crate::internal::*;
pub use crate::views::*;
pub use crate::external::*;



mod calls;
mod views;
mod internal;
mod metadata;
mod external;



// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub ideas: UnorderedMap<IdeaId, IdeaMetadata>,
    pub investment: UnorderedMap<InvestmentId, InvestmentMetadata>,
    pub goals:UnorderedMap<IdeaId,Vec<ProjectPhaseGoals>>,   
    pub other: UnorderedMap<IdeaId, String>, //empty map

}



// Define the default, which automatically initializes the contract
impl Default for Contract{
    fn default() -> Self{
        Self{
            ideas: UnorderedMap::new(b"s".to_vec()),
            investment: UnorderedMap::new(b"i".to_vec()),
            goals: UnorderedMap::new(b"g".to_vec()),
            other: UnorderedMap::new(b"o".to_vec()),
          
        }
    }
}

#[near_bindgen]
impl Contract {

    
   
    
    
}



























 





