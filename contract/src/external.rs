
use crate::*;

use near_sdk::{ext_contract};


use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, Promise, PromiseResult, AccountId};

pub const TGAS: u64 = 1_000_000_000_000;
pub const NO_DEPOSIT: u128 = 0;
pub const XCC_SUCCESS: u64 = 1;


#[ext_contract(ext_ft)]
pub trait FungibleToken {
    fn ft_balance_of(&self, account_id: AccountId) -> U128;
    fn ft_transfer(&self, receiver_id: AccountId, amount: U128, memo: Option<String>) -> bool;
}

#[ext_contract(ext_self)]
pub trait MyContract {
    fn my_callback(&self) -> String;
}



#[near_bindgen]
impl Contract {
    pub fn balance_of(&self, contract_id: AccountId, account_id:AccountId) -> Promise {
        //let account=account_id.clone();
        let promise = ext_ft::ext(contract_id).with_static_gas(near_sdk::Gas(5*TGAS)).ft_balance_of(account_id);
        
        return promise.then(ext_self::ext(env::current_account_id()).with_static_gas(near_sdk::Gas(5*TGAS)).my_callback());
    }

    pub fn my_callback(&self) -> String {
        assert_eq!(
            env::promise_results_count(),
            1,
            "This is a callback method"
        );

        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => "Cannot read from that contract, please contact support.".to_string(),
            PromiseResult::Successful(result) => {
               let balance=String::from_utf8(result).unwrap();
               balance
            }
        }
    }

    //transfer fungible token
    pub fn transfer(&self, contract_id: AccountId, receiver_id:AccountId, amount:U128) -> Promise {
        let deposit=1;
        let promise = ext_ft::ext(contract_id).with_attached_deposit(deposit).with_static_gas(near_sdk::Gas(5*TGAS)).ft_transfer(receiver_id, amount, None);
        promise
    }
//callback method for transfer
    pub fn my_callback2(&self) -> String {
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
}
