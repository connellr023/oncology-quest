import { EntryStructure, UserTask } from "../models/task"

type RetrieveCacheResponse<T> = [T | null, string | null]

const useCache = () => {
    const timestamp = () => new Date().toISOString()

    const cacheUserTasks = (tasks: Record<number, UserTask>) => {
        sessionStorage.setItem("tasks", JSON.stringify(tasks))
        sessionStorage.setItem("taskCacheTimestamp", timestamp())
    }

    const retrieveUserTasks = (): RetrieveCacheResponse<Record<number, UserTask>> => {
        const cachedTasks = sessionStorage.getItem("tasks")
        return [
            cachedTasks ? JSON.parse(cachedTasks) : null,
            sessionStorage.getItem("taskCacheTimestamp")
        ]
    }

    const cacheDomainEntries = (domainId: number, entries: EntryStructure) => {
        sessionStorage.setItem(`entries.${domainId}`, JSON.stringify(entries))
        sessionStorage.setItem(`domainEntriesCacheTimestamp.${domainId}`, timestamp())
    }

    const retrieveDomainEntries = (domainId: number): RetrieveCacheResponse<EntryStructure> => {
        const cachedEntries = sessionStorage.getItem(`entries.${domainId}`)
        const parsedEntries = cachedEntries ? JSON.parse(cachedEntries) as EntryStructure : null

        return [
            parsedEntries,
            sessionStorage.getItem(`domainEntriesCacheTimestamp.${domainId}`)
        ]
    }

    const retrieveOrCacheUserTasks = (tasks?: Record<number, UserTask>): Record<number, UserTask> => {
        if (tasks) {
            cacheUserTasks(tasks)
            return tasks
        }

        const [cachedTasks] = retrieveUserTasks()
        return cachedTasks || {}
    }

    return {
        cacheUserTasks,
        retrieveUserTasks,
        cacheDomainEntries,
        retrieveDomainEntries,
        retrieveOrCacheUserTasks
    }
}

export default useCache