import React from "react";
import {withRouter, Redirect} from "react-router-dom";
import '../stylesheets/page.scss';
import Navbar from "../components/navbar";
import Footer from "../components/footer";




const About = () => {//zamijeniti sa imenom stranice Page

  return (
    <React.Fragment>
      <div className="container-fluid g-0 first-section page">
      <Navbar />
      <div className="container page-wrap">
        <div className="row">
          <h1 className="page-title">About</h1>
          <div className="col-12 content-wrap">


<h3>We are a community of web3 innovators, early adopters and investors where creators can validate their innovative ideas, get them funded and turn them into:</h3>



<h2>THE NEXT BIG THING IN THE BLOCKCHAIN SPACE!</h2>

<p>We are certain that this unique &ldquo;idea-only-based&rdquo; approach will foster immense creativity and encourage investment that will nurture fragile ideas and bring them to life.</p>

<p>As a result, we can hardly wait to see the impact of funded ideas</p>

<h3>OUR MISSION</h3>

<p>To unite people, web3 ideas and empower them to bring those ideas to life.</p>

<h3>OUR VISION</h3>

<p>To be the platform of choice for creators and, investors for ground-breaking web3 ideas whose time has come.</p>

<h3>WE VALUE</h3>


<ul>
	<li><strong>IDEAS</strong> that are innovative and purposefully disruptive.</li>
	<li><strong>TRANSPARENCY</strong> - Open and honest communication about everything.</li>
	<li><strong>ENTHUSIASM</strong>, <strong>SKILL</strong> and the <strong>CAPABILITIES</strong> of creators and the global blockchain community.</li>
	<li><strong>TRUST</strong> - we build trust in every relationship which is what blockchain is all about.</li>
</ul>

            </div>
        </div>
      </div>
      </div>
      <Footer />
    </React.Fragment>
    
  );

};

export default withRouter(About);//zamijeniti sa imenom stranice
