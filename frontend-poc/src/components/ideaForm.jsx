import React from "react";
import styled from "styled-components";
import { withStyles } from "@material-ui/core/styles";
import Dialog from "@material-ui/core/Dialog";
import MuiDialogTitle from "@material-ui/core/DialogTitle";
import MuiDialogContent from "@material-ui/core/DialogContent";
import Typography from "@material-ui/core/Typography";
import Button from "@material-ui/core/Button";
import MuiDialogActions from "@material-ui/core/DialogActions";

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

function IdeaForm(props) {

  const [ideaInfo, setIdeaInfo] = useState(false);

  useEffect(() => {
    if(props.ideaId) {
      getIdea(props.ideaId).then( idea => {
        console.log(idea);
        setIdeaInfo(idea);
      });
    }
  },[]);

  function submitIdea(event) {
    event.preventDefault();
    const formData = new FormData(event.currentTarget);
    if(ideaInfo) {
      formData.append('idea_id', parseInt(props.ideaId));
    }else {
      formData.append('idea_id', parseInt(new Date().getTime()))
    }
    
    formData.append('owner_id', window.accountId);
    console.log('FROM DATA: ', formData.entries());
    
    let metadata = ['website', 'title', 'description', 'picture_url', 'team', 'excerpt', 'value_proposition', 'owner_id', 'token_contract'];
    let data = {};
    for (let [key, value] of formData.entries()) {
      if(metadata.includes(key)) {
        if(data['metadata']) {
          data['metadata'][key] = value;
        }else {
          data['metadata'] = {};
          data['metadata'][key] = value;
        }
      }else if(key === 'competitors') {
        let competitors = value.split(',');
        data['metadata'][key] = competitors;
      }else if(key === 'tags') {
        let tags = value.split(',');
        data['metadata'][key] = tags;
      }else if(key === 'idea_id') {
        data['idea_id'] = parseInt(value);
      }else {
        data[key] = parseInt(value);
      }
    }
    console.log('DATA:', data);
    if(ideaInfo) {
      editIdea(data).then((response) => {
        console.log('response from create idea: ', response)
      });
    }else {
      data['amount2'] = 0;
      data['amount3'] = 0;
      data['amount4'] = 0;
      create_idea(data).then((response)=> {
        console.log('response from create idea: ', response);
        window.location.href = '/profile';
      })
    }
  }

  const renderPhasesInput = () => {
    if(!ideaInfo) {
      return;
    }
    let content = [];
    let res = [];
    ideaInfo.investments.forEach((inv, i) => {
      let disabled = false;
      if(inv.goal_reached) {
        disabled = true;
      }else {
        if (i > 0){
          if(ideaInfo.investments[i-1].goal_reached) {
            disabled = false
          }else {
            disabled = true;
          } 
        }else {
          disabled = false;
        }
        
      }
      
      if(i == 2) {
        content = [];
      }
      content.push(
        <div className="input-wrap">
          <label className="form-label" htmlFor={"amount" + (i+1)}>Phase {i +1} goal:</label>
          <input name={"amount" + (i+1)} type="number" className={!disabled ? 'form-control active' : 'form-control'} id={"amount" + (i+1)} readOnly={disabled} defaultValue={ inv.goal } />
        </div>
      );    

      if(i == 1 || i == 3) {
        res.push(<div className="phase-wrap">{content}</div>);
      }
    })
    
    return res;
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
          <IconButton aria-label="Close" onClick={(e) => props.setOpenIdeaForm(false)}>
            X
          </IconButton>
          <ModalBody fullWidth>
            
            <div className="container-fluid">
              <div className="row">
                <div className="col col-wrap">
                  <form onSubmit={submitIdea}>
                    <div className="input-wrap">
                      <label htmlFor="title" className="form-label">Title* <span>(max 100 characters)</span></label>
                      <input type="text" name="title" required className="form-control" id="title" aria-describedby="title" maxLength={100} defaultValue={ideaInfo ? ideaInfo.title : ''}/>
                    </div>
                    <div className="input-wrap textarea-wrap">
                      <label className="form-label" htmlFor="excerpt">Excerpt* <span>(max 200 characters)</span></label>
                      <textarea name="excerpt" className="form-control" required id="excerpt" rows="3" maxLength={200} defaultValue={ideaInfo ? ideaInfo.excerpt : ''}></textarea>
                    </div>
                    <div className="input-wrap textarea-wrap">
                      <label className="form-label" htmlFor="description">Description*</label>
                      <textarea name="description" className="form-control" required id="description" rows="3" defaultValue={ideaInfo ? ideaInfo.description : ''}></textarea>
                    </div>
                    {/* <div className="input-wrap">
                      <label htmlFor="investment_goal" className="form-label">Investment goal:</label>
                      <input name="investment_goal" type="number" className="form-control" id="investment_goal" aria-describedby="investment_goal" />
                    </div> */}
                    <div className="input-wrap">
                      <label htmlFor="team" className="form-label">Team*</label>
                      <input name="team" type="text" className="form-control" required id="team" aria-describedby="team" defaultValue={ideaInfo ? ideaInfo.team : ''} />
                    </div>
                    <div className="input-wrap">
                      <label htmlFor="tags" className="form-label">Tags <span>(divide by comma)*</span></label>
                      <input name="tags" type="text" className="form-control" id="tags" required aria-describedby="tags" defaultValue={ideaInfo ? ideaInfo.tags.join(',') : ''} />
                    </div>
                    <div className="input-wrap">
                      <label htmlFor="competitors" className="form-label">Competitors</label>
                      <input name="competitors" type="text" className="form-control" id="competitors" aria-describedby="competitors" defaultValue={ideaInfo ? ideaInfo.competitors.join(',') : ''} />
                    </div>
                    <div className="input-wrap">
                      <label htmlFor="value_proposition" className="form-label">Value proposition*</label>
                      <input name="value_proposition" type="text" className="form-control" required id="value_proposition" aria-describedby="value_proposition" defaultValue={ideaInfo ? ideaInfo.value_proposition : ''} />
                    </div>
                    <div className="input-wrap">
                      <label className="form-label" htmlFor="website">External website URL</label>
                      <input name="website" type="url" className="form-control" id="website" defaultValue={ideaInfo ? ideaInfo.website : ''}/>
                    </div>
                    <div className="input-wrap">
                      <label className="form-label" htmlFor="picture_url">Idea image file path*</label>
                      <input name="picture_url" type="url" className="form-control" required id="picture_url" defaultValue={ideaInfo ? ideaInfo.picture_url : ''}/>
                    </div>
                    <div className="input-wrap">
                      <label htmlFor="token_contract" className="form-label">Token contract address*<span>(mandatory before 3. goal)*</span></label>
                      <input name="token_contract" type="text" className="form-control" required id="token_contract" aria-describedby="token_contract" defaultValue={ideaInfo ? ideaInfo.token_contract : ''} />
                    </div>
                    <h3 className="form-header">Phases</h3>
                    <div className="phases-wrap">
                    {ideaInfo ? 
                      renderPhasesInput()
                      : 
                      <div className="phase-wrap">
                        <div className="input-wrap">
                          <label className="form-label" htmlFor="amount1">Phase 1 goal*</label>
                          <input name="amount1" type="number" required className="form-control" id="amount1" />
                        </div>
                      </div> 
                    }
                    </div>
                    <div className="d-flex justify-content-start align-items-center submit-wrap">
                      <button type="submit" className="submit-idea-btn">SUBMIT</button>
                      <button className="close-btn" onClick={(e) => props.setOpenIdeaForm(false)}>CANCEL</button>
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

export default IdeaForm;
