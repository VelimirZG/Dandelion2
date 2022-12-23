import React from "react";
import {withRouter} from "react-router-dom";
import styled from "styled-components";
import FavoriteIcon from '@mui/icons-material/Favorite';

const Card = styled.div`
  display: flex;

  .img-wrap {
    width: 30%;

    img {
      width: 100%;
    }
  }
  .content-wrap {
    width: 40%;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    margin-left: 30px;

    .title {
      margin-bottom: 20px;

      h3 { 
        font-size: 50px;
        margin: 0;
      }
    }

    .description {
      margin-bottom: 20px;
      font-size: 20px;

      p {
        margin: 0;
      }
    }
    
    .tags {
      display: flex;

      .button {
        font-family: PTSerif;
        padding: 15px;
        font-size: 20px;
        max-width: 300px;
      }

      ul {
        display: flex;
        list-style: none;
        overflow-x: auto;
        padding: 0;
        white-space: nowrap;
        margin: 0;
        font-family: PTSerif;
        font-size: 20px;
        margin-left: 20px;

        li {
          border: 1px solid black;
          margin-right: 20px;
          display: flex;
          align-items: center;
          justify-content: center;
          font-weight: bold;
          font-family: PTSerif;
          padding: 0 10px;
          text-transform: uppercase;
        }
      }
      .favorite {
        display: flex;
        justify-content: center;
        align-items: center;
        margin-left: 20px;


        svg {
          width: 50px;
          height: 50px;
        }
        
      }
    }
  }
`;

const Archive = () => {

  return (
    <Card>
      <div className="img-wrap">
        <img src="/logo512.png" />
      </div>
      <div className="content-wrap">
        <div className="title">
          <h3>Idea title</h3>
        </div>
        <div className="description">
          <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.</p>
        </div>
        <div className="tags">
          <button className="button">
            TAGS
          </button>
          <ul>
            <li>Tag 1</li>
            <li>Lorem Ipsum used</li>
            <li>etur adipiscing elit, sed do </li>
            <li>ipsum dolor</li>
          </ul>
          <div className="favorite">
            <FavoriteIcon />
          </div>
        </div>
      </div>
      <div className="action-wrap">

      </div>
    </Card>
  );

};

export default withRouter(Archive);
