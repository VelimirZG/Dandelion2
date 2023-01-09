import React, { useEffect } from "react";
import { BrowserRouter, Route } from "react-router-dom";
import Single from "./pages/single";
import './index.css';
import Homepage from "./pages/homepage";
import Profile from "./pages/profile";

const Main = () => {


  useEffect(() => {
    const url = new URL(window.location.href);
    const searchParams = url.searchParams;
    const walletId = searchParams.get('account_id');
    const allKeys = searchParams.get('all_keys');
    if(walletId) {
      console.log('NEW WALLET: ', walletId)
      localStorage.setItem('account_id', walletId);
      localStorage.setItem('all_keys', allKeys);
    }
  }, []);


  // async function setUserWallet(walletId) {
  //   console.log(window.nearConnection);
  //   const account = await window.nearConnection.account(walletId);

  //   window.nearCurrentAccountId = walletId;
  //   console.log(account);
  // }

  return (
    <BrowserRouter >
      <Route exact path="/" render={() => <Homepage/> } />
      <Route exact path="/:ideaId"  render={() => <Single/> } />
      <Route exact path="/profile"  render={() => <Profile/> } />
    </BrowserRouter>
  );

};

export default Main;
