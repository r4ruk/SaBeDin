// This file contains basic type definitions for data contracts which are sent to backend.

export type user = {
  id: string,
  name: string,
  password: string,
  email: string,
  age: number
};

export type LoginUserData = {
  email: string,
  password: string,
}