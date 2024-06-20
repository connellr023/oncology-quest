import { EntryStructure, UserTaskStructure } from "../models/tasks"

type RetrieveCacheResponse<T> = [T | null, string | null]

const useCache = () => {
    const timestamp = () => new Date().toISOString()

    const cacheUserTasks = (userId: number, rotationId: number, tasks: UserTaskStructure) => {
        sessionStorage.setItem(`taskCacheTimestamp.${userId}`, timestamp())
        sessionStorage.setItem(`tasks.${rotationId}.${userId}`, JSON.stringify(tasks))
    }

    const retrieveUserTasks = (userId: number, rotationId: number): RetrieveCacheResponse<UserTaskStructure> => {
        const cachedTasks = sessionStorage.getItem(`tasks.${rotationId}.${userId}`)
        const parsedTasks = cachedTasks ? JSON.parse(cachedTasks) as UserTaskStructure : null

        return [
            parsedTasks,
            sessionStorage.getItem(`taskCacheTimestamp.${userId}`)
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

    return {
        cacheUserTasks,
        retrieveUserTasks,
        cacheRotationEntries,
        retrieveRotationEntries
    }
}

export default useCache