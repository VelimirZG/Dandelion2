
use crate::*;

use near_sdk::{ext_contract, serde_json};


use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, Promise, PromiseResult, AccountId};

pub const TGAS: u64 = 1_000_000_000_000;
pub const NO_DEPOSIT: u128 = 0;
pub const XCC_SUCCESS: u64 = 1;


#[ext_contract(ext_ft)]
pub trait FungibleToken {
    fn ft_balance_of(&self, account_id: AccountId) -> U128;
    fn ft_transfer(&self, receiver_id: AccountId, amount: U128, memo: Option<String>) -> bool;
    fn ft_metadata(&self) -> FTmetadata;
    fn ft_total_supply(&self) -> U128;
}

#[ext_contract(ext_self)]
pub trait MyContract {
    fn balance_callback(&self, idea_id:IdeaId) -> String;
    fn transfer_callback(&self) -> String;
    fn metadata_callback(&self, idea_id:IdeaId) -> String;
    fn total_supply_callback(&self, idea_id:IdeaId) -> String;
}





#[near_bindgen]
impl Contract {
    pub fn balance_of(&self, contract_id: AccountId, account_id:AccountId, idea_id:IdeaId) -> Promise {
        //let account=account_id.clone();
        let promise = ext_ft::ext(contract_id).with_static_gas(near_sdk::Gas(5*TGAS)).ft_balance_of(account_id);
        
        return promise.then(ext_self::ext(env::current_account_id()).with_static_gas(near_sdk::Gas(5*TGAS)).balance_callback(idea_id));
    }

    pub fn balance_callback(&mut self, idea_id:IdeaId) -> U128 {
        assert_eq!(
            env::promise_results_count(),
            1,
            "This is a callback method"
        );

        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => near_sdk::json_types::U128(0),//"Cannot read from that contract, please contact support.".to_string(),
            PromiseResult::Successful(result) => {
              let balance:U128=serde_json::from_slice(&result).unwrap();
              log!("balance_json: {:?}", balance);
               self.insert_contract_supply(idea_id, balance);
               //let balance=balance.to_string();
               balance
              
            }
        }
    }

    //transfer fungible token
    pub fn transfer(&self, contract_id: AccountId, receiver_id:AccountId, amount:U128) -> Promise {
        let deposit=1;
        let promise = ext_ft::ext(contract_id).with_attached_deposit(deposit).with_static_gas(near_sdk::Gas(5*TGAS)).ft_transfer(receiver_id, amount, None);
        return promise.then(ext_self::ext(env::current_account_id()).with_static_gas(near_sdk::Gas(5*TGAS)).transfer_callback());
    }
//callback method for transfer
    pub fn transfer_callback(&self) -> String {
        assert_eq!(
            env::promise_results_count(),
            1,
            "This is a callback method"
        );

        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => "oops!".to_string(),
            PromiseResult::Successful(result) => {
            

               let balance=String::from_utf8(result).unwrap();
               balance
            }
        }
    }

     //get fungible token metadata
     pub fn metadata(&self, contract_id: AccountId, idea_id:IdeaId) -> Promise {
      let promise = ext_ft::ext(contract_id).with_static_gas(near_sdk::Gas(5*TGAS)).ft_metadata();
      return promise.then(ext_self::ext(env::current_account_id()).with_static_gas(near_sdk::Gas(5*TGAS)).metadata_callback(idea_id));
  }

  //callback method for metadata
  pub fn metadata_callback(&mut self, idea_id:IdeaId) -> String {
      assert_eq!(
          env::promise_results_count(),
          1,
          "This is a callback method"
      );

      match env::promise_result(0) {
          PromiseResult::NotReady => unreachable!(),
          PromiseResult::Failed => "oops!".to_string(),
          PromiseResult::Successful(result) => {
            //save result in Ftmetadata struct
            let metadata: FTmetadata = serde_json::from_slice(&result).unwrap();
            self.token.insert(&idea_id, &metadata);
            let metadata=String::from_utf8(result).unwrap();
            metadata

            
          }
            
            //  let balance=String::from_utf8(result).unwrap();
            //  balance
          
      }
  }

  //get total supply of fungible token
  pub fn total_supply(&self, contract_id: AccountId, idea_id:IdeaId) -> Promise {
    let promise = ext_ft::ext(contract_id).with_static_gas(near_sdk::Gas(5*TGAS)).ft_total_supply();
    return promise.then(ext_self::ext(env::current_account_id()).with_static_gas(near_sdk::Gas(5*TGAS)).total_supply_callback(idea_id));

  }

  //callback method for total supply
  pub fn total_supply_callback(&mut self, idea_id:IdeaId) -> U128 {
      assert_eq!(
          env::promise_results_count(),
          1,
          "This is a callback method"
      );

      match env::promise_result(0) {
          PromiseResult::NotReady => unreachable!(),
          PromiseResult::Failed => near_sdk::json_types::U128(0),//"oops!".to_string(),
          PromiseResult::Successful(result) => {
            let balance:U128=serde_json::from_slice(&result).unwrap();
            log!("balance_json: {:?}", balance);
            self.insert_total_supply(idea_id, balance);
             //let balance=String::from_utf8(result).unwrap();
             balance
          }
      }
  }


}
