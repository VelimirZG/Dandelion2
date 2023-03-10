import React from "react";
import '../stylesheets/footer.scss';

const Footer = () => {

  return (
    <div className="container-fluid g-0 footer">
      <div className="footer-inner container">
        <div className="row main-footer">
          <div className="col-12 col-lg-3">
              <img src={`${process.env.PUBLIC_URL}/logo-footer.png`} />
          </div>
          <div className="col-12 col-lg-6 main-footer-menu-wrap d-flex">
            <div className="col-6 col-lg-6 f-menu-1">
              <h6 className="footer-menu-title">About</h6>
              <ul className="footer-menu-ul">
                <li><a href="./about">About us</a></li>
                <li><a href="#">Careers</a></li>
              
              </ul>
            </div>
            <div className="col-6 col-lg-6">
              <h6 className="footer-menu-title">Support</h6>
              <ul className="footer-menu-ul">
                <li><a href="./how">How it works</a></li>
             
                <li><a href="./faq">FAQ</a></li>
                
              </ul>
            </div>
          </div> 
          <div className="col-12 col-lg-3 f-menu-2">
            <h6 className="footer-menu-title">More from us</h6>
            <ul className="footer-menu-ul">
              <li><a href="#">Twitter</a></li>
              <li><a href="#">Medium</a></li>
              <li><a href="#">Discord</a></li>
            </ul>
          </div>
        </div>
        <hr />
        <div className="footer-2">
            <div className="row second-footer">
              <div className="col-12 col-lg-6">
                <p>Dandelion, 2023. All rights reserved.</p>
              </div>
              <div className="col-12 col-lg-6 footer-links">
                <a href="./privacy">Privacy policy</a>
                <a href="./terms">Terms of use</a>
              </div>
            </div>
        </div>
      </div>
    </div>
  );

};

export default Footer;
