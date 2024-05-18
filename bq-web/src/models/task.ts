export type UserTaskEntries = Record<number, Record<number, Record<number, UserTask>>>
export type EntryIndex = [number] | [number, number] | [number, number, number]

/**
 * Represents a response from the server containing the structure
 * of a user's tasks and the timestamp of the last update.
 */
export interface EntryStructureResponse {
    lastUpdated: string,
    entries: Task[]
}

/**
 * Represents a subtask.
 */
export interface SubTask {
    title: string,
    tasks: string[]
}

/**
 * Represents a task.
 */
export interface Task {
    title: string,
    tasks: SubTask[]
}

/**
 * Represents a collection of tasks.
 * Contains the entry structure.
 */
export interface Tasks {
    entries: Task[]
}

/**
 * Represents a single user task.
 */
export interface UserTask {
    completed: boolean,
    comment: string
}