import { Entity, Column, Index, OneToMany } from 'typeorm';
import Model from './baseModel';
import { Comment } from './commentModel';

export enum RoleEnumType {
  USER = 'user',
  ADMIN = 'admin',
}

@Entity('users')
export class User extends Model {
  @OneToMany(() => Comment, (comment) => comment.user)
  comments: Comment[];

  @OneToMany(() => Comment, (comment) => comment.user)
  projects: Comment[];

  toJSON() {
    return {
      ...this,
      password: undefined,
      verified: undefined,
      verificationCode: undefined,
    };
  }
}