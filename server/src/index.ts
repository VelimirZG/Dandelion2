
import express from 'express';
import router from './routes/routes';
import { dbConnection } from './config/database';

// initialize servers
const app = express();
const bodyParser = require('body-parser');
const cors = require('cors');
const fileUpload = require('express-fileupload');


let http: {
  listen: (arg0: number, arg1: any) => any;
} = require('http').createServer(app);

app.set('port', process.env.PORT || 3000);
app.use(cors());
app.use(bodyParser.urlencoded({extended: false}));
app.use(bodyParser.json());
app.use(function (req, res, next) {
    // res.setHeader('Access-Control-Allow-Origin', '*');
    // res.setHeader(
    //     'Access-Control-Allow-Headers',
    //     'Origin, X-Requested-With, Content-Type, Accept'
    // );
    // res.setHeader(
    //     'Access-Control-Allow-Methods',
    //     'GET, POST, OPTIONS, PUT, PATCH, DELETE'
    // );
    next();
});
app.use(
  fileUpload({
      limits: {
          fileSize: 10000000, // Around 10MB
      },
      abortOnLimit: true,
      // createParentPath: true
  })
);

app.use('/api/', router);

//pre-flight requests
app.options('*', function (req, res) {
    res.send(200);
});
app.use(express.static('public'));

async function init() {
  dbConnection.initialize().then(() => {
    console.log("Data Source has been initialized!")
  })
  .catch((err) => {
    console.error("Error during Data Source initialization", err)
  })
}

init().then(() =>
  http.listen(9999, (err: any) => {
      if (err) {
          console.log(err);
      }
      /* eslint-disable no-console */
      console.log('Server started');
  })
);
