import React from "react";
import {withRouter} from "react-router-dom";
import { useState } from "react";
import { useEffect } from "react";
import Button from 'react-bootstrap/Button';
import { HeartFill } from 'react-bootstrap-icons';
import Navbar from "../components/navbar";
import Footer from "../components/footer";
import { get_all_ideas_homepage_by_investor_id2, get_invested_ideas_count, count_phases_and_ideas_by_investor_id, get_sum_of_amount_for_investor, get_all_ideas_homepage_by_owner_id, add_like_to_idea, count_phases_and_ideas_by_owner_id, get_investor_count_for_owner, get_sum_of_amount_for_owner, collect_funds_for_all_phases } from "../assets/near/utils";

import '../stylesheets/profile.scss';
import IdeaForm from "../components/ideaForm";
import IdeaCard from "../components/ideaCard";

import Tab from 'react-bootstrap/Tab';
import Tabs from 'react-bootstrap/Tabs';

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

  const [index, setIndex] = useState(0);
  const limit = 20;

  
  useEffect(() => { 
   
    count_phases_and_ideas_by_owner_id(accountId).then((phases) => {
      setPhases(phases.join('/'))
    });
    get_investor_count_for_owner(accountId).then((investors) => {
      setInvestors(investors);
    })
    get_sum_of_amount_for_owner(accountId).then((amount) => {
      setAmount(amount);
    })
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
    get_all_ideas_homepage_by_investor_id2(accountId).then( res => {
      console.log('idea profile inv: ', res);
      if(nextPage) {
        setInvIdeas([...ideas, ...res])
      }else {
        setInvIdeas(res);
      }
    });
  }

  

  function listIdeas(nextPage = false) {
    get_all_ideas_homepage_by_owner_id(accountId).then( res => {
      console.log('idea profile owner: ', res);
      res.map(idea => {
        if(idea.active) {
          setActiveIdeas([...activeIdeas, idea]);
        }else {
          setInactiveIdeas([...inactiveIdeas, idea]);
        }
      })
      if(nextPage) {
        setIdeas([...ideas, ...res])
      }else {
        setIdeas(res);
      }
    });
  }

  function loadMoreIdeas() {
    setIndex(index + limit);
    listIdeas(true);
  }

  function editIdea(event) {
    const ideaId = event.target.getAttribute('data-idea');
    setIdeaId(ideaId);
    setOpenIdeaForm(true);
  }


  async function likeIdea(event) {
    console.log(event.target);
    if(accountId) {
      const ideaId = event.currentTarget.getAttribute('data-idea');
      const likedIdea = await add_like_to_idea({ideaId: ideaId, accountId: accountId});
      console.log('LIKED IDEA: ', likedIdea);
    }
  }

  function collectFunds(ideaId) {
    collect_funds_for_all_phases({accountId: accountId, ideaId: ideaId})
  }
  

  return (
  <React.Fragment>
    <div className="container-fluid first-section profile">
      <Navbar />
      <div className="container main-wrap">
        <section className="row">
          <div className="col-12 col-lg-6 title-wrap">
            <h1>Welcome to the <br/> <b>the web3 revolution!</b></h1>
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
                        <h6 className="projects-description">Ovo je neki tekst za aktivne ideje</h6>
                      { activeIdeas.length > 0 && 
                        <React.Fragment>
                          <IdeaCard ideas={activeIdeas} loadMoreIdeas={loadMoreIdeas} onProfile={true} collectFunds={collectFunds} editIdea={editIdea} />
                        </React.Fragment>
                      }
                      <h5 className="projects-headline">Inactive Ideas</h5>
                      <h6 className="projects-description">Ovo je neki tekst za neaktivne ideje</h6>
                      { inactiveIdeas.length > 0 && 
                        <React.Fragment>
                        
                          <IdeaCard ideas={inactiveIdeas} loadMoreIdeas={loadMoreIdeas} onProfile={true} collectFunds={collectFunds} editIdea={editIdea} />
                        </React.Fragment>
                      }
                      
                      </React.Fragment>
                    :
                    <React.Fragment>
                      <p>We are thrilled that you are considering sharing your breakthrough blockchain-based idea or ideas within our vibrant and engaged community.</p>
                      <p>Dandelion will allow your idea to get visibility, feedback and validation, requests for collaboration and most importantly funding.</p>
                      <p>So, bring your ideas to life - submit today!</p>
                      <p>You can submit one IDEA at a time and the process is very simple - just click &ldquo;Create Idea&rdquo; and fill in the blanks!&nbsp;</p>
                      <p>If you have completed your submission successfully it will be visible immediately</p>
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
                  <IdeaCard ideas={invIdeas} isInvestment={true} loadMoreIdeas={loadMoreIdeas} onProfile={true} collectFunds={collectFunds} editIdea={editIdea} />
                </div>
              </section>
            </Tab>
          </Tabs>
        </div>
      </div>
    </div>
    <Footer />
    { openIdeaForm && <IdeaForm setOpenIdeaForm={setOpenIdeaForm} ideaId={ideaId} /> }
  </React.Fragment>

    
  );
};

export default withRouter(Profile);