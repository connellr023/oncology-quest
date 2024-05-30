import { Domain } from "./domain"
import { UserTask } from "./tasks"

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
    domains: Record<number, Domain>,
    tasks?: Record<number, UserTask>
}

export interface UserWithTasks {
    user: User,
    tasks: Record<number, UserTask>
}