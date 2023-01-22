import { Request } from 'express';
import { Comment } from '../models/commentModel';
import { User } from '../models/userModel';
import { dbConnection } from '../config/database';

const commentRepository = dbConnection.getRepository(Comment);

export const createComment = async (input: Partial<Comment>, user: User) => {
  return await commentRepository.save(commentRepository.create({ ...input, user }));
};

export const getComment = async (commentId: string) => {
  return await commentRepository.findOneBy({ id: commentId });
};

export const findComments = async (req: Request) => {
  const builder = commentRepository.createQueryBuilder('comment');

  if (req.query.search) {
    builder.where('comment.content LIKE :search', {
      search: `%${req.query.search}%`,
    });
  }

  if (req.query.sort) {
    const sortQuery = req.query.sort === '-price' ? 'DESC' : 'ASC';
    builder.orderBy('comment.id', sortQuery);
  }

  return await builder.getMany();
};