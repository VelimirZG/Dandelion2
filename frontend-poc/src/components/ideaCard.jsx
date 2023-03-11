import React from "react";
import { useEffect, useState } from "react";
import Button from 'react-bootstrap/Button';
import { HeartFill } from 'react-bootstrap-icons';
import Popup from '../pages/popup';
import { invest, ideasCount } from "../assets/near/utils";

import * as nearAPI from "near-api-js";

const IdeaCard = (props) => {

  const [currentInvValue, setCurrentInvValue] = useState(0.1);
  const accountId = window.accountId;
  const [popupInfo, setPopupInfo] = useState({open: false, msg: ''});
  const investOptions = [0.1,0.2,0.5,1,2,3,4,5];
  const isOnProfile = props.onProfile;
  const { utils } = nearAPI;
  const [allLikes, setAllLikes] = useState([]);


  useEffect(() => {
    getAllLikes();
  }, []);


  async function getAllLikes() {
    const rawResponse = await fetch(process.env.REACT_APP_SERVER_URL + '/likes', {
      method: 'POST',
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({walletId: accountId})
    })
    const content = await rawResponse.json();
    setAllLikes(content.likes);
  }


  function investInIdea(event) {
    
    if(accountId) {
      const ideaId = event.target.getAttribute('data-idea');
      const amountInYocto = utils.format.parseNearAmount(currentInvValue.toString());
      invest({value: amountInYocto, acc: accountId, ideaId: parseInt(ideaId)});
    }else {
      setPopupInfo({open: true, msg: 'Please connect wallet to invest into the idea'});
    }
  }


  async function likeIdea(event) {
    if(accountId) {
      const ideaId = event.currentTarget.getAttribute('data-idea');
      const rawResponse = await fetch(process.env.REACT_APP_SERVER_URL + '/like', {
        method: 'POST',
        headers: {
          'Accept': 'application/json',
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({walletId: accountId, projectId: ideaId})
      });
      const content = await rawResponse.json();
      if(content.likeDeleted) {
       const updatedLikes = allLikes.filter(like => like.id != content.like);
       setAllLikes(updatedLikes);
      }else {
        setAllLikes([...allLikes, content.like]);
      }
    }else {
      setPopupInfo({open: true, msg: 'Please connect wallet to like the idea'});
    }
  }

  const ideas = props.ideas;
  return (
    <React.Fragment>
    {
    ideas &&
      ideas.map((item, id) => {
        let isLiked = allLikes.find(like => like.projectId == item.idea_id);
        let favColor;
        if(isLiked) {
          favColor="#F9ED32";
        }else {
          favColor="#FFF";
        }
        let iconPath;
        if (item.project_phase == 0){
          iconPath = process.env.PUBLIC_URL + '/phase_1.png';
        } else if (item.project_phase == 1){
          iconPath = process.env.PUBLIC_URL + '/phase_2.png';
        } else if (item.project_phase == 2){
          iconPath = process.env.PUBLIC_URL + '/phase_3.png';
        }else{
          iconPath = process.env.PUBLIC_URL + '/phase_4.png';
        }

        return (
          <div className="project-card-wrap" key={id}>
            <div className="col-12 mt-3 card-wrap">
              <div className="card">
                  <div className="d-flex mt-auto flex-column flex-lg-row ">
                    <div className="col-xs-12 col-sm-12 col-lg-4 d-flex justify-content-center align-items-center p-0 img-container" style={{backgroundImage: 'url(' + item.picture_url + ')'}}>
                    </div>
                    <a href={process.env.PUBLIC_URL + '/' + item.idea_id} className="card-content d-flex mb-lg-auto flex-column justify-content-between  col-xs-12 col-sm-12 col-md-12 col-lg-5" style={{textDecoration: 'none'}}>
                      <h4 className="card-title text-center text-md-start text-lg-start" data-image={iconPath} style={{cursor: 'pointer'}} >{item.title}</h4>
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
                    </a>
                    <div className="card-info col-xs-12 col-sm-12 col-lg-3 mt-4 mt-md-0 mt-lg-0">
                      <button className="ms-auto favorite-icon" style={{height: '35px'}} data-idea={item.idea_id} onClick={(e) => likeIdea(e)} >
                        <HeartFill color={favColor} size={25} />;
                      </button>
                      <div className="project-info-wrap">
                        <div className="raised-wrap progress-wrap-card">
                          <div className="info-raised-text">
                            <p className="">Raised</p>
                            <div className="stat-wrap">
                              <span className="out-of ms-1">{item.sum} /</span> <span className="out-of-total">{item.goal_amount}</span>
                            </div>
                          </div>
                          <div className="progress" style={{backgroundColor: "#313131"}}>
                            <div className="progress-bar" style={{ width: ((100 * item.sum) / item.goal_amount ) + '%', background: "linear-gradient(142.91deg, #F9ED32 -54.28%, #E8A523 -12.42%, #CE225B 48.56%, #693E98 100%)" }} role="progressbar" aria-valuenow="50" aria-valuemin="0" aria-valuemax="100"></div>
                          </div>
                        </div>
                        {
                          !isOnProfile ?
                          ( 
                            <div className="invest-wrap d-flex justify-content-start align-items-center">
                              <select className="form-select" defaultValue={0.1} style={{width: '30%'}} aria-label="Default select example" onChange={(e) => setCurrentInvValue(e.target.value)}>
                                {
                                  investOptions.map((option, key) => {
                                    if(option <= (item.goal_amount - item.sum)) {
                                      return (
                                        <option key={key} value={option}>{option}</option>
                                      )
                                    }
                                  })
                                }
                              </select>
                              <img src={`${process.env.PUBLIC_URL}/near-logo-white.png`} className="ms-3" style={{height: '17px', width: '17px'}}/>
                              <Button variant="outline-primary ms-auto tag-btn" data-idea={item.idea_id} onClick={(e) => investInIdea(e)}>INVEST</Button>
                            </div>
                          )
                          :
                            <div className="invest-wrap d-flex justify-content-start align-items-center">
                              <div className="support-wrap">
                                <p className="supporters"><b>{item.investors_count}</b> supportes</p>
                              </div>
                              {
                                item.collect_enabled && !props.isInvestment && 
                                <Button variant="outline-primary ms-auto tag-btn collect-btn" data-idea={item.idea_id} onClick={(e) => props.collectFunds(e)}>
                                  COLLECT
                                </Button>
                              }
                              {
                                !props.isInvestment &&
                                <Button variant="outline-primary ms-auto tag-btn edit-btn" data-idea={item.idea_id} onClick={(e) => props.editIdea(e)}>
                                  EDIT <img src={`${process.env.PUBLIC_URL}/pen.png`} />
                                </Button>
                              }
                            </div>
                        }
                       
                      </div>
                    </div>
                  </div>
              </div>
            </div>
          </div>
        )
      })
    }
    {
      ideas && (props.ideasCount > props.ideaIndex) &&
        <button className="btn header-button mb-3 load-more-btn" onClick={()=> props.loadMoreIdeas()}>
          Load more ideas
        </button>
    }
    {
      popupInfo.open &&
        <Popup msg={popupInfo.msg} setPopupInfo={setPopupInfo} />
    }
    </React.Fragment>
   
  );

};

export default IdeaCard;


