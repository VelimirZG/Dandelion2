import { Request } from 'express';
import { Like } from '../models/likeModel';
import { User } from '../models/userModel';
import { dbConnection } from '../config/database';

const likeRepository = dbConnection.getRepository(Like);

export const createLike = async (input: Partial<Like>, user: User) => {
  return await likeRepository.save(likeRepository.create({ ...input, user }));
};

export const getLike = async (likeId: string) => {
  return await likeRepository.findOneBy({ id: likeId });
};

export const findLikes = async (req: Request) => {
  const builder = likeRepository.createQueryBuilder('like');

  if (req.query.search) {
    builder.where('like.content LIKE :search', {
      search: `%${req.query.search}%`,
    });
  }

  if (req.query.sort) {
    const sortQuery = req.query.sort === '-price' ? 'DESC' : 'ASC';
    builder.orderBy('like.id', sortQuery);
  }

  return await builder.getMany();
};