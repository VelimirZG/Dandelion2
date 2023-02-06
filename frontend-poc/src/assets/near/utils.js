import { connect, Contract, keyStores, WalletConnection } from 'near-api-js'
import getConfig from './config'


const nearConfig = getConfig('development');
console.log('CONFIG: ', nearConfig);

// Initialize contract & set global variables
export async function initContract() {
  console.log('OBJECT ASSIGN: ', Object.assign({ keyStore: new keyStores.BrowserLocalStorageKeyStore() }, nearConfig));
  const near = await connect(Object.assign({ keyStore: new keyStores.BrowserLocalStorageKeyStore() }, nearConfig))
  
  // Initializing Wallet based Account. It can work with NEAR testnet wallet that
  // is hosted at https://wallet.testnet.near.org
  window.walletConnection = new WalletConnection(near)

  // Getting the Account ID. If still unauthorized, it's just empty string
  window.accountId = window.walletConnection.getAccountId()

  // Initializing our contract APIs by contract name and configuration
  window.contract = await new Contract(window.walletConnection.account(), nearConfig.contractName, {
    // View methods are read only. They don't modify the state, but usually return some value.
    viewMethods: ['get_all_inactive_ideas_homepage_by_owner_id', 'get_all_active_ideas_homepage_by_owner_id', 'count_ideas_by_owner_id', 'count_all_ideas', 'get_all_ideas_homepage_by_investor_id2', 'get_invested_ideas_count', 'count_phases_and_ideas_by_investor_id', 'get_sum_of_amount_for_investor', 'get_all_ideas_homepage_sorted', 'count_phases_and_ideas_by_owner_id', 'get_investor_count_for_owner', 'get_sum_of_amount_for_owner', 'get_all_ideas_homepage_by_owner_id', 'ideas_for_owner', 'get_idea_for_single', 'get_investments', 'total_investments', 'get_all_ideas_homepage', 'get_investment_goal', 'get_investment_for_idea'],
    // Change methods can modify the state. But you don't receive the returned value when called.
    changeMethods: ['edit_idea', 'create_idea', 'invest_in_idea', 'add_like_to_idea', 'collect_funds_for_all_phases'],
  })
}

export function logout() {
  window.walletConnection.signOut()
  // reload page
  window.location.replace(window.location.origin + window.location.pathname)
}

export function login() {
  // Allow the current app to make calls to the specified contract on the
  // user's behalf.
  // This works by creating a new access key for the user's account and storing
  // the private key in localStorage.
  window.walletConnection.requestSignIn(nearConfig.contractName)
}

export async function create_idea(data){
  let response = await window.contract.create_idea({
    args: data,
    contractId: window.contract.contractId,
    callbackUrl: window.location.origin + '/profile'
  })
  return response
}

export async function editIdea(data){
  let response = await window.contract.edit_idea({
    args: data,
    contractId: window.contract.contractId
  })
  return response
}

export async function ideas_for_owner(){
  const owner_ideas = await window.contract.ideas_for_owner({account_id: "dandelion_test.testnet"})
  return owner_ideas
}

export async function getIdea(ideaId){
  const idea = await window.contract.get_idea_for_single({idea_id: parseInt(ideaId)})
  return idea
}

export async function get_investment_for_idea(ideaId){
  const investments = await window.contract.get_investment_for_idea({idea_id: ideaId})
  return investments
}

export async function total_investments(){
  const total_investments = await window.contract.total_investments()
  return total_investments
}
export async function get_all_ideas(index, limit){
  const all_ideas = await window.contract.get_all_ideas_homepage({"from_index": index, "limit": limit})
  return all_ideas;
}

export async function get_all_ideas_homepage_sorted(index, limit){
  const all_ideas = await window.contract.get_all_ideas_homepage_sorted({"from_index": index, "limit": limit})
  return all_ideas;
}

export async function get_investment_goal(ideaId){
  const inv_goal = await window.contract.get_investment_goal({idea_id: ideaId})
  return inv_goal;
}

export async function invest(data){
  console.log('DATA: ', data);
  let gas = 300000000000000;
  // const invested = await window.contract.invest_in_idea({account_id: data.acc, idea_id: data.ideaId}, gas.toLocaleString('fullwide', {useGrouping:false}) , data.value.toLocaleString('fullwide', {useGrouping:false}));
  const invested = await window.contract.invest_in_idea({
      args: {
        account_id: data.acc, 
        idea_id: data.ideaId
      },
      amount: String(data.value)
    }
  );
  return invested;
}

// export async function invest(data){

//   const invested = await window.contract.invest_in_idea({
//     account_id: data.acc, 
//     idea_id: data.ideaId
//   }, undefined, "10000000000000000000000000");
//   return invested;
// }
export async function collect_funds_for_all_phases(data) {
  const response = await window.contract.collect_funds_for_all_phases({accountId: data.accountId, idea_id: data.ideaId});
  return response;
}



export async function add_like_to_idea(data){
  console.log('LIKE DATA: ', data);
  const likeIdea = await window.contract.add_like_to_idea({account: data.accountId, idea_id: data.ideaId});
  return likeIdea;
}

export async function get_all_ideas_homepage_by_owner_id(accountId){
  const invested = await window.contract.get_all_ideas_homepage_by_owner_id({owner_id: accountId, from_index: 0, limit: 10});
  return invested;
}

export async function get_sum_of_amount_for_owner(accountId){
  const amount = await window.contract.get_sum_of_amount_for_owner({owner_id: accountId});
  return amount;
}

export async function get_investor_count_for_owner(accountId){
  const investors = await window.contract.get_investor_count_for_owner({owner_id: accountId});
  return investors;
}

export async function count_phases_and_ideas_by_owner_id(accountId){
  const phases = await window.contract.count_phases_and_ideas_by_owner_id({owner_id: accountId});
  return phases;
}

export async function count_ideas_by_owner_id(accountId){
  const phases = await window.contract.count_ideas_by_owner_id({owner_id: accountId});
  return phases;
}

export async function get_all_ideas_homepage_by_investor_id2(accountId, index, limit){
  const invested = await window.contract.get_all_ideas_homepage_by_investor_id2({investor_id: accountId, from_index: 0, limit: 10});
  return invested;
}
export async function get_all_active_ideas_homepage_by_owner_id(accountId, index, limit){
  const invested = await window.contract.get_all_active_ideas_homepage_by_owner_id({owner_id: accountId, from_index: index, limit: limit});
  return invested;
}

export async function get_all_inactive_ideas_homepage_by_owner_id(accountId, index, limit){
  const invested = await window.contract.get_all_inactive_ideas_homepage_by_owner_id({owner_id: accountId, from_index: index, limit: limit});
  return invested;
}

export async function get_sum_of_amount_for_investor(accountId){
  const sumAmount = await window.contract.get_sum_of_amount_for_investor({investor_id: accountId});
  return sumAmount;
}

export async function get_invested_ideas_count(accountId){
  const ideasCount = await window.contract.get_invested_ideas_count({investor_id: accountId});
  return ideasCount;
}

export async function count_phases_and_ideas_by_investor_id(accountId){
  const countPhases = await window.contract.count_phases_and_ideas_by_investor_id({investor_id: accountId});
  return countPhases;
}

export async function ideasCount(){
  const count = await window.contract.count_all_ideas();
  return count;
}
