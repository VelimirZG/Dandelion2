import React, { useEffect } from "react";
import { BrowserRouter, Route } from "react-router-dom";
import Single from "./pages/single";
import './index.css';
import Homepage from "./pages/homepage";
import Profile from "./pages/profile";
import Page from "./pages/page";
import About from "./pages/about";
import How from "./pages/how_it_works";
import Faq from "./pages/faq";
import Terms from "./pages/terms";
import Privacy from "./pages/privacy";


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
      <Route exact path="/page"  render={() => <Page/> } /> 
      <Route exact path="/about"  render={() => <About/> } /> 
      <Route exact path="/how"  render={() => <How/> } />
      <Route exact path="/faq"  render={() => <Faq/> } />
      <Route exact path="/terms"  render={() => <Terms/> } />
      <Route exact path="/privacy"  render={() => <Privacy/> } />
      {/* copy the line and change the page */}
    </BrowserRouter>
  );

};

export default Main;
