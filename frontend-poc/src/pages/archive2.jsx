import { useEffect, useState } from "react";
import React from "react";
import {withRouter, Redirect} from "react-router-dom";
import Button from 'react-bootstrap/Button';
import { HeartFill } from 'react-bootstrap-icons';
import Dropdown from 'react-bootstrap/Dropdown';
import 'regenerator-runtime/runtime'
import { Swiper, SwiperSlide } from 'swiper/react';
import { Navigation, Pagination, Scrollbar, A11y } from 'swiper';

// Import Swiper styles
import 'swiper/css';

import { Contract } from 'near-api-js'

import './archive2.css';
import '../stylesheets/archive.scss';
import { login, get_all_ideas, get_investment_goal, total_investments, get_investment_for_idea, invest, add_like_to_idea, logout } from "../assets/near/utils";
import Popup from "./popup";
import IdeaForm from "../components/ideaForm";
import Navbar from "../components/navbar";
import Footer from "../components/footer";




const Archive2 = () => {
  
  const [ideas, setIdeas] = useState([]);
  const [currentInvValue, setCurrentInvValue] = useState(0.2);
  const ONE_NEAR= 1000000000000000000000000;
  const [popupInfo, setPopupInfo] = useState({open: false, msg: ''});
  const accountId = window.accountId;

  useEffect(() => { 
    get_all_ideas().then( resIdeas => {
      console.log('ideas from all ideas: ', resIdeas);
      setIdeas(resIdeas);
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

  function useHover(styleOnHover, styleOnNotHover = {}) {
    const [style, setStyle] = React.useState(styleOnNotHover);

    const onMouseEnter = () => setStyle(styleOnHover)
    const onMouseLeave = () => setStyle(styleOnNotHover)

    return {style, onMouseEnter, onMouseLeave}
  }
  
  function walletLogout() {
    logout();
    console.log('AFTER LOGUT: ', accountId)
  }
  return (
    <React.Fragment>
      <div className="container-fluid g-0 first-section">
      <Navbar />
        <div className="container-fluid hero-section">
          <div className="container">
            <div className="row">
              <div className="col-12 col-lg-6">
                <h1 className="hero-title"><b className="bold-glow">Invest early</b> in world-class creators solving real problems using <b className="bold-glow">blockchain technology</b></h1>
                <h2 className="hero-subtitle">Invest in the projects at the earliest stage possible where the opportunities for value creation are highest. Start investing from as low as 0,1        (~0,5$)</h2>
              </div>
              <div className="col-12 col-lg-6">
                <div className="list-wrap list-first-section">
                  <h3 className="info-title">For <b className="bold-glow">Idea Generators</b></h3>
                  <ul className="info-list">
                    <li><p>Receive feedback and financials support at a very early stage - from the idea creation</p></li>
                    <li><p>Gain attraction and recognition very fast</p></li>
                    <li><p>Fail proof multiple ideas in short time</p></li>
                  </ul>
                </div>
              </div>
            </div>
          </div>
          <div className="container hero-second-section-wrap">
            <div className="row pb-5 hero-second-section w-100 m-0">
              <div className="col-12 col-lg-6 second-part">
                <div className="list-wrap">
                  <h3 className="info-title">For <b className="bold-glow">Investors</b></h3>
                  <ul className="info-list">
                    <li><p>Receive feedback and financials support at a very early stage - from the idea creation</p></li>
                    <li><p>Gain attraction and recognition very fast</p></li>
                    <li><p>Fail proof multiple ideas in short time</p></li>
                  </ul>
                </div>
              </div>
              
              <div className="col-12 col-lg-6 second-part-img">
                <img src={`${process.env.PUBLIC_URL}/business-handshake.png`}/> 
              </div>
            </div>
          </div>
        </div>
      </div>
      <div className="container-fluid part-2">
        <div className="container">
          <section id="agenda" className="custom-slider">
            <div className="container-lg py-5 custom-slider-wrap">
              <div className="container-lg px-0 mb-4">
                <div className="row">
                  <div className="col d-flex justify-content-between align-items-center">
                    <h3 className="title-section">Trending</h3>
                    <hr className="solid w-75" />
                    <div className="button-nav-wrap">
                      <button className="btn-archive btn-sm prev-btn" type="button">
                        <img src={`${process.env.PUBLIC_URL}/arrow-prev.png`}/> 
                      </button>
                      <span className="mx-1">&nbsp;</span>
                      <button className="btn-archive btn-sm next-btn" type="button">
                        <img src={`${process.env.PUBLIC_URL}/arrow-next.png`}/> 
                      </button>
                    </div>
                  </div>
                </div>
              </div>
              
              <Swiper 
                modules={[Navigation, Pagination]}
                spaceBetween={10}
                slidesPerView={1}
                navigation={{
                  nextEl: '.next-btn',
                  prevEl: '.prev-btn'
                }}
                pagination={{ 
                  el: '.swiper-pagination',
                  clickable: true 
                }}
                breakpoints={{
                  640: {
                    slidesPerView: 2,
                    spaceBetween: 10,
                  },
                  768: {
                    slidesPerView: 3,
                    spaceBetween: 10,
                  },
              
                  1024: {
                    slidesPerView: 3,
                    spaceBetween: 10,
                  },
                  1280: {
                    slidesPerView: 4,
                    spaceBetween: 10,
                  },
                }}
              >
                <SwiperSlide >
                  <div className="card swiper-card-wrap p-1" {...useHover({"border": "none", "opacity": "0.8", "borderRadius": "5px", "backgroundImage": "linear-gradient(180deg, rgba(249,237,50,0.80) 0%, rgba(232,165,35,0.80) 27%, rgba(206,34,91,0.80) 67%, rgba(105,62,152,0.80) 100%), url('/preuzmi.png')"}, {"borderRadius": "5px", "border": "none", "backgroundImage": `linear-gradient(180deg, rgba(0,0,0,0.00) 0%, rgba(0,0,0,0.01) 3%, #000000 74%), url("/preuzmi.png")`})}>
                    <div className="card-body">
                      <div className="card-title">
                        <h5 className="card-title-text">The Input Technology for the Metaverse</h5>
                      </div>
                      <p className="card-text">Receive feedback and financials support at a very early stage - from the idea creation</p>
                      <div className="progress-wrap-card mt-3">
                        <div className="progress" style={{backgroundColor: "#313131"}}>
                            <div className="progress-bar" style={{ width:  '50%', backgroundColor: "#EEA91E" }} role="progressbar" aria-valuenow="50" aria-valuemin="0" aria-valuemax="100"></div>
                          </div>
                      </div>
                      <div className="progress-text mt-3">
                        <span className="raised">Raised </span>
                        <span className="out-of ms-1">300 /</span> <span className="out-of-total">500</span>
                      </div>
                    </div>
                  </div>
                </SwiperSlide>
                <SwiperSlide >
                  <div className="card swiper-card-wrap p-1" {...useHover({"border": "none", "opacity": "0.8", "borderRadius": "5px", "backgroundImage": "linear-gradient(180deg, rgba(249,237,50,0.80) 0%, rgba(232,165,35,0.80) 27%, rgba(206,34,91,0.80) 67%, rgba(105,62,152,0.80) 100%), url('/preuzmi.png')"}, {"borderRadius": "5px", "border": "none", "backgroundImage": `linear-gradient(180deg, rgba(0,0,0,0.00) 0%, rgba(0,0,0,0.01) 3%, #000000 74%), url("/preuzmi.png")`})}>
                    <div className="card-body">
                      <div className="card-title">
                        <h5 className="card-title-text">The Input Technology for the Metaverse</h5>
                      </div>
                      <p className="card-text">Receive feedback and financials support at a very early stage - from the idea creation</p>
                      <div className="progress-wrap-card mt-3">
                        <div className="progress" style={{backgroundColor: "#313131"}}>
                            <div className="progress-bar" style={{ width:  '50%', backgroundColor: "#EEA91E" }} role="progressbar" aria-valuenow="50" aria-valuemin="0" aria-valuemax="100"></div>
                          </div>
                      </div>
                      <div className="progress-text mt-3">
                        <span className="raised">Raised </span>
                        <span className="out-of ms-1">300 /</span> <span className="out-of-total">500</span>
                      </div>
                    </div>
                  </div>
                </SwiperSlide>
                <SwiperSlide >
                  <div className="card swiper-card-wrap p-1" {...useHover({"border": "none", "opacity": "0.8", "borderRadius": "5px", "backgroundImage": "linear-gradient(180deg, rgba(249,237,50,0.80) 0%, rgba(232,165,35,0.80) 27%, rgba(206,34,91,0.80) 67%, rgba(105,62,152,0.80) 100%), url('/preuzmi.png')"}, {"borderRadius": "5px", "border": "none", "backgroundImage": `linear-gradient(180deg, rgba(0,0,0,0.00) 0%, rgba(0,0,0,0.01) 3%, #000000 74%), url("/preuzmi.png")`})}>
                    <div className="card-body">
                      <div className="card-title">
                        <h5 className="card-title-text">The Input Technology for the Metaverse</h5>
                      </div>
                      <p className="card-text">Receive feedback and financials support at a very early stage - from the idea creation</p>
                      <div className="progress-wrap-card mt-3">
                        <div className="progress" style={{backgroundColor: "#313131"}}>
                            <div className="progress-bar" style={{ width:  '50%', backgroundColor: "#EEA91E" }} role="progressbar" aria-valuenow="50" aria-valuemin="0" aria-valuemax="100"></div>
                          </div>
                      </div>
                      <div className="progress-text mt-3">
                        <span className="raised">Raised </span>
                        <span className="out-of ms-1">300 /</span> <span className="out-of-total">500</span>
                      </div>
                    </div>
                  </div>
                </SwiperSlide>
                <SwiperSlide >
                  <div className="card swiper-card-wrap p-1" {...useHover({"border": "none", "opacity": "0.8", "borderRadius": "5px", "backgroundImage": "linear-gradient(180deg, rgba(249,237,50,0.80) 0%, rgba(232,165,35,0.80) 27%, rgba(206,34,91,0.80) 67%, rgba(105,62,152,0.80) 100%), url('/preuzmi.png')"}, {"borderRadius": "5px", "border": "none", "backgroundImage": `linear-gradient(180deg, rgba(0,0,0,0.00) 0%, rgba(0,0,0,0.01) 3%, #000000 74%), url("/preuzmi.png")`})}>
                    <div className="card-body">
                      <div className="card-title">
                        <h5 className="card-title-text">The Input Technology for the Metaverse</h5>
                      </div>
                      <p className="card-text">Receive feedback and financials support at a very early stage - from the idea creation</p>
                      <div className="progress-wrap-card mt-3">
                        <div className="progress" style={{backgroundColor: "#313131"}}>
                            <div className="progress-bar" style={{ width:  '50%', backgroundColor: "#EEA91E" }} role="progressbar" aria-valuenow="50" aria-valuemin="0" aria-valuemax="100"></div>
                          </div>
                      </div>
                      <div className="progress-text mt-3">
                        <span className="raised">Raised </span>
                        <span className="out-of ms-1">300 /</span> <span className="out-of-total">500</span>
                      </div>
                    </div>
                  </div>
                </SwiperSlide>
                <SwiperSlide >
                  <div className="card swiper-card-wrap p-1" {...useHover({"border": "none", "opacity": "0.8", "borderRadius": "5px", "backgroundImage": "linear-gradient(180deg, rgba(249,237,50,0.80) 0%, rgba(232,165,35,0.80) 27%, rgba(206,34,91,0.80) 67%, rgba(105,62,152,0.80) 100%), url('/preuzmi.png')"}, {"borderRadius": "5px", "border": "none", "backgroundImage": `linear-gradient(180deg, rgba(0,0,0,0.00) 0%, rgba(0,0,0,0.01) 3%, #000000 74%), url("/preuzmi.png")`})}>
                    <div className="card-body">
                      <div className="card-title">
                        <h5 className="card-title-text">The Input Technology for the Metaverse</h5>
                      </div>
                      <p className="card-text">Receive feedback and financials support at a very early stage - from the idea creation</p>
                      <div className="progress-wrap-card mt-3">
                        <div className="progress" style={{backgroundColor: "#313131"}}>
                            <div className="progress-bar" style={{ width:  '50%', backgroundColor: "#EEA91E" }} role="progressbar" aria-valuenow="50" aria-valuemin="0" aria-valuemax="100"></div>
                          </div>
                      </div>
                      <div className="progress-text mt-3">
                        <span className="raised">Raised </span>
                        <span className="out-of ms-1">300 /</span> <span className="out-of-total">500</span>
                      </div>
                    </div>
                  </div>
                </SwiperSlide>
                
              </Swiper>
            </div>
          </section>
          <section className="container-lg projects-wrap pt-5" >
            <div className="mt-5">
              <h5 className="projects-headline">Projects</h5>
            {
              ideas.map((item, id) => {
                return (<div className="row project-card-wrap" key={id}>
                  <div className="col-12 mt-3 card-wrap">
                    <div className="card">
                        <div className="d-flex mt-auto flex-sm-column flex-lg-row row">
                          <div className="col-xs-12 col-sm-12 col-lg-4 d-flex justify-content-center align-items-center p-0 img-container" style={{backgroundImage: 'url(' + item.picture_url + ')'}}>
                            {/* <img className="w-100 h-100" src={item.metadata.picture_url} alt="Card image cap" /> */}
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
                                  <div className="progress-bar" style={{ width: ((100 * item.amount) / item.sum ) + '%', background: "linear-gradient(142.91deg, #F9ED32 -54.28%, #E8A523 -12.42%, #CE225B 48.56%, #693E98 100%)" }} role="progressbar" aria-valuenow="50" aria-valuemin="0" aria-valuemax="100"></div>
                                </div>
                              </div>
                              {/* <div className="supp-wrap d-flex justify-content-between align-items-center mt-3">
                                <p style={{margin: 0}}>Supporters</p>
                                <Button variant="outline-primary">{item.investors > 0 ? item.investors : 0}</Button>
                              </div> */}
                              <div className="invest-wrap d-flex justify-content-start align-items-center">
                                <select className="form-select" defaultValue={0.1} style={{width: '30%'}} aria-label="Default select example" onChange={(e) => setCurrentInvValue(e.target.value)}>
                                  <option value="0.1">0.1</option>
                                  <option value="0.2">0.2</option>
                                  <option value="0.5">0.5</option>
                                  <option value="1">1</option>
                                  <option value="5">5</option>
                                  <option value="10">10</option>
                                </select>
                                {/* <p className="me-3 ms-1">NEAR</p> */}
                                <img src={`${process.env.PUBLIC_URL}/near-logo-white.png`} className="ms-3" style={{height: '17px', width: '17px'}}/>
                                <Button variant="outline-primary ms-auto tag-btn" data-idea={item.idea_id} onClick={(e) => investInIdea(e)}>INVEST</Button>
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
          <section id="news" className="custom-slider">
            <div className="container-lg py-5 news-container-wrap">
              <div className="container-lg px-0 mb-4 container-header-wrap">
                <div className="row">
                  <div className="col d-flex justify-content-between align-items-center">
                    <h3 className="title-section">Latest News</h3>
                    <hr className="solid" />
                    <div className="button-nav-wrap">
                      <button className="btn-archive btn-sm prev-btn" type="button">
                        <img src={`${process.env.PUBLIC_URL}/arrow-prev.png`}/> 
                      </button>
                      <span className="mx-1">&nbsp;</span>
                      <button className="btn-archive btn-sm next-btn" type="button">
                        <img src={`${process.env.PUBLIC_URL}/arrow-next.png`}/> 
                      </button>
                    </div>
                  </div>
                </div>
              </div>
              
              <Swiper 
                modules={[Navigation, Pagination]}
                spaceBetween={10}
                slidesPerView={1}
                navigation={{
                  nextEl: '.next-btn',
                  prevEl: '.prev-btn'
                }}
                pagination={{ 
                  el: '.swiper-pagination',
                  clickable: true 
                }}
                breakpoints={{
                  640: {
                    slidesPerView: 2,
                    spaceBetween: 10,
                  },
                  768: {
                    slidesPerView: 3,
                    spaceBetween: 10,
                  },
              
                  1024: {
                    slidesPerView: 3,
                    spaceBetween: 10,
                  },
                  1280: {
                    slidesPerView: 4,
                    spaceBetween: 10,
                  },
                }}
              >
                <SwiperSlide >
                  <div className="card swiper-card-wrap p-1">
                    <div className="card-body">
                      <div className="card-title">
                        <span className="news-date">12/10/2022    Admin Administrator</span>
                        <h5 className="card-title-text">The Input Technology for the Metaverse</h5>
                      </div>
                      <p className="card-text">Receive feedback and financials support at a very early stage - from the idea creation</p>
                      <a href="#" className="read-more-btn">Read more</a>
                    </div>
                  </div>
                </SwiperSlide>
                <SwiperSlide >
                  <div className="card swiper-card-wrap p-1">
                    <div className="card-body">
                      <div className="card-title">
                        <span className="news-date">12/10/2022    Admin Administrator</span>
                        <h5 className="card-title-text">The Input Technology</h5>
                      </div>
                      <p className="card-text">Receive feedback and financials support at a very early stage - from the idea creation</p>
                      <a href="#" className="read-more-btn">Read more</a>
                    </div>
                  </div>
                </SwiperSlide>
                <SwiperSlide >
                  <div className="card swiper-card-wrap p-1">
                    <div className="card-body">
                      <div className="card-title">
                        <span className="news-date">12/10/2022    Admin Administrator</span>
                        <h5 className="card-title-text">The Input Technology for the Metaverse</h5>
                      </div>
                      <p className="card-text">Receive feedback and financials support at a very early stage - from the idea creation</p>
                      <a href="#" className="read-more-btn">Read more</a>
                    </div>
                  </div>
                </SwiperSlide>
                <SwiperSlide >
                  <div className="card swiper-card-wrap p-1">
                    <div className="card-body">
                      <div className="card-title">
                        <span className="news-date">12/10/2022    Admin Administrator</span>
                        <h5 className="card-title-text">The Input Technology for the Metaverse</h5>
                      </div>
                      <p className="card-text">Receive feedback and financials support at a very early stage - from the idea creation</p>
                      <a href="#" className="read-more-btn">Read more</a>
                    </div>
                  </div>
                </SwiperSlide>
                <SwiperSlide >
                  <div className="card swiper-card-wrap p-1">
                    <div className="card-body">
                      <div className="card-title">
                        <span className="news-date">12/10/2022    Admin Administrator</span>
                        <h5 className="card-title-text">The Input Technology for the Metaverse</h5>
                      </div>
                      <p className="card-text">Receive feedback and financials support at a very early stage - from the idea creation</p>
                      <a href="#" className="read-more-btn">Read more</a>
                    </div>
                  </div>
                </SwiperSlide>
                
              </Swiper>
            </div>
          </section>
        </div>
      </div>
      <Footer />
    {
      popupInfo.open &&
      <Popup msg={popupInfo.msg} setPopupInfo={setPopupInfo} />
    }
    </React.Fragment>
    
  );

};

export default withRouter(Archive2);
