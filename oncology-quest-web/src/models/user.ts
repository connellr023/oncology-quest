import { Rotation } from "./rotation"
import { UserTask } from "./tasks"

export interface User {
    id: number,
    username: string,
    name: string,
    isAdmin: boolean,
    loginCount: number
}

export interface Session {
    user: User,
    rotations: Record<number, Rotation>,
    tasks?: Record<number, UserTask>
}