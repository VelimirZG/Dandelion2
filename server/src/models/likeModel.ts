import { Column, Entity, JoinColumn, ManyToOne } from 'typeorm';
import Model from './baseModel';
import { User } from './userModel';

@Entity('likes')
export class Like extends Model {
  @Column()
  projectId: string;

  @ManyToOne(() => User, (user) => user.comments)
  @JoinColumn()
  user!: User;
}