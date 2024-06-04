import { EntryStructure, UserTask } from "../models/tasks"

type RetrieveCacheResponse<T> = [T | null, string | null]

const useCache = () => {
    const timestamp = () => new Date().toISOString()

    const cacheUserTasks = (userId: number, tasks: Record<number, UserTask>) => {
        sessionStorage.setItem("tasks", JSON.stringify(tasks))
        sessionStorage.setItem("tasksOwner", userId.toString())
        sessionStorage.setItem("taskCacheTimestamp", timestamp())
    }

    const retrieveTasksOwner = (): number | null => {
        const owner = sessionStorage.getItem("tasksOwner")
        return owner ? parseInt(owner) : null
    }

    const retrieveUserTasks = (): RetrieveCacheResponse<Record<number, UserTask>> => {
        const cachedTasks = sessionStorage.getItem("tasks")

        return [
            cachedTasks ? JSON.parse(cachedTasks) : null,
            sessionStorage.getItem("taskCacheTimestamp")
        ]
    }

    const cacheRotationEntries = (rotationId: number, entries: EntryStructure) => {
        sessionStorage.setItem(`entries.${rotationId}`, JSON.stringify(entries))
        sessionStorage.setItem(`rotationEntriesCacheTimestamp.${rotationId}`, timestamp())
    }

    const retrieveRotationEntries = (rotationId: number): RetrieveCacheResponse<EntryStructure> => {
        const cachedEntries = sessionStorage.getItem(`entries.${rotationId}`)
        const parsedEntries = cachedEntries ? JSON.parse(cachedEntries) as EntryStructure : null

        return [
            parsedEntries,
            sessionStorage.getItem(`rotationEntriesCacheTimestamp.${rotationId}`)
        ]
    }

    const retrieveOrCacheUserTasks = (userId: number, tasks?: Record<number, UserTask>): Record<number, UserTask> => {
        if (tasks) {
            cacheUserTasks(userId, tasks)
            return tasks
        }

        if (userId !== retrieveTasksOwner()) {
            cacheUserTasks(userId, {})
            return {}
        }

        const [cachedTasks] = retrieveUserTasks()
        return cachedTasks || {}
    }

    return {
        cacheUserTasks,
        retrieveUserTasks,
        cacheRotationEntries,
        retrieveRotationEntries,
        retrieveOrCacheUserTasks
    }
}

export default useCache