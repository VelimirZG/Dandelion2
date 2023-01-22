import { Request } from 'express';
import { Project } from '../models/projectModel';
import { User } from '../models/userModel';
import { dbConnection } from '../config/database';

const projectRepository = dbConnection.getRepository(Project);

export const createProject = async (input: Partial<Project>, user: User) => {
  return await projectRepository.save(projectRepository.create({ ...input, user }));
};

export const getProject = async (projectId: string) => {
  return await projectRepository.findOneBy({ id: projectId });
};

export const findProjectById = async (projectId: string) => {
  return await projectRepository.findOneBy({ id: projectId });
};