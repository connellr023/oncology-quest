import { Rotation } from "./rotation"

export interface User {
    id: number,
    username: string,
    name: string,
    isAdmin: boolean,
    loginCount: number
}

export interface Session {
    user: User,
    rotations: Record<number, Rotation>
}