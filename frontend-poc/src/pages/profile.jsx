import React from "react";
import {withRouter} from "react-router-dom";
import { useState } from "react";
import { useEffect } from "react";
import Button from 'react-bootstrap/Button';
import { HeartFill } from 'react-bootstrap-icons';
import Navbar from "../components/navbar";
import Footer from "../components/footer";
import { get_all_inactive_ideas_homepage_by_owner_id, get_all_active_ideas_homepage_by_owner_id, count_ideas_by_owner_id, get_all_ideas_homepage_by_investor_id2, get_invested_ideas_count, count_phases_and_ideas_by_investor_id, get_sum_of_amount_for_investor, get_all_ideas_homepage_by_owner_id, count_phases_and_ideas_by_owner_id, get_investor_count_for_owner, get_sum_of_amount_for_owner, collect_funds_for_all_phases } from "../assets/near/utils";

import '../stylesheets/profile.scss';
import IdeaForm from "../components/ideaForm";
import IdeaCard from "../components/ideaCard";

import Tab from 'react-bootstrap/Tab';
import Tabs from 'react-bootstrap/Tabs';
import Popup from "./popup";

const Profile = (props) => {

  const [ideas, setIdeas] = useState([]);
  const [activeIdeas, setActiveIdeas] = useState([]);
  const [inactiveIdeas, setInactiveIdeas] = useState([]);
  const accountId = window.accountId;
  const [openIdeaForm, setOpenIdeaForm] = useState(false);
  const [ideaId, setIdeaId] = useState(false);
  const [phases, setPhases] = useState(0);
  const [investors, setInvestors] = useState(0);
  const [amount, setAmount] = useState(0);

  const [investment, setInvestment] = useState(0);
  const [ideaNumber, setIdeaNumber] = useState(0);
  const [invPhases, setInvPhases] = useState(0);
  const [invIdeas, setInvIdeas] = useState([]);

  const [popupInfo, setPopupInfo] = useState({open: false, msg: ''});

  const [activeIdeasIndex, setActiveIdeasIndex] = useState(0);
  const [inactiveIdeasIndex, setInactiveIdeasIndex] = useState(0);
  const [activeIdeasCount, setActiveIdeasCount] = useState(0);
  const [inactiveIdeasCount, setInactiveIdeasCount] = useState(0);

  const [index, setIndex] = useState(4);
  const limit = 4;

  
  useEffect(() => {

    count_ideas_by_owner_id(accountId).then((result) => {
      setActiveIdeasCount(result[0]);
      setInactiveIdeasCount(result[1]);
    });
   
    count_phases_and_ideas_by_owner_id(accountId).then((phases) => {
      setPhases(phases.join('/'))
    });
    get_investor_count_for_owner(accountId).then((investors) => {
      setInvestors(investors);
    })
    get_sum_of_amount_for_owner(accountId).then((amount) => {
      setAmount(amount);
    })
    listActiveIdeas();
    listInactiveIdeas();
    listIdeas();

    get_sum_of_amount_for_investor(accountId).then((inv) => {
      setInvestment(inv);
    })

    get_invested_ideas_count(accountId).then((ideasNum) => {
      setIdeaNumber(ideasNum)
    })

    count_phases_and_ideas_by_investor_id(accountId).then((phases) => {
      setInvPhases(phases.join('/'));
    })
    investorIdeas();
  }, [] )

  function investorIdeas(nextPage = false) {
    get_all_ideas_homepage_by_investor_id2(accountId, index, limit).then( res => {
      console.log('idea profile inv: ', res);
      if(nextPage) {
        setInvIdeas([...ideas, ...res])
      }else {
        setInvIdeas(res);
      }
    });
  }

  function listActiveIdeas(nextPage = false) {
    let currentIndex = activeIdeasIndex;
    if(!nextPage) {
      currentIndex = 0;
    }
    get_all_active_ideas_homepage_by_owner_id(accountId, currentIndex, limit).then( res => {
      console.log('active ideas: ', res);
      setActiveIdeas(res);
      setActiveIdeasIndex(res.length);
      if(nextPage) {
        setActiveIdeas([...activeIdeas, ...res])
      }else {
        setActiveIdeas(res);
      }
    });
  }

  function listInactiveIdeas(nextPage = false) {
    let currentIndex = inactiveIdeasIndex;
    if(!nextPage) {
      currentIndex = 0;
    }
    get_all_inactive_ideas_homepage_by_owner_id(accountId, currentIndex, limit).then( res => {
      console.log('inactive ideas: ', res);
      setInactiveIdeas(res);
      if(nextPage) {
        setInactiveIdeas([...inactiveIdeas, ...res])
      }else {
        setInactiveIdeas(res);
      }
    });
  }

  function listIdeas(nextPage = false) {
    get_all_ideas_homepage_by_owner_id(accountId).then( res => {
      if(nextPage) {
        setIdeas([...ideas, ...res])
      }else {
        setIdeas(res);
      }
    });
  }

  function loadMoreActiveIdeas() {
    setActiveIdeasIndex((activeIdeasIndex) => activeIdeasIndex + limit);
    listActiveIdeas(true);
  }

  function loadMoreInactiveIdeas() {
    setInactiveIdeasIndex(inactiveIdeasIndex + limit);
    listInactiveIdeas(true);
  }

  function loadMoreIdeas() {
    setIndex(index + limit);
    investorIdeas(true);
  }

  function editIdea(event) {
    const ideaId = event.target.getAttribute('data-idea');
    setIdeaId(ideaId);
    setOpenIdeaForm(true);
  }

  function collectFunds(event) {
    const ideaId = event.currentTarget.getAttribute('data-idea');
    collect_funds_for_all_phases({accountId: accountId, ideaId: ideaId})
  }
  

  return (
  <React.Fragment>
    <div className="container-fluid first-section profile">
      <Navbar />
      <div className="container main-wrap">
        <section className="row">
          <div className="col-12 col-lg-6 title-wrap">
            <h1>Welcome to the <br/> <b> web3 revolution!</b></h1>
          </div>
        </section>
        <div className="tab-wrap">
          <Tabs defaultActiveKey="profile" id="uncontrolled-tab-example" >
            <Tab eventKey="profile" title="My Ideas">
              <section className="row cards">
                <div className="col-12 col-lg-4">
                  <div className="card">
                    <div className="card-content">
                      <h3 className="card-title near-collected">{amount}</h3>
                      <p  className="card-subtitle">NEAR COLLECTED</p>
                    </div>
                  </div>
                </div>
                <div className="col-12 col-lg-4">
                  <div className="card">
                    <div className="card-content">
                      <h3 className="card-title supporters">{investors}</h3>
                      <p className="card-subtitle">TOTAL SUPPORTERS</p>
                    </div>
                  </div>
                </div>
                <div className="col-12 col-lg-4">
                  <div className="card">
                    <div className="card-content">
                      <h3 className="card-title phases">{phases}</h3>
                      <p className="card-subtitle">TOTAL SUCCESSFULL PHASES/PROJECTS</p>
                    </div>
                  </div>
                </div>
              </section>
              <section className="container-lg projects-wrap pt-5" >
                <div className="mt-5 d-flex flex-column">
                  {
                    ideas.length > 0 ?
                      <React.Fragment>
                        <h5 className="projects-headline">Active Ideas</h5>
                        <h6 className="projects-description">Projects that are currently open for funding and actively accepting contributions from supporters.</h6>
                      { activeIdeas.length > 0 && 
                        <React.Fragment>
                          <IdeaCard ideas={activeIdeas} loadMoreIdeas={loadMoreActiveIdeas} onProfile={true} collectFunds={collectFunds} editIdea={editIdea} ideaIndex={activeIdeasIndex} ideasCount={activeIdeasCount} />
                        </React.Fragment>
                      }
                      <h5 className="projects-headline">Inactive Ideas</h5>
                      <h6 className="projects-description">Projects that have completed their current funding phase but are still ongoing and may have future funding phases. You can re-activate your project by clicking "Edit" and updating the information with a new goal. </h6>
                      { inactiveIdeas.length > 0 && 
                        <React.Fragment>
                          <IdeaCard ideas={inactiveIdeas} loadMoreIdeas={loadMoreInactiveIdeas} onProfile={true} collectFunds={collectFunds} editIdea={editIdea} ideaIndex={inactiveIdeasCount} ideasCount={inactiveIdeasCount} />
                        </React.Fragment>
                      }
                      
                      </React.Fragment>
                    :
                    <React.Fragment>
                      <h4>
                      <p>We are thrilled that you are considering sharing your breakthrough blockchain-based idea or ideas within our vibrant and engaged community.</p>
                      <p>Dandelion will allow your idea to get visibility, feedback and validation, requests for collaboration and most importantly funding.</p>
                      <p>So, bring your ideas to life - submit today!</p><br></br>
                      <p>Here are some tips to help you get started:</p>
                      <p>You can submit one IDEA at a time and the process is very simple - just click &ldquo;Create Idea&rdquo; and fill in the blanks!&nbsp;</p>
                      <p>If you have completed your submission successfully it will be visible immediately</p></h4>
                    </React.Fragment>
                  }
                </div>
              </section>
            </Tab>
            <Tab eventKey="investments" title="My investments" className="second-tab">
              <section className="row cards">
                <div className="col-12 col-lg-4">
                  <div className="card">
                    <div className="card-content">
                      <h3 className="card-title near-collected">{investment}</h3>
                      <p  className="card-subtitle">NEAR INVESTED</p>
                    </div>
                  </div>
                </div>
                <div className="col-12 col-lg-4">
                  <div className="card">
                    <div className="card-content">
                      <h3 className="card-title supporters">{ideaNumber}</h3>
                      <p className="card-subtitle">NUMBER OF INVESTMENTS</p>
                    </div>
                  </div>
                </div>
                <div className="col-12 col-lg-4">
                  <div className="card">
                    <div className="card-content">
                      <h3 className="card-title phases">{invPhases}</h3>
                      <p className="card-subtitle">TOTAL SUCCESSFULL PHASES/PROJECTS</p>
                    </div>
                  </div>
                </div>
              </section>
              <section className="container-lg projects-wrap pt-5" >
              <div className="mt-5 d-flex flex-column">
                  {invIdeas.length > 0 ?
                    <IdeaCard isInvestment={true} ideas={invIdeas} loadMoreIdeas={loadMoreIdeas} onProfile={true} collectFunds={collectFunds} editIdea={editIdea} />
                    : 
                    <React.Fragment>
                      <h4>
                      <p>We are thrilled that you are considering investing in breakthrough blockchain-based ideas within our community.</p>
                      <p>The potential to change the world and WIN BIG here is huge and all with a very small investment - can there be anything more exciting&hellip; we think not!</p><br></br>
                      <p>So get in early, as an idea investor of web3 projects before they blast off!</p></h4>
                    </React.Fragment>
                  }
                </div>
              </section>
            </Tab>
          </Tabs>
        </div>
      </div>
    </div>
    <Footer />
    { openIdeaForm && <IdeaForm setOpenIdeaForm={setOpenIdeaForm} ideaId={ideaId} /> }
    {
      popupInfo.open &&
        <Popup msg={popupInfo.msg} setPopupInfo={setPopupInfo} />
    }
  </React.Fragment>

    
  );
};

export default withRouter(Profile);