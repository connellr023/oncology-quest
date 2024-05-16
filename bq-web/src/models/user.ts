import { EntryStructureResponse, UserTaskEntries } from "./task";

export interface User {
    username: string,
    name: string,
    email: string,
    isAdmin: boolean,
    tasks: UserTaskEntries
}

export interface UserSessionResponse {
    user: User,
    structure: EntryStructureResponse
}