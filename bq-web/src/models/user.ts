import { EntryStructureResponse, UserTaskEntries } from "./task"

/**
 * Represents a user.
 */
export interface User {
    username: string,
    name: string,
    email: string,
    isAdmin: boolean,
    tasks: UserTaskEntries
}

/**
 * Represents a response from the server containing a user and the structure of their tasks.
 */
export interface UserSessionResponse {
    user: User,
    structure: EntryStructureResponse
}