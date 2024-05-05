import { Task, UserTaskEntries } from "./task";

export interface User {
    username: string,
    name: string,
    email: string,
    isAdmin: boolean,
    tasks: UserTaskEntries
}

export interface UserSession {
    user: User,
    entries: Task[]
}