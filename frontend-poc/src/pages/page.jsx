import React from "react";
import {withRouter, Redirect} from "react-router-dom";
import '../stylesheets/page.scss';
import Navbar from "../components/navbar";
import Footer from "../components/footer";




const Page = () => {//zamijeniti sa imenom stranice Page

  return (
    <React.Fragment>
      <div className="container-fluid g-0 first-section page">
      <Navbar />
      <div className="container page-wrap">
        <div className="row">
          <h1 className="page-title">Pravila privatnosti</h1>
          <div className="col-12 content-wrap">
                <h1>Header 1</h1>
                <h2>Header 2</h2>
                <h3>Header 3</h3>
              <p>
              Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
              </p>
              <ul>
              <li>Lorem ipsum dolor sit amet, consectetur adipiscing elit.</li>
              <li>Aenean semper neque ac nibh semper luctus.</li>
              </ul>
              <p></p>
              <p>
              </p><ul>
              <li>Cras non metus quis arcu tincidunt suscipit elementum nec orci.</li>
              </ul>
              <p></p>
              <p>
              </p><ul>
              <li>Nunc hendrerit libero et ante dictum, sed convallis nisi tristique.</li>
              </ul>
              <p></p>
              <p>
              </p><ul>
              <li>Nulla et ex quis dolor volutpat interdum id at felis.</li>
              <li>Nam pharetra felis eget purus varius auctor.</li>
              <li>Curabitur in risus iaculis, dictum quam at, finibus lacus.</li>
              <li>Phasellus a massa ut mi ullamcorper porttitor.</li>
              <li>Nunc aliquam diam vitae ligula placerat, quis venenatis sapien vestibulum.</li>
              </ul>
              <p></p>
              <p>
              </p><ul>
              <li>Phasellus aliquet nibh sodales tellus ullamcorper euismod.</li>
              <li>Duis maximus sem a quam eleifend, luctus elementum massa convallis.</li>
              </ul>
              <p></p>
            </div>
        </div>
      </div>
      </div>
      <Footer />
    </React.Fragment>
    
  );

};

export default withRouter(Page);//zamijeniti sa imenom stranice
