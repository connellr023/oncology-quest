export type TaskEntries<U extends (string | number | symbol), V> = Record<U, Record<U, V>>

export type NamedTaskEntries = TaskEntries<string, string[]>
export type UserTaskEntries = TaskEntries<number, Record<number, UserTask>>

export interface Tasks {
    entries: NamedTaskEntries
}

export interface UserTask {
    completed: boolean,
    comment: string
}