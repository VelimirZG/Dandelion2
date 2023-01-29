import React from "react";
import {withRouter} from "react-router-dom";
import { useState } from "react";
import { useEffect } from "react";
import Button from 'react-bootstrap/Button';
import { HeartFill } from 'react-bootstrap-icons';
import Navbar from "../components/navbar";
import Footer from "../components/footer";
import { get_all_ideas_homepage_by_investor_id2, get_invested_ideas_count, count_phases_and_ideas_by_investor_id, get_sum_of_amount_for_investor, get_all_ideas_homepage_by_owner_id, add_like_to_idea, count_phases_and_ideas_by_owner_id, get_investor_count_for_owner, get_sum_of_amount_for_owner } from "../assets/near/utils";

import '../stylesheets/profile.scss';
import IdeaForm from "../components/ideaForm";
import IdeaCard from "../components/ideaCard";

import Tab from 'react-bootstrap/Tab';
import Tabs from 'react-bootstrap/Tabs';

const Profile = (props) => {

  const [ideas, setIdeas] = useState(null);
  const accountId = window.accountId;
  const [openIdeaForm, setOpenIdeaForm] = useState(false);
  const [ideaId, setIdeaId] = useState(false);
  const [phases, setPhases] = useState(0);
  const [investors, setInvestors] = useState(0);
  const [amount, setAmount] = useState(0);

  const [investment, setInvestment] = useState(0);
  const [ideaNumber, setIdeaNumber] = useState(0);
  const [invPhases, setInvPhases] = useState(0);
  const [invIdeas, setInvIdeas] = useState(null);

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
      console.log('idea profile: ', res);
      if(nextPage) {
        setInvIdeas([...ideas, ...res])
      }else {
        setInvIdeas(res);
      }
    });
  }

  

  function listIdeas(nextPage = false) {
    get_all_ideas_homepage_by_owner_id(accountId).then( res => {
      console.log('idea profile: ', res);
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

  function collectFunds() {

  }
  

  return (
  <React.Fragment>
    <div className="container-fluid first-section profile">
      <Navbar />
      <div className="container main-wrap">
        <section className="row">
          <div className="col-12 col-lg-5 title-wrap">
            <h1>Welcome, <br/> <b>John Doe Johnson</b></h1>
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
                  <IdeaCard ideas={ideas} loadMoreIdeas={loadMoreIdeas} onProfile={true} collectFunds={collectFunds} editIdea={editIdea} />
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
                      <p className="card-subtitle">TOTAL SUPPORTED</p>
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
                  <IdeaCard ideas={invIdeas} loadMoreIdeas={loadMoreIdeas} onProfile={true} collectFunds={collectFunds} editIdea={editIdea} />
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