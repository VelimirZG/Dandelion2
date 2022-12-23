import React from 'react';
import ReactDOM from 'react-dom/client';
import Main from './main';
import 'bootstrap/dist/css/bootstrap.css';
import { initContract } from './assets/near/utils';
import * as buffer from 'buffer';
window.Buffer = buffer.Buffer; 

const root = ReactDOM.createRoot(document.getElementById('root'));
window.nearInitPromise = initContract()
  .then(() => {
    root.render(<Main />);
  })
  .catch(console.error)

