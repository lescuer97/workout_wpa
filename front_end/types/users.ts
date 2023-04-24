declare global {
  type UserRole = typeof UserRole[keyof typeof UserRole];
  type UserRegistration = {
    email: string;
    password: string;
    password_repeat: string;
  };
  interface State {
    goodLogin?: boolean;
  }
  type MiddlewareResponse = Response & {
    goodLogin: boolean;
  };

  type UserLogin = {
    email: string;
    password: string;
  };
}

export const UserRole = {
  EditSelf: "EditSelf",
  EditOther: "EditOther",
  RemoveOther: "RemoveOther",
  WatchOther: "WatchOther",
  SuperAdmin: "SuperAdmin",
} as const;
