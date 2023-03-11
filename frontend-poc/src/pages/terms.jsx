import React from "react";
import {withRouter, Redirect} from "react-router-dom";
import '../stylesheets/page.scss';
import Navbar from "../components/navbar";
import Footer from "../components/footer";




const Terms = () => {//zamijeniti sa imenom stranice Page

  return (
    <React.Fragment>
      <div className="container-fluid g-0 first-section page">
      <Navbar />
      <div className="container page-wrap">
        <div className="row">
          <h1 className="page-title">Terms of use</h1>
          <div className="col-12 content-wrap">
     

<p>Welcome to Dandelion, the community platform for blockchain and web3 innovators. By accessing or using our services, you agree to be bound by these Terms of Use. Please read them carefully before using our platform.</p>

<p>Description of Service</p>

<p>Dandelion is a decentralized and open-source platform that provides resources for idea creators and investors in the blockchain and web3 space. For creators, Dandelion offers exposure, validation, and the ability to raise funds to bring their ideas to life. For investors, Dandelion offers access to innovative projects and the ability to diversify their portfolio. The platform is designed with four different phases for projects: idea validation, proof of concept, delivery plan, and presale.</p>

<p>Use of the Platform</p>

<p>You may use Dandelion as an idea creator or investor, provided that you comply with these Terms of Use and any applicable laws and regulations. You are solely responsible for any content you post on the platform, and you represent and warrant that you have all necessary rights to post such content.</p>

<p>Fee policy</p>

<p>Please note that we charge a fee of 0.25% on investments made through our platform. This fee covers the cost of operating our platform and providing our services to you. By using our platform, you agree to pay this fee. This fee is deducted from the total amount raised. We do not charge any fees for unsuccessful campaigns</p>

<p>No Investment Advice</p>

<p>Dandelion does not provide investment advice, and any investment decisions you make are solely your responsibility. Dandelion is not liable for any investment losses you may incur as a result of using our platform.</p>

<p>No Liability for Creators or Code Contributors</p>

<p>Dandelion is an open-source platform, and its creators and code contributors are not liable for any damages or losses you may incur as a result of using our platform. By using Dandelion, you agree to release and hold harmless its creators and code contributors from any claims, damages, or losses.&nbsp;Dandelion does not warrant that the platform will be uninterrupted or error-free, or that any defects will be corrected. You use the platform at your own risk.</p>

<p>Termination</p>

<p>We reserve the right to terminate or suspend your access to the platform at any time and for any reason, without notice or liability. Upon termination, you will no longer have access to the platform or any content you posted on the platform.</p>

<p>Changes to the Terms of Use</p>

<p>We may modify these Terms of Use at any time and without notice. Your continued use of the platform after any such modifications constitutes your acceptance of the revised Terms of Use.</p>

<p>Governing Law and Arbitration</p>

<p>These Terms of Use shall be governed by and construed in accordance with the laws of [insert governing law]. Any dispute arising out of or in connection with these Terms of Use shall be resolved by binding arbitration in accordance with the rules of [insert arbitration rules].</p>

<p>Contact Us</p>

<p>If you have any questions or comments about these Terms of Use, please contact us at info@mydandelion.app.</p>


            </div>
        </div>
      </div>
      </div>
      <Footer />
    </React.Fragment>
    
  );

};

export default withRouter(Terms);//zamijeniti sa imenom stranice
