import React from "react";
import {withRouter} from "react-router-dom";
import Badge from 'react-bootstrap/Badge';
import Tab from 'react-bootstrap/Tab';
import Tabs from 'react-bootstrap/Tabs';
import { getIdea, get_investment_for_idea, get_investment_goal, login, invest } from "../assets/near/utils";
import { useState } from "react";
import { useEffect } from "react";
import Button from 'react-bootstrap/Button';
import Navbar from "../components/navbar";
import Popup from '../pages/popup';

import '../stylesheets/single.scss';
import Footer from "../components/footer";

const Single = (props) => {

  const [idea, setIdea] = useState(false);
  const accountId = window.accountId;
  console.log(accountId)
  const [currentInvValue, setCurrentInvValue] = useState(0);
  const ideaId = props.match.params.ideaId;
  const [popupInfo, setPopupInfo] = useState({open: false, msg: ''});

  const ONE_NEAR= 1000000000000000000000000;

  useEffect(() => { 
    getIdea(ideaId).then( idea => {
      console.log(idea);
      calculateIdeaStatus(idea);
      
    });
  }, [] )
  
  function calculateIdeaStatus(idea) {
    let sum =0, goal =0;
    idea.investments.forEach( inv => {
      goal += inv.goal;
      sum += inv.sum;
    });
    idea.sum=sum;
    idea.goal=goal;
    setIdea(idea);
    console.log('idea: ', idea);
  }

  function investInIdea() {
    if(currentInvValue > (idea.goal - idea.sum)) {
      setPopupInfo({open: true, msg: 'You cannot invest more than needed for current phase'});
    }else {
      invest({value: (currentInvValue * ONE_NEAR), acc: accountId, ideaId: ideaId});  
    }
  }

  
  
  if(idea) {
    return (
    <React.Fragment>
      <div className="container-fluid first-section single">
        <Navbar />
        <div className="container main-wrap">
          <div className="row mt-5 first-part-wrap">
            {/* <div className="col-12 col-lg-7 d-flex flex-column "> */}
              <div className="row title-wrap">
                <div className="col-12 col-lg-7 d-flex flex-column">
                  <div className="row text-wrap w-100 m-0 mobile-padding">
                    <div className="col-12 col-lg-8">
                      <h1 className="idea-title">{idea.title}</h1>
                    </div>
                  </div>
                  <div className="row d-flex justify-content-center align-items-center w-100 m-0">
                    <div className="col-12 col-lg-9 tag-wrap">
                      <div class="tags">
                        {
                          idea.tags.map((element, i) => {
                            return (<Button pill key={i} className="tag-btn me-2" >{element}</Button>);
                          })
                        }
                      </div>
                    </div>
                    <div className="col-12 col-lg-3 link-wrap">
                      {
                        idea.website && <a href={idea.website}>Go to <b>Website</b> <img src={`${process.env.PUBLIC_URL}/pointer.png`} className="link-img"/></a>
                      }
                    </div>
                  </div>
              
                  <img className="w-100 idea-img" src={idea.picture_url} alt="Card image cap" />
                </div>
                <div className="col-12 col-lg-5 d-flex flex-column current-status-wrap">
                  <div className="card">
                    <div className="card-body">
                      <h6 className="card-subtitle">TOTAL DEPOSITED</h6>
                      <h3 className="current-status">{idea.sum} <img src={`${process.env.PUBLIC_URL}/near-logo-single.png`} /></h3>
                      {/* <p className="cash-amount">~ $842,996.247 USD</p> */}
                      <div className="row info-wrap">
                        <div className="col">
                          <h4 className="status-title">SUPPORTERS</h4>
                          <h5 className="status-text mobile-align-left"><b>{idea.investors_count}</b></h5>
                        </div>
                        {/* <div className="col">
                          <p className="text-muted">SUPPORTERS</p>
                          <h4 className="fw-bold">86</h4>
                        </div> */}
                        <div className="col">
                          <h4 className="status-title mobile-align-right">STATUS</h4>
                          <h5 className="status-text light-text">
                            {
                              Math.round((100 * idea.sum) / idea.goal ) >= 100 ? 'FINISHED' : 'IN PROGRESS'
                            }
                          </h5>
                        </div>
                      </div>
                    </div>
                    <hr className="solid" />
                    <h4 className="goals">GOALS</h4>
                      <ul className="progress-list">
                      {
                          idea.investments.map((investment, i) => {
                            let percentage = investment.goal == 0 ? 0 : (Math.round((100 * investment.sum) / investment.goal ));
                            let title;
                            let color;
                            let iconPath;

                            if (i==0){
                              title="Idea";
                              color="#F9ED32";
                              iconPath = process.env.PUBLIC_URL + '/phase_1.png';
                            } else if (i==1){
                              title="Prototype";
                              color="#EEA91E";
                              iconPath = process.env.PUBLIC_URL + '/phase_2.png';
                            } else if (i==2){
                              title="MVP";
                              color="#693E98";
                              iconPath = process.env.PUBLIC_URL + '/phase_3.png';
                            }else{
                              title="Presale";
                              color="#CE225B";
                              iconPath = process.env.PUBLIC_URL + '/phase_4.png';
                            }
                            return (<li class="progress-list-item-wrap">
                            <div className="row progress-list-item w-100 m-0">
                              <div className="idea-img-wrap">
                                <img className="w-100 idea-img" src={iconPath} alt="Card image cap" />
                              </div>
                              <div className="p-0 info-wrap">
                                <div className="row m-0 w-100 status-first-part">
                                  <div className="col-4 col-lg-3 p-0">
                                    <p className="goal-title">{title}</p>
                                    <div className="mobile-goal-wrap">
                                      <p className="goal-goal">{investment.goal} st</p>
                                      <img className="w-100 idea-img" src={`${process.env.PUBLIC_URL}/near-logo-small.png`} alt="" />
                                    </div>
                                  </div>
                                  <div className="col-8 col-lg-9 list-item-wrap p-0">
                                    <p className="goal-goal">{investment.goal} st</p>
                                    <img className="w-100 idea-img" src={`${process.env.PUBLIC_URL}/near-logo-small.png`} alt="" />
                                    {
                                       investment.sum == investment.goal ? <Button pill className="status-btn me-2" >COMPLETED</Button> : <Button pill className="status-btn me-2" >IN PROGRESS</Button>
                                    }
                                    <p className="percentage">{percentage} %</p>
                                  </div>
                                </div>
                                <div className="row  m-0">
                                  <div className="col p-0">
                                    <div className="progress" style={{backgroundColor: '#262626'}}>
                                      <div className="progress-bar" style={{ width: percentage + '%', backgroundColor: color }} role="progressbar" aria-valuenow="50" aria-valuemin="0" aria-valuemax="100"></div>
                                    </div>
                                  </div>
                                </div>
                              </div>
                            </div>
                          </li>);
                          })
                        }
                      </ul>
                    {
                      !accountId ?
                      (<button type="button" className="connect-btn mt-3" onClick={() => login()  }>Connect wallet to fund</button>)
                      :
                      <div className="invest-wrap d-flex mt-3  justify-content-start align-items-center">
                        <input type="number" className="investInput" onChange={(event) => setCurrentInvValue(event.target.value)} />
                        <img src="/near-logo-single.png" className="ms-2" style={{height: '30px', width: 'auto'}}/>
                        <Button variant="outline-primary ms-auto" className="btn btn-outline-primary ms-auto tag-btn" onClick={(e) => investInIdea(e)}>INVEST</Button>
                      </div>
                    }
                  </div>
                </div>
              </div>
              <div className="row description-wrap">
                <div className="col-12 col-md-12 col-lg-7">
                  <div className="tab-wrap">
                    <Tabs defaultActiveKey="description" id="uncontrolled-tab-example" >
                      <Tab eventKey="description" title="Description">
                        <p dangerouslySetInnerHTML={{__html: idea.description.replace(/\n/g, "<br />")}} />
                      </Tab>
                      <Tab eventKey="competitors" title="Competitors">
                        <p>{idea.competitors}</p>
                      </Tab>
                      <Tab eventKey="valueProposition" title="Value proposition">
                        <p dangerouslySetInnerHTML={{__html: idea.value_proposition.replace(/\n/g, "<br />")}} />
                      </Tab>
                      <Tab eventKey="team" title="Team">
                        {/* <p>{idea.team}</p> */}
                        <p>dummy tekst</p>
                      </Tab>
                    </Tabs>
                  </div>
                    
                  <div className="comments-wrap">
                    <Tabs defaultActiveKey="comments" id="uncontrolled-tab-example" >
                      <Tab eventKey="comments" title="Comments">
                        <div className="row comment-wrap">
                          <div className="col-2 image-wrap">
                            <img src={`${process.env.PUBLIC_URL}/profile.png`}/>
                          </div>
                          <div className="col-10 content-wrap">
                            <div className="author-wrap">
                              <p className="author">John Doe Johnson</p>
                              <p className="date">3w ago</p>
                            </div>
                            <p className="content">Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen.</p>
                          </div>
                        </div>
                        <div className="row">
                          <div className="col-2 image-wrap">
                            <img src={`${process.env.PUBLIC_URL}/profile.png`}/>
                          </div>
                          <div className="col-10 content-wrap">
                            <div className="author-wrap">
                              <p className="author">John Doe Johnson</p>
                              <p className="date">3w ago</p>
                            </div>
                            <p className="content">Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen.</p>
                          </div>
                        </div>
                      </Tab>
                    </Tabs>
                  </div>
                </div>
              </div>
            {/* </div> */}
          </div>
        </div>  
      </div>
      {
      popupInfo.open &&
        <Popup msg={popupInfo.msg} setPopupInfo={setPopupInfo} />
      }
      <Footer />
    </React.Fragment>
    );
  }else {
    return;
  }
  

};

export default withRouter(Single);