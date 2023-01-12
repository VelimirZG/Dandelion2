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

const DialogActions = withStyles((theme) => ({
  root: {
    margin: 0,
    padding: theme.spacing(1),
  },
}))(MuiDialogActions);

const DialogTitle = withStyles(styles)((props) => {
  const { children, classes, onClose, ...other } = props;
  return (
    <MuiDialogTitle disableTypography className={classes.root} {...other}>
      <Typography variant="h6">{children}</Typography>
    </MuiDialogTitle>
  );
});

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
    
    let metadata = ['website', 'title', 'description', 'picture_url', 'team', 'excerpt', 'value_proposition', 'owner_id'];
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
      create_idea(data).then((response)=>
        console.log('response from create idea: ', response)
      )
    }
  }

  const renderPhasesInput = () => {
    if(!ideaInfo) {
      return;
    }
    let content = [];
    ideaInfo.investments.forEach((inv, i) => {
      let disabled = false;
      if(inv.goal_reached) {
        disabled = true;
      }else {
        if(ideaInfo.investments[i-1].goal_reached) {
          disabled = false
        }else {
          disabled = true;
        }
      }
      content.push(
        <div className="mb-5">
          <label className="form-label" htmlFor={"amount" + (i+1)}>Phase {i +1} goal:</label>
          <input name={"amount" + (i+1)} type="number" className="form-control" id={"amount" + (i+1)} readOnly={disabled} defaultValue={ inv.goal } />
        </div>
      );
    })
    
    return content;
  }

  return (
    <div>
      <Dialog
        aria-labelledby="customized-dialog-title"
        open={true}
        fullWidth
        maxWidth="md"
        style={{'zIndex': 2000000}}
      >
        <DialogTitle id="customized-dialog-title" style={{textAlign: 'center', fontWeight: 'bold'}}>
          {ideaInfo ? 'UPDATE YOUR IDEA' : 'CREATE NEW IDEA'}
        </DialogTitle>
        <DialogContent dividers>
          <ModalBody fullWidth>
            <div className="container-fluid">
              <div className="row">
                <div className="col">
                <form onSubmit={submitIdea}>
                  <div className="mb-3">
                    <label htmlFor="title" className="form-label">Title:</label>
                    <input type="text" name="title" className="form-control" id="title" aria-describedby="title" defaultValue={ideaInfo ? ideaInfo.title : ''}/>
                  </div>
                  <div className="mb-3">
                    <label htmlFor="excerpt">Excerpt:</label>
                    <textarea name="excerpt" className="form-control" id="excerpt" rows="3" defaultValue={ideaInfo ? ideaInfo.excerpt : ''}></textarea>
                  </div>
                  <div className="mb-3">
                    <label htmlFor="description">Description:</label>
                    <textarea name="description" className="form-control" id="description" rows="3" defaultValue={ideaInfo ? ideaInfo.description : ''}></textarea>
                  </div>
                  {/* <div className="mb-3">
                    <label htmlFor="investment_goal" className="form-label">Investment goal:</label>
                    <input name="investment_goal" type="number" className="form-control" id="investment_goal" aria-describedby="investment_goal" />
                  </div> */}
                  <div className="mb-3">
                    <label htmlFor="team" className="form-label">Team:</label>
                    <input name="team" type="text" className="form-control" id="team" aria-describedby="team" defaultValue={ideaInfo ? ideaInfo.team : ''} />
                  </div>
                  <div className="mb-3">
                    <label htmlFor="tags" className="form-label">Tags (devide by comma):</label>
                    <input name="tags" type="text" className="form-control" id="tags" aria-describedby="tags" defaultValue={ideaInfo ? ideaInfo.tags.join(',') : ''} />
                  </div>
                  <div className="mb-3">
                    <label htmlFor="competitors" className="form-label">Competitors:</label>
                    <input name="competitors" type="text" className="form-control" id="competitors" aria-describedby="competitors" defaultValue={ideaInfo ? ideaInfo.competitors.join(',') : ''} />
                  </div>
                  <div className="mb-3">
                    <label htmlFor="value_proposition" className="form-label">Value proposition:</label>
                    <input name="value_proposition" type="text" className="form-control" id="value_proposition" aria-describedby="value_proposition" defaultValue={ideaInfo ? ideaInfo.value_proposition : ''} />
                  </div>
                  <div className="mb-5">
                    <label className="form-label" htmlFor="website">External website URL:</label>
                    <input name="website" type="url" className="form-control" id="website" defaultValue={ideaInfo ? ideaInfo.website : ''}/>
                  </div>
                  <div className="mb-5">
                    <label className="form-label" htmlFor="picture_url">Idea image file path:</label>
                    <input name="picture_url" type="url" className="form-control" id="picture_url" defaultValue={ideaInfo ? ideaInfo.picture_url : ''}/>
                  </div>
                  {ideaInfo ? 
                    renderPhasesInput()
                    : 
                    <div className="mb-5">
                      <label className="form-label" htmlFor="amount1">Phase 1 goal:</label>
                      <input name="amount1" type="number" className="form-control" id="amount1" />
                    </div>
                }
                  <div className="mb-3 d-flex justify-content-between align-items-center">
                    <button className="btn btn-danger" onClick={(e) => props.setOpenIdeaForm(false)}>CANCEL</button>
                    <button type="submit" className="btn btn-primary">SUBMIT</button>
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
