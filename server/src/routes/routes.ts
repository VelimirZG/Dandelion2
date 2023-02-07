import express from 'express';
import { Response } from 'express';
import { createComment, getAllProjectComments } from '../services/commentService';
import { checkIfLikeExists, createLike, deleteLike, getAllUserLikes } from '../services/likeService';
import { createProject, findProjectById } from '../services/projectService';
import { createUser, findUserById } from '../services/userService';


const router = express.Router();
const path = require('path');
const uuid = require('uuid');

router.post('/register', async (req: any, res: any) => {
  const walletId = req.body.walletId;
  
  if (!walletId) return res.sendStatus(400);
  
  const user = {
    id: walletId
  }
  const reponse = await createUser(user);

  if(reponse) {
    res.sendStatus(200);
  }else {
    res.sendStatus(500);
  }
});

router.post('/upload', async (req: any, res: Response) => {
  // Get the file that was set to our field named "image"
  console.log(req.body);
  const { image } = req.files;
  const walletId: string = req.body.walletId;
  const projectId: string = req.body.projectId;

  // If no image submitted, exit
  if (!image) return res.sendStatus(300);
  console.log(path.extname(image.name));
  // If does not have image mime type prevent from uploading
  if (path.extname(image.name) != '.png' && path.extname(image.name) != '.jpg') return res.sendStatus(500);

  // Move the uploaded image to our upload folder
  const imagePath: string = __dirname + '/../../upload/' + projectId + path.extname(image.name);
  image.mv(imagePath);
  
  
  const user: any = await findUserById(walletId);

  if(!user) {
    const user = await createUser({
      id: walletId
    });
    if(!user) {
      res.sendStatus(500);
      return;
    }
  }
  const imageURL = '/upload/' + projectId + path.extname(image.name)
  console.log(imageURL);
  const project: any = {
    id: projectId,
    photo: imageURL,
    user: user.id
  }
  await createProject(project, user)

  res.send({status: 1, imageURL: imageURL});
  return;
});

router.post('/like', async (req: any, res: any) => {
  const walletId: string = req.body.walletId;
  const projectId: string = req.body.projectId;
  const id = uuid.v4();

  console.log('like', walletId, projectId);

  if(!walletId || !projectId) {
    res.sendStatus(500);
  }

  let user: any = await findUserById(walletId);

  if(!user) {
    const user = await createUser({
      id: walletId
    });
    if(!user) {
      res.sendStatus(500);
    }
  }

  const like = {
    id: id,
    projectId: projectId
  };
  const isLiked = await checkIfLikeExists(projectId, walletId);

  if(isLiked) {
    deleteLike(isLiked.id);
    res.send({status: 0, like: isLiked.id, likeDeleted: true});
    return;
  }

  const response = await createLike(like, walletId);

  if(!response) {
    res.sendStatus(500);
  }

  res.send({status: 1, like: response, likeDeleted: false});
  return;
});

router.post('/likes', async (req: any, res: any) => {
  const walletId: string = req.body.walletId;
  
  if(!walletId) {
    res.sendStatus(500);
  }
  const allLikes = await getAllUserLikes(walletId);

  res.send({status: 0, likes: [...allLikes]});

});

router.post('/comments', async (req: any, res: any) => {
  const projectId: string = req.body.projectId;
  
  if(!projectId) {
    res.send({status: 0, comments: null});
  }
  const allComments = await getAllProjectComments(projectId);

  res.send({status: 1, comments: [...allComments]});

});


router.post('/comment', async (req: any, res: any) => {
  const walletId: string = req.body.walletId;
  const projectId: string = req.body.projectId;
  const content: string = req.body.content;
  

  if(!walletId || !projectId) {
    res.sendStatus({status: 0, comment: null});
  }

  const user: any = await findUserById(walletId);

  if(!user) {
    const user = await createUser({
      id: walletId
    });
    if(!user) {
      res.sendStatus({status: 0, comment: null});
    }
  }

  const id = uuid.v4();
  const comment:any = {
    id: id,
    projectId: projectId,
    content: content,
    user: walletId
  };

  const response = await createComment(comment);

  if(!response) {
    res.sendStatus(500);
    return;
  }

  res.send({status: 1, comment: response});
  return;
});


export default router;