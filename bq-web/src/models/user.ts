import { Domain } from "./domain"
import { UserTask } from "./task"

export interface User {
    id: number,
    username: string,
    name: string,
    email: string,
    isAdmin: boolean,
    loginCount: number
}

export interface Session {
    user: User,
    domains: Domain[],
    tasks?: UserTask[]
}

export interface SelectedUser {
    user: User,
    tasks: UserTask[]
}