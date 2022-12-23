import React from "react";
import {withRouter} from "react-router-dom";
import { useState } from "react";
import { useEffect } from "react";
import Button from 'react-bootstrap/Button';
import { HeartFill } from 'react-bootstrap-icons';
import Navbar from "../components/navbar";
import Footer from "../components/footer";
import { login, get_all_ideas, get_investment_goal, total_investments, get_investment_for_idea, invest, add_like_to_idea, logout } from "../assets/near/utils";

import '../stylesheets/profile.scss';

const Profile = (props) => {

  const [ideas, setIdeas] = useState([]);
  const [currentInvValue, setCurrentInvValue] = useState(0.2);
  const ONE_NEAR= 1000000000000000000000000;
  const accountId = window.accountId;
  const [popupInfo, setPopupInfo] = useState({open: false, msg: ''});


  useEffect(() => { 
    get_all_ideas().then( resIdeas => {
      console.log('ideas from all ideas: ', resIdeas);
      get_ideas_info(resIdeas);
    });
  }, [] )


  async function get_ideas_info(ideas) {
    for(const idea of ideas) {
      const goal = await get_investment_goal(idea.idea_id)
      idea.inv_goal = goal;

      const inv = await get_investment_for_idea(idea.idea_id)
      idea.inv_total = inv.total_amount / ONE_NEAR;

      const totalInv = await total_investments();
      idea.investors = totalInv;
    }

    setIdeas(ideas);

  }

  function investInIdea(event) {
    
    if(accountId) {
      const ideaId = event.target.getAttribute('data-idea');
      invest({value: (currentInvValue * ONE_NEAR), acc: accountId, ideaId: ideaId});
    }else {
      setPopupInfo({open: true, msg: 'Please connect wallet to invest into the idea'});
    }
  }


  async function likeIdea(event) {
    console.log(event.target);
    
    if(accountId) {
      const ideaId = event.currentTarget.getAttribute('data-idea');
      const likedIdea = await add_like_to_idea({ideaId: ideaId, accountId: accountId});
      console.log('LIKED IDEA: ', likedIdea);
    }else {
      setPopupInfo({open: true, msg: 'Please connect wallet to like the idea'});
    }
    
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
                <h3 className="card-title near-collected">187,000.66544</h3>
                <p  className="card-subtitle">NEAR COLLECTED</p>
              </div>
            </div>
          </div>
          <div className="col-12 col-lg-4">
            <div className="card">
              <div className="card-content">
                <h3 className="card-title supporters">1.963</h3>
                <p className="card-subtitle">TOTAL SUPPORTERS</p>
              </div>
            </div>
          </div>
          <div className="col-12 col-lg-4">
            <div className="card">
              <div className="card-content">
                <h3 className="card-title phases">16/3</h3>
                <p className="card-subtitle">TOTAL SUCCESSFULL PHASES/PROJECTS</p>
              </div>
            </div>
          </div>
        </section>
        <section className="container-lg projects-wrap pt-5" >
          <div className="mt-5">
            {
            ideas.map((item, id) => {
              return (<div className="row project-card-wrap" key={id}>
                <div className="col-12 card-wrap">
                  <div className="card">
                      <div className="d-flex mt-auto flex-sm-column flex-lg-row row">
                        <div className="col-xs-12 col-sm-12 col-lg-4 d-flex justify-content-center align-items-center p-0 img-container" style={{backgroundImage: 'url(' + item.metadata.picture_url + ')'}}>
                          {/* <img className="w-100 h-100" src={item.metadata.picture_url} alt="Card image cap" /> */}
                        </div>
                        <div className="card-content d-flex mb-lg-auto flex-column justify-content-center  col-xs-12 col-sm-12 col-md-12 col-lg-5">
                          <h4 className="card-title text-center text-md-start text-lg-start" style={{cursor: 'pointer'}} onClick={() => { window.location.href= process.env.PUBLIC_URL + '/' + item.idea_id}}>{item.metadata.title}</h4>
                          <p className="card-text">
                            {item.metadata.excerpt}
                          </p>
                          <p className="card-tags d-flex justify-content-start align-items-start flex-wrap">
                            {
                              item.metadata.tags.map((element, i) => {
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
                                  <span className="out-of ms-1">300 /</span> <span className="out-of-total">500</span>
                                </div>
                              </div>
                              <div className="progress" style={{backgroundColor: "#313131"}}>
                                <div className="progress-bar" style={{ width: ((100 * item.inv_total) / item.inv_goal ) + '%', backgroundColor: "#EEA91E" }} role="progressbar" aria-valuenow="50" aria-valuemin="0" aria-valuemax="100"></div>
                              </div>
                            </div>
                            <div className="invest-wrap d-flex justify-content-start align-items-center">
                              <div className="support-wrap">
                                <p className="supporters"><b>22</b> supportes</p>
                                </div>
                              <Button variant="outline-primary ms-auto tag-btn edit-btn" data-idea={item.idea_id} onClick={(e) => investInIdea(e)}>
                                INVEST <img src={`${process.env.PUBLIC_URL}/pen.png`}/>
                              </Button>
                            </div>
                          </div>
                        </div>
                      </div>
                  </div>
                </div>
              </div>)
              })
            }
            </div>
          </section>
      </div>  
    </div>
    <Footer />
  </React.Fragment>
  );
};

export default withRouter(Profile);