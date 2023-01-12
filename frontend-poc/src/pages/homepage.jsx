import { useEffect, useState } from "react";
import React from "react";
import {withRouter, Redirect} from "react-router-dom";
import 'regenerator-runtime/runtime'
import { Swiper, SwiperSlide } from 'swiper/react';
import { Navigation, Pagination, Scrollbar, A11y } from 'swiper';

// Import Swiper styles
import 'swiper/css';

import { Contract } from 'near-api-js'

import './homepage.css';
import '../stylesheets/homepage.scss';
import { get_all_ideas, logout } from "../assets/near/utils";
import Navbar from "../components/navbar";
import Footer from "../components/footer";
import IdeaCard from "../components/ideaCard";




const Homepage = () => {
  
  const [ideas, setIdeas] = useState([]);
  const [index, setIndex] = useState(0);
  const limit = 20;
  
  useEffect(() => { 
    listIdeas();
  }, [] )


  function listIdeas(nextPage = false) {
    get_all_ideas(index, limit).then( res => {
      console.log('ideas from all ideas: ', res);
      console.log([...ideas, ...res]);
      if(nextPage) {
        setIdeas([...ideas, ...res])
      }else {
        setIdeas(res);
      }
      console.log(ideas);
    });
  }

 
  function useHover(styleOnHover, styleOnNotHover = {}) {
    const [style, setStyle] = React.useState(styleOnNotHover);

    const onMouseEnter = () => setStyle(styleOnHover)
    const onMouseLeave = () => setStyle(styleOnNotHover)

    return {style, onMouseEnter, onMouseLeave}
  }

  function loadMoreIdeas() {
    setIndex(index + limit);
    listIdeas(true);
  }
  
  // function walletLogout() {
  //   logout();
  //   console.log('AFTER LOGUT: ', accountId)
  // }
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
          <section className="container-lg projects-wrap pt-5 mt-5 d-flex flex-column">
              <h5 className="projects-headline">Projects</h5>
              <IdeaCard ideas={ideas} loadMoreIdeas={loadMoreIdeas} />
             
            {/* </div> */}
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
    </React.Fragment>
    
  );

};

export default withRouter(Homepage);
