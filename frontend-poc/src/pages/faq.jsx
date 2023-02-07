import React from "react";
import {withRouter, Redirect} from "react-router-dom";
import '../stylesheets/page.scss';
import Navbar from "../components/navbar";
import Footer from "../components/footer";




const faq = () => {//zamijeniti sa imenom stranice Page

  return (
    <React.Fragment>
      <div className="container-fluid g-0 first-section page">
      <Navbar />
      <div className="container page-wrap">
        <div className="row">
          <h1 className="page-title">Frequently asked questions</h1>
          <div className="col-12 content-wrap"><p>Q: What is Dandelion?</p>

                <p>A: Dandelion is a decentralized platform that utilizes blockchain technology to connect creators and investors. It allows creators to receive financial support and feedback for their ideas, while investors can discover and invest in innovative projects.</p>

                <p>Q: What makes a Dandelion platform different from traditional investment platforms?</p>

                <p>A: Dandelion provide opportunities for creators to receive early stage support and feedback for their ideas, which can help them bring their projects to market faster.</p>

                <p>Q: Who can participate?</p>

                <p>A: Both creators and investors can participate in a web3 investment platform. Creators can submit their ideas for funding and feedback, while investors can discover and invest in innovative projects.</p>

                <p>Q: What kind of projects can I find on Dandelion?</p>

                <p>A: You can find a wide range of projects, including blockchain-based applications, web3 technologies, and decentralized systems. These projects are focused on solving real problems and improving the way we live, work, and interact with the world.</p>

                <p>Q: Is it safe to invest in a web3 projects?</p>

                <p>A: As with any investment, there is always risk involved. However, web3 investment platforms utilize blockchain technology to provide a more secure and transparent investment process. Additionally, by investing in an idea, you have the opportunity to diversify your portfolio and invest in a range of innovative projects.</p>

                <p>Q: How do I submit an idea for investment on the platform?</p>

                <p>A: To submit an idea, you must first connect your wallet. Once your wallet is connected, you can submit your idea by filling out the required information. If you have completed your submission successfully it will be visible immediately. Idea should be submitted according to guideline in terms and policies, othervise it will be removed.</p>

                <p>Q: How do I invest in an idea on the platform?</p>

                <p>A: You can search for ideas that interest you and review their details. If you decide to support the idea, you can select the amount you would like to invest and complete the transaction through your Near wallet.</p>

                <p>Q: What types of ideas are accepted on the platform?</p>

                <p>A: The platform is focused on innovative and impactful ideas in the blockchain and web3 space. We are looking for ideas that have the potential to solve real problems and create positive change in the world.</p>

                <p>Q: What happens after I invest in an idea?</p>

                <p>A: After you invest in an idea, you will become eligible for the airdrop in the percentage of the investment amount for that phase. Under the dashboard you will be able to track its progress and get regular update.</p>

            </div>
        </div>
      </div>
      </div>
      <Footer />
    </React.Fragment>
    
  );

};

export default withRouter(faq);//zamijeniti sa imenom stranice
