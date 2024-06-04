export interface UserTask {
    id: number,
    userId: number,
    subtaskId: number,
    isCompleted: boolean,
    comment: string
}

export interface Supertask {
    id: number,
    title: string,
    rotationId: number
}

export interface Task {
    id: number,
    supertaskId: number,
    title: string,
    rotationId: number
}

export interface Subtask {
    id: number,
    taskId: number,
    title: string,
    rotationId: number
}

type PossibleEntry = Supertask | Task | Subtask
export interface EntryLevel<T = PossibleEntry, U = any> {
    entry: T,
    children: U[]
}

type EntryHierarchy = EntryLevel<Supertask, EntryLevel<Task, Subtask>>
export type EntryStructure = EntryHierarchy[]