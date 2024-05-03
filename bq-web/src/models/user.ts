import { UserTaskEntries, NamedTaskEntries } from "./task";

export interface User {
    username: string,
    name: string,
    email: string,
    tasks: UserTaskEntries
}

export interface UserSession {
    user: User,
    entries: NamedTaskEntries
}