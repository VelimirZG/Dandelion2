import React from "react";
import {withRouter} from "react-router-dom";
import { useState } from "react";
import { useEffect } from "react";
import Button from 'react-bootstrap/Button';
import { HeartFill } from 'react-bootstrap-icons';
import Navbar from "../components/navbar";
import Footer from "../components/footer";
import { get_all_ideas, get_all_ideas_homepage_by_owner_id, add_like_to_idea, count_phases_and_ideas_by_owner_id, get_investor_count_for_owner, get_sum_of_amount_for_owner } from "../assets/near/utils";

import '../stylesheets/profile.scss';
import IdeaForm from "../components/ideaForm";
import IdeaCard from "../components/ideaCard";

const Profile = (props) => {

  const [ideas, setIdeas] = useState(null);
  const accountId = window.accountId;
  const [openIdeaForm, setOpenIdeaForm] = useState(false);
  const [ideaId, setIdeaId] = useState(false);
  const [phases, setPhases] = useState(0);
  const [investors, setInvestors] = useState(0);
  const [amount, setAmount] = useState(0);

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
  }, [] )

  function listIdeas(nextPage = false) {
    get_all_ideas_homepage_by_owner_id(accountId).then( res => {
      console.log('idea profile: ', res);
      if(nextPage) {
        setIdeas([...ideas, ...res])
      }else {
        setIdeas(res);
      }
    });
    // get_all_ideas(index, limit).then( res => {
    //   console.log('ideas from all ideas: ', res);
    //   if(nextPage) {
    //     setIdeas([...ideas, ...res])
    //   }else {
    //     setIdeas(res);
    //   }
    //   console.log(ideas);
    // });
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
            {/* {
            ideas && ideas.map((item, id) => {
              return (<div className="row project-card-wrap" key={id}>
                <div className="col-12 card-wrap">
                  <div className="card">
                      <div className="d-flex mt-auto flex-sm-column flex-lg-row row">
                        <div className="col-xs-12 col-sm-12 col-lg-4 d-flex justify-content-center align-items-center p-0 img-container" style={{backgroundImage: 'url(' + item.picture_url + ')'}}>
                        </div>
                        <div className="card-content d-flex mb-lg-auto flex-column justify-content-center  col-xs-12 col-sm-12 col-md-12 col-lg-5">
                          <h4 className="card-title text-center text-md-start text-lg-start" style={{cursor: 'pointer'}} onClick={() => { window.location.href= process.env.PUBLIC_URL + '/' + item.idea_id}}>{item.title}</h4>
                          <p className="card-text">
                            {item.excerpt}
                          </p>
                          <p className="card-tags d-flex justify-content-start align-items-start flex-wrap">
                            {
                              item.tags.map((element, i) => {
                                if(i === 0) {
                                  return (<Button key={i} className="tag-btn me-2">{element}</Button>);
                                }else {
                                  return (<Button key={i} className="tag-btn me-2">{element}</Button>);
                                } 
                              })
                            }
                          </p>
                        </div>
                        <div className="card-info col-xs-12 col-sm-12 col-lg-3 mt-4 mt-md-0 mt-lg-0">
                          <button className="ms-auto favorite-icon" style={{height: '35px'}} data-idea={item.idea_id} onClick={(e) => likeIdea(e)} >
                            <HeartFill color="#C6C6C6" size='30px'/>
                          </button>
                          <div className="project-info-wrap">
                            <div className="raised-wrap progress-wrap-card">
                              <div className="info-raised-text">
                                <p className="">Raised</p>
                                <div className="stat-wrap">
                                  <span className="out-of ms-1">{item.sum} /</span> <span className="out-of-total">{item.amount}</span>
                                </div>
                              </div>
                              <div className="progress" style={{backgroundColor: "#313131"}}>
                                <div className="progress-bar" style={{ width: ((100 * item.amount) / item.sum ) + '%', backgroundColor: "#EEA91E" }} role="progressbar" aria-valuenow="50" aria-valuemin="0" aria-valuemax="100"></div>
                              </div>
                            </div>
                            <div className="invest-wrap d-flex justify-content-start align-items-center">
                              <div className="support-wrap">
                                <p className="supporters"><b>22</b> supportes</p>
                              </div>
                              {
                                item.goal_reached && 
                                <Button variant="outline-primary ms-auto tag-btn edit-btn" data-idea={item.idea_id} onClick={(e) => collectFunds(e)}>
                                  COLLECT <img src={`${process.env.PUBLIC_URL}/pen.png`}/>
                                </Button>
                              }
                              
                              <Button variant="outline-primary ms-auto tag-btn edit-btn" data-idea={item.idea_id} onClick={(e) => editIdea(e)}>
                                EDIT <img src={`${process.env.PUBLIC_URL}/pen.png`}/>
                              </Button>
                            </div>
                          </div>
                        </div>
                      </div>
                  </div>
                </div>
              </div>)
              })
            } */}
            </div>
          </section>
      </div>
    </div>
    <Footer />
    { openIdeaForm && <IdeaForm setOpenIdeaForm={setOpenIdeaForm} ideaId={ideaId} /> }
  </React.Fragment>

    
  );
};

export default withRouter(Profile);