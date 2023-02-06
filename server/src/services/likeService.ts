import { Request } from 'express';
import { Like } from '../models/likeModel';
import { dbConnection } from '../config/database';

const likeRepository = dbConnection.getRepository(Like);

export const createLike = async (input: Partial<Like>, walletId: string) => {
  return await likeRepository.save(likeRepository.create({ ...input, user: walletId }));
};

export const getLike = async (likeId: string) => {
  return await likeRepository.findOneBy({ id: likeId });
};

export const checkIfLikeExists = async (projectId: string, walletId: string) => {
  return await likeRepository.findOneBy({ projectId: projectId, user: walletId });
};

export const getAllUserLikes = async (walletId: string) => {
  return await likeRepository.findBy({ user: walletId });
};

export const deleteLike = async (likeId: string) => {
  return await likeRepository.delete({ id: likeId });
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