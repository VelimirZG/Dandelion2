import { useEffect, useState } from "react";
import React from "react";
import {withRouter, Redirect} from "react-router-dom";

import { login, logout } from "../assets/near/utils";
import IdeaForm from "./ideaForm";
import { colors } from "@material-ui/core";


import '../stylesheets/navbar.scss';
import Popup from "../pages/popup";

const Navbar = () => {
  
  const [openIdeaForm, setOpenIdeaForm] = useState(false);
  const accountId = window.accountId;
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);
  const [popupInfo, setPopupInfo] = useState({open: false, msg: ''});

  function mobileMenu() {
    console.log(document.querySelector("#menu"));
    if(!mobileMenuOpen) {
      document.querySelector("#menu").classList.add('open');
      document.querySelector("body").classList.add('remove-scroll');
    }else {
      document.querySelector("body").classList.remove('remove-scroll');
      document.querySelector("#menu").classList.remove('open');
    }
    setMobileMenuOpen(!mobileMenuOpen);
  }
  
  function walletLogout() {
    logout();
    console.log('AFTER LOGUT: ', accountId)
  }
  function checkIfLoggedIn() {
    if(accountId) {
      setOpenIdeaForm(true);
    }else {
      setPopupInfo({open: true, msg: 'Please connect wallet to create an idea'});
    }
  }
  return (
    <div className="container-fluid">
      <div id="test" className="row d-flex justify-content-center align-items-center navbar">
        <div className="col-sm-12 col-md-12 col-lg-3 wrap-header-logo">
            <a href="/" className="logo-wrap">
              <img src={`${process.env.PUBLIC_URL}/logo-header.png`} style={{width: 'auto'}}/>
            </a>
            {/* <div className="hamburger-menu">
              <input type="checkbox" className="hamburger-checkbox" />
              <span></span>
              <span></span>
              <span></span>
            </div> */}
            <div id="menuToggle">
              <input type="checkbox"  onClick={() => mobileMenu()}/>
              <span></span>
              <span></span>
              <span></span>
              <div></div>
            </div>
        </div>
        <div className="col-sm-12 col-lg-9 header-menu">
          <ul>
            <li><a href="./how">How it works</a></li>
            <li><a href="./profile">Dashboard</a></li>
            <li><a href="./about">About</a></li>
          </ul>
          <button className="btn header-button create-idea-btn" onClick={()=> checkIfLoggedIn()}>
            Create idea
          </button>
          {accountId ? 
            <button className="btn header-button connect-wallet" onClick={()=> walletLogout()}>Disconnect wallet</button>
              :
            <button className="btn header-button connect-wallet" onClick={()=>login()}>Connect wallet</button>
          }
        </div>
        {/* <ul id="mobile-menu">
          <li><a href="#">How it works</a></li>
          <li><a href="#">FAQ</a></li>
          <li><a href="#">About</a></li>
        </ul> */}
        {
        openIdeaForm &&
          <IdeaForm setOpenIdeaForm={setOpenIdeaForm} />
        }
      </div>
      <div className="container-fluid" id="menu">
        <div className="container h-100">
          <ul className="h-100">
            <div>
              <li><a href="#">How it works</a></li>
              <li><a href="#">FAQ</a></li>
              <li><a href="#">About</a></li>
            </div>
            <div className="mobile-buttons">

              <button className="btn header-button mb-3 create-idea-btn" onClick={()=> checkIfLoggedIn()}>
                Create idea
              </button>
              {
              accountId ? 
                <button className="btn header-button connect-wallet" onClick={()=> walletLogout()}>Disconnect wallet</button>
                  :
                <button className="btn header-button connect-wallet" onClick={()=>login()}>Connect wallet</button>
              }
            </div>
          </ul>
        </div>
      </div>
      {
      popupInfo.open &&
        <Popup msg={popupInfo.msg} setPopupInfo={setPopupInfo} />
      }
    </div>
    
  );

};

export default Navbar;
