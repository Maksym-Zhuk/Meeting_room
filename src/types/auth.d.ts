import { User } from "../../types/models/User";

export type AuthResponse = {
  token: string;
  user: User;
};
