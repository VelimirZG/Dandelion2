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
    viewMethods: ['ideas_for_owner', 'get_idea_for_single', 'get_investments', 'total_investments', 'get_all_ideas_homepage', 'get_investment_goal', 'get_investment_for_idea'],
    // Change methods can modify the state. But you don't receive the returned value when called.
    changeMethods: ['create_idea', 'invest', 'add_like_to_idea'],
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
export async function get_all_ideas(){
  const all_ideas = await window.contract.get_all_ideas_homepage({"from_index":0,"limit":20})
  return all_ideas;
}

export async function get_investment_goal(ideaId){
  const inv_goal = await window.contract.get_investment_goal({idea_id: ideaId})
  return inv_goal;
}

export async function invest(data){
  console.log('DATA: ', data);
  let gas = 300000000000000;
  const invested = await window.contract.invest({account_id: data.acc, idea_id: data.ideaId}, gas.toLocaleString('fullwide', {useGrouping:false}) , data.value.toLocaleString('fullwide', {useGrouping:false}));
  return invested;
}

export async function add_like_to_idea(data){
  console.log('LIKE DATA: ', data);
  const likeIdea = await window.contract.add_like_to_idea({account: data.accountId, idea_id: data.ideaId});
  return likeIdea;
}
