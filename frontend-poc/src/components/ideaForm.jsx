import React from "react";
import styled from "styled-components";
import { withStyles } from "@material-ui/core/styles";
import Dialog from "@material-ui/core/Dialog";
import MuiDialogTitle from "@material-ui/core/DialogTitle";
import MuiDialogContent from "@material-ui/core/DialogContent";
import Typography from "@material-ui/core/Typography";
import Button from "@material-ui/core/Button";
import MuiDialogActions from "@material-ui/core/DialogActions";

import { create_idea } from "../assets/near/utils";

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


  function submitIdea(event) {
    event.preventDefault();
    const formData = new FormData(event.currentTarget);
    formData.append('idea_id', Date.now())
    formData.append('owner_id', window.accountId);
    console.log('FROM DATA: ', formData.entries());

    let metadata = ['title', 'description', 'picture_url', 'team', 'competitors', 'excerpt', 'value_proposition'];
    let data = {};
    for (let [key, value] of formData.entries()) {
      console.log(key, value);
      if(metadata.includes(key)) {
        if(data['metadata']) {
          data['metadata'][key] = value; 
        }else {
          data['metadata'] = {};
          data['metadata'][key] = value;
        }
      }else if(key === 'tags') {
        let tags = value.split(',');
        data['metadata'][key] = tags;
      }else {
        data[key] = value;
      }
    }
    data.investment_goal = parseInt(data.investment_goal);
    console.log(data);
    create_idea(data).then((response)=>
      console.log('response from create idea: ', response)
    )
  }
  
  return (
    <div>
      <Dialog
        aria-labelledby="customized-dialog-title"
        open={true}
        fullWidth
        maxWidth="md"
      >
        <DialogTitle id="customized-dialog-title" style={{textAlign: 'center', fontWeight: 'bold'}}>
          CREATE NEW IDEA
        </DialogTitle>
        <DialogContent dividers>
          <ModalBody fullWidth>
            <div className="container-fluid">
              <div className="row">
                <div className="col">
                <form onSubmit={submitIdea}>
                  <div className="mb-3">
                    <label htmlFor="title" className="form-label">Title:</label>
                    <input type="text" name="title" className="form-control" id="title" aria-describedby="title" />
                  </div>
                  <div className="mb-3">
                    <label htmlFor="excerpt">Excerpt:</label>
                    <textarea name="excerpt" className="form-control" id="excerpt" rows="3"></textarea>
                  </div>
                  <div className="mb-3">
                    <label htmlFor="description">Description:</label>
                    <textarea name="description" className="form-control" id="description" rows="3"></textarea>
                  </div>
                  <div className="mb-3">
                    <label htmlFor="investment_goal" className="form-label">Investment goal:</label>
                    <input name="investment_goal" type="number" className="form-control" id="investment_goal" aria-describedby="investment_goal" />
                  </div>
                  <div className="mb-3">
                    <label htmlFor="team" className="form-label">Team:</label>
                    <input name="team" type="text" className="form-control" id="team" aria-describedby="team" />
                  </div>
                  <div className="mb-3">
                    <label htmlFor="tags" className="form-label">Tags (devide by comma):</label>
                    <input name="tags" type="text" className="form-control" id="tags" aria-describedby="tags" />
                  </div>
                  <div className="mb-3">
                    <label htmlFor="competitors" className="form-label">Competitors:</label>
                    <input name="competitors" type="text" className="form-control" id="competitors" aria-describedby="competitors" />
                  </div>
                  <div className="mb-3">
                    <label htmlFor="value_proposition" className="form-label">Value proposition:</label>
                    <input name="value_proposition" type="text" className="form-control" id="value_proposition" aria-describedby="value_proposition" />
                  </div>
                  <div className="mb-5">
                    <label className="form-label" htmlFor="picture_url">Idea image file path:</label>
                    <input name="picture_url" type="url" className="form-control" id="picture_url" />
                  </div>
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
