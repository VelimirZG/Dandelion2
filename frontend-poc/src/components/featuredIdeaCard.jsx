import React from "react";
import { Swiper, SwiperSlide } from 'swiper/react';
import { Navigation, Pagination, Scrollbar, A11y } from 'swiper';

import '../stylesheets/featuredIdeaCard.scss';

const FeaturedIdeaCard = (props) => {


  const ideas = props.ideas ? props.ideas : [];

  function SetCardHover(styleOnHover, styleOnNotHover = {}) {
    const [style, setStyle] = React.useState(styleOnNotHover);

    const onMouseEnter = () => setStyle(styleOnHover)
    const onMouseLeave = () => setStyle(styleOnNotHover)

    return {style, onMouseEnter, onMouseLeave}
  }

  return (
    <React.Fragment>
       <Swiper 
          modules={[Navigation, Pagination]}
          spaceBetween={10}
          slidesPerView={1}
          className="featured-idea-card"
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
          {
          ideas &&
            ideas.map((item, id) => {

              let style = SetCardHover({
                "border": "none",
                "opacity": "0.8",
                "borderRadius": "5px",
                "backgroundImage": "linear-gradient(180deg, rgba(249,237,50,0.80) 0%, rgba(232,165,35,0.80) 27%, rgba(206,34,91,0.80) 67%, rgba(105,62,152,0.80) 100%), url('" + item.picture_url + "')",
                "backgroundSize": "cover"
              }, {
                "borderRadius": "5px",
                "border":"none",
                "backgroundImage": `linear-gradient(180deg, rgba(0,0,0,0.00) 0%, rgba(0,0,0,0.01) 3%, #000000 74%), url("` + item.picture_url + `")`,
                "backgroundSize": "cover"
              });
              return (
                <SwiperSlide onClick={() => { window.location.href= process.env.PUBLIC_URL + '/' + item.idea_id}} style={{cursor: 'pointer'}}>
                  <div className="card swiper-card-wrap p-1" {...style}>
                    <div className="card-body">
                      <div className="card-title">
                        <h5 className="card-title-text">{item.title}</h5>
                      </div>
                      <p className="card-text">{item.excerpt}</p>
                      <div className="progress-wrap-card mt-3">
                        <div className="progress" style={{backgroundColor: "#313131"}}>
                            <div className="progress-bar" style={{ width:  ((100 * item.sum) / item.goal_amount ) + '%', backgroundColor: "#EEA91E" }} role="progressbar" aria-valuenow="50" aria-valuemin="0" aria-valuemax="100"></div>
                          </div>
                      </div>
                      <div className="progress-text mt-3">
                        <span className="raised">Raised </span>
                        <span className="out-of ms-1">{item.sum} /</span> <span className="out-of-total">{item.goal_amount}</span>
                      </div>
                    </div>
                  </div>
                </SwiperSlide>
              )
            })
          }
        </Swiper>
    </React.Fragment>
   
  );

};

export default FeaturedIdeaCard;


