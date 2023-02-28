import React from "react";
import styled from "styled-components";
import { withStyles } from "@material-ui/core/styles";
import Dialog from "@material-ui/core/Dialog";
import MuiDialogTitle from "@material-ui/core/DialogTitle";
import MuiDialogContent from "@material-ui/core/DialogContent";
import Typography from "@material-ui/core/Typography";
import Button from "@material-ui/core/Button";
import MuiDialogActions from "@material-ui/core/DialogActions";

import {useDropzone} from 'react-dropzone';
import { create_idea, editIdea, getIdea } from "../assets/near/utils";
import { useEffect, useState } from "react";

import '../stylesheets/ideaForm.scss';
import { IconButton } from "@material-ui/core";

const ModalBody = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
  background: transparent;
  padding: 50px 70px;

  @media (max-width: 860px) {
    padding: 30px 0;
  }

  .col-wrap {
    @media (max-width: 860px) {
      padding: 0;
    }
  }

  .input-wrap {
    display: flex;
    justify-content: center;
    align-items: center;
    margin-bottom: 20px;

    @media (max-width: 860px) {
      flex-direction: column;
      justify-content: start;
      align-items: start;
    }

    &.textarea-wrap {
      align-items: start;
    }
    
    .form-label {
      margin-bottom: 0;
      font-family: 'Titillium Web';
      font-style: normal;
      font-weight: 300;
      font-size: 18px;
      line-height: 27px;
      color: #FFFFFF;
      width: 100%;

      @media (max-width: 860px) {
        margin-bottom: 10px;
      }

      span {
        font-size: 14px;
        color: #4D4949;
        padding-left: 5px;
      }
    }

    label {
      width: 30%;
    }

    input, textarea {
      background: #000000;
      border: 1px solid #4D4949;
      border-radius: 2px;
      font-family: 'Titillium Web';
      font-style: normal;
      font-weight: 300;
      font-size: 18px;
      line-height: 27px;

      color: #FFFFFF;

      &:focus {
        border: 1px solid #CE225B;
        box-shadow: 0px 0px 35px -5px #CE225B;
      }
    }
  }

  .phase-wrap {
    display: flex;

    @media (max-width: 860px) {
      flex-direction: column;
    }
    
    .input-wrap {
      width: 50%;
      display: flex;
      justify-content: space-between;


      @media (max-width: 860px) {
        width: 100%;
      }

      &:nth-child(2) {
        padding-left: 75px;

        @media (max-width: 860px) {
          padding-left: 0;
        }
      }

      .form-label {
        width: 100%;
      }

      input {
        width: 60%;

        &.active {
          border: 1px solid #CE225B;
          box-shadow: 0px 0px 35px -5px #CE225B;
        }

        @media (max-width: 860px) {
          width: 100%;
        }
      }
    }
  }
`;


const DialogContent = withStyles((theme) => ({
  root: {
    padding: theme.spacing(2),
  },
}))(MuiDialogContent);

function CommentForm(props) {

  const ideaId = props.ideaId;
  const accountId = window.accountId;

  async function submitComment(event) {
    event.preventDefault();
    const formData = new FormData(event.currentTarget);
    formData.append('walletId', accountId);
    formData.append('projectId', ideaId);

    const rawResponse = await fetch('http://mydandelion.app:9999/api/comment', {
      method: 'POST',

      body: formData
    })  
    const response = await rawResponse.json();
    console.log(response);

    if(response.status === 1) {
      props.setOpenCommentForm(false);
      props.setIdeaComments((prev) => [...prev, response.comment]);
    }else {
      alert('Something went wrong, please try again later.');
      props.setOpenCommentForm(false);
    }
  }

  return (
    <div>
      <Dialog
        aria-labelledby="customized-dialog-title"
        open={true}
        fullWidth
        maxWidth="md"
        style={{'zIndex': 2000000}}
        className="idea-form"
      >
        <DialogContent dividers>
          <IconButton aria-label="Close" onClick={(e) => props.setOpenCommentForm(false)}>
            X
          </IconButton>
          <ModalBody fullWidth>
            
            <div className="container-fluid">
              <div className="row">
                <div className="col col-wrap">
                  <form onSubmit={submitComment}>
                    <div className="input-wrap textarea-wrap">
                      <label className="form-label" htmlFor="content">Comment*</label>
                      <textarea name="content" className="form-control" required id="content" rows="3"></textarea>
                    </div>
                    <div className="d-flex justify-content-start align-items-center submit-wrap">
                      <button type="submit" className="submit-idea-btn">SUBMIT</button>
                      <button className="close-btn" onClick={(e) => props.setOpenCommentForm(false)}>CANCEL</button>
                    </div>
                  </form>
                </div>
              </div>
            </div>
          </ModalBody>
        </DialogContent>
      </Dialog>
    </div>
  );
}

export default CommentForm;
