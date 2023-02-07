import { Column, Entity, JoinColumn, ManyToOne } from 'typeorm';
import Model from './baseModel';
import { Project } from './projectModel';
import { User } from './userModel';

@Entity('comments')
export class Comment extends Model {
  @Column()
  content: string;

  @Column()
  projectId: string;

  @Column()
  user: string;
}