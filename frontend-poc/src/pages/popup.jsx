import React from "react";
import styled from "styled-components";
import { withStyles } from "@material-ui/core/styles";
import Dialog from "@material-ui/core/Dialog";
import MuiDialogTitle from "@material-ui/core/DialogTitle";
import MuiDialogContent from "@material-ui/core/DialogContent";
import Typography from "@material-ui/core/Typography";
import Button from "@material-ui/core/Button";
import MuiDialogActions from "@material-ui/core/DialogActions";


import Modal from 'react-bootstrap/Modal';


import '../stylesheets/popups.scss';
import { useEffect } from "react";

const styles = (theme) => ({
  root: {
    margin: 0,
    padding: theme.spacing(2),
  },
  closeButton: {
    position: "absolute",
    right: theme.spacing(1),
    top: theme.spacing(1),
    color: theme.palette.grey[500],
  },
  paper: {
    backgroundColor: "rgba(0,0,0,1)",
  },
});

const ModalBody = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;

  div {
    margin: 5px;
  }
  p {
    margin: 20px;
  }
  .data-privacy {
    max-height: 300px;
    overflow: hidden;
    overflow-y: scroll;
    border-color: black;
    border-width: thin;
    border-style: solid;
  }
  .agree-or-not {
    display: flex;
    flex-direction: row;
    align-items: space-between;
    button {
      margin: 20px;
    }
  }
`;

function Popup(props) {


  useEffect(() => {
    document.body.style.overflowY = 'hidden';

    return () => {
      document.body.style.overflowY = 'auto';
    }    
  }, []);

  return (
    <div className="custom-modal ">
      <div className="custom-modal-content container">
        <div className="row modal-wrap">
          <div className="col-12 col-lg-8 modal-content-wrap">
            <span className="close" onClick={() => props.setPopupInfo({open: false, msg: ''})}>&times;</span>
            <img src={`${process.env.PUBLIC_URL}/popup-img.png`} />
            <h3>Ooops!</h3>
            <p>{props.msg}</p>
          </div>
        </div>
      </div>
    </div>
  );
}

export default Popup;
