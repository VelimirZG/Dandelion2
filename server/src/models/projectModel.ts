import { Column, Entity, JoinColumn, ManyToOne, OneToMany } from 'typeorm';
import Model from './baseModel';
import { Comment } from './commentModel';
import { User } from './userModel';

@Entity('projects')
export class Project extends Model {
  @Column({
    default: 'default.png',
  })
  photo: string;

  @OneToMany(() => Comment, (comment) => comment.projectId)
  comments: Comment[];

  @ManyToOne(() => User, (user) => user.projects)
  @JoinColumn()
  user!: User;
}