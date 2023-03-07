import React from "react";
import {withRouter, Redirect} from "react-router-dom";
import '../stylesheets/page.scss';
import Navbar from "../components/navbar";
import Footer from "../components/footer";




const How = () => {//zamijeniti sa imenom stranice Page

  return (
    <React.Fragment>
      <div className="container-fluid g-0 first-section page">
      <Navbar />
      <div className="container page-wrap">
        <div className="row">
          <h1 className="page-title">How it works?</h1>
          <div className="col-12 content-wrap">
     

<p><strong>With Dandelion, you can validate your ideas, prove their feasibility, create a detailed delivery plan, and raise the funds you need to bring your project to life.</strong></p>

<h3>CREATORS</h3>

<p>Dandelion is the perfect platform to turn your dreams into reality. You can start by submitting a simple explanation of your idea and why you believe it can work. If your idea is validated, you can then move on to the proof of concept stage, where you can demonstrate that your idea is viable and can be turned into a reality.</p>

<p>Once you have a solid proof of concept, you can create a detailed delivery plan with a timeline and scope of work that needs to be done. During all the phases you will raise the funds you need to get your project off the ground and make it a reality.</p>

<h3>INVESTORS</h3>

<p>Dandelion is a great opportunity to get in on the ground floor of exciting new projects. You can invest in dozens of ideas, read simple explanations and make as many investments as you like. Those ideas that make sense will be funded, and you could be sitting on a fortune from just a few pennies invested. With Dandelion, the sky is the limit for both creators and investors. Join us today and let&#39;s build a better future together!</p>

<p>The platform is designed to have 4 different phases for the projects:</p>

<ol>
	<li>
	<p>Idea validation: In this phase, the project creators are required to write a simple idea explanation and explain why they think it can work.</p>
	</li>
	<li>
	<p>Proof of concept: In this phase, the project creators are required to prove that the idea is feasible and can be turned into reality.</p>
	</li>
	<li>
	<p>Delivery plan: Once the proof of concept has been established, the project creators are required to create a detailed delivery plan with a timeline and scope of work that needs to be done. They also raise money to finance the project.</p>
	</li>
	<li>
	<p>Presale: After the project is completed, a presale of the token begins. For earlier investors tokens are released through airdrop.</p>
	</li>
</ol>
<p>IMPORTANT!! <br></br>If goal is not reached within 90 days, investments are reverted to investors and idea is closed.</p>


            </div>
        </div>
      </div>
      </div>
      <Footer />
    </React.Fragment>
    
  );

};

export default withRouter(How);//zamijeniti sa imenom stranice
