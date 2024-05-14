export type UserTaskEntries = Record<number, Record<number, Record<number, UserTask>>>
// export type TaskProgress = Record<number, {
//     progress: number,
//     subtasks: Record<number, number>
// }>

export interface SubTask {
    title: string,
    tasks: string[]
}

export interface Task {
    title: string,
    tasks: SubTask[]
}

export interface Tasks {
    entries: Task[]
}

export interface UserTask {
    completed: boolean,
    comment: string
}