import React from "react";
import styled from "styled-components";
import { withStyles } from "@material-ui/core/styles";
import Dialog from "@material-ui/core/Dialog";
import MuiDialogTitle from "@material-ui/core/DialogTitle";
import MuiDialogContent from "@material-ui/core/DialogContent";
import Typography from "@material-ui/core/Typography";
import Button from "@material-ui/core/Button";
import MuiDialogActions from "@material-ui/core/DialogActions";

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

function Popup(props) {
  
  return (
    <div>
      <Dialog
        aria-labelledby="customized-dialog-title"
        open={true}
      >
        <DialogTitle id="customized-dialog-title">
          Information:
        </DialogTitle>
        <DialogContent dividers>
          <ModalBody>
            <>
              <Typography>{props.msg}</Typography>
            </>
          </ModalBody>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => props.setPopupInfo({open: false, msg: ''})} color="primary">
            OK
          </Button>
        </DialogActions>
      </Dialog>
    </div>
  );
}

export default Popup;
