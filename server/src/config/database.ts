import { DataSource } from "typeorm";
import { Comment } from '../models/commentModel';
import { Like } from '../models/likeModel';
import { Project } from '../models/projectModel';
import { User } from '../models/userModel';

export const dbConnection = new DataSource({
  "type": "postgres",
  "host": "127.0.0.1",
  "port": 5435,
  "username": "postgres",
  "password": "dandelion#654",
  "database": "dandelion",
  "synchronize": true,
  "entities": [
    Comment,
    User,
    Project,
    Like
  ]
});