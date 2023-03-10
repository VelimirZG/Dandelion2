import React from "react";
import {withRouter, Redirect} from "react-router-dom";
import '../stylesheets/page.scss';
import Navbar from "../components/navbar";
import Footer from "../components/footer";




const Privacy = () => {//zamijeniti sa imenom stranice Page

  return (
    <React.Fragment>
      <div className="container-fluid g-0 first-section page">
      <Navbar />
      <div className="container page-wrap">
        <div className="row">
          <h1 className="page-title">Privacy policy</h1>
          <div className="col-12 content-wrap">
     

          <p>At Dandelion, we respect your privacy. This Privacy Policy explains how we collect, use, and protect the information provided to us by both idea creators and investors when using our decentralized and open-source platform for blockchain and web3 innovations.</p>

<p>Information We Collect</p>

<p>We may collect the following types of information:</p>

<ul>
	<li>Wallet Information: We collect information about your cryptocurrency wallet in order to facilitate investments and payouts.</li>
	<li>Investment Information: We collect information about the investments you make on our platform, including the amount and date of the investment.</li>
	<li>Idea Information: We collect information about the ideas shared on our platform by creators, including the idea description, feasibility proof, delivery plan, and presale details.</li>
</ul>

<p>How We Use Your Information</p>

<p>We may use your information for the following purposes:</p>

<ul>
	<li>To facilitate investments: We use your wallet and investment information to facilitate investments and payouts.</li>
	<li>To improve our platform: We use your investment and idea information to understand how our platform is used and to make improvements.</li>
	<li>To communicate with you: We use your investment and idea information to communicate with you about your investments, the progress of the idea you invested in, and our platform.</li>
</ul>

<p>Information Sharing and Disclosure</p>

<p>We do not share your information with third parties except in the following circumstances:</p>

<ul>
	<li>With service providers: We may share your information with service providers who help us operate our platform or provide our services.</li>
	<li>In response to legal process: We may disclose your information in response to a subpoena, court order, or other legal process.</li>
</ul>

<p>Data Security</p>

<p>We take reasonable measures to protect your information from unauthorized access, use, and disclosure. However, no security system is perfect, and we cannot guarantee the security of your information.</p>

<p>Changes to this Privacy Policy</p>

<p>We may update this Privacy Policy from time to time. If we make material changes to the policy, we will notify you by email or by posting a notice on our platform.</p>

<p>Creators Disclaimer</p>

<p>As a creator, you understand and agree that the ideas you share on our platform are public and can be viewed by anyone. You acknowledge that you are solely responsible for the ideas you share, and that Dandelion is not liable for any damages or losses resulting from the sharing of your ideas.</p>

<p>Contact Us</p>

<p>If you have any questions about this Privacy Policy or the creators&#39; disclaimer, please contact us at info@mydandelion.app.</p>



            </div>
        </div>
      </div>
      </div>
      <Footer />
    </React.Fragment>
    
  );

};

export default withRouter(Privacy);//zamijeniti sa imenom stranice
