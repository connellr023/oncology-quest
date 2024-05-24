import { EntryStructure, UserTask } from "../models/task"

type RetrieveCacheResponse<T> = [T | null, string]

const useCache = () => {
    const timestamp = () => new Date().toISOString()

    const cacheUserTasks = (tasks: UserTask[]) => {
        sessionStorage.setItem("tasks", JSON.stringify(tasks))
        sessionStorage.setItem("taskCacheTimestamp", timestamp())
    }

    const retrieveUserTasks = (): RetrieveCacheResponse<UserTask[]> => {
        const cachedTasks = sessionStorage.getItem("tasks")
        return [
            cachedTasks ? JSON.parse(cachedTasks) : null,
            sessionStorage.getItem("taskCacheTimestamp") || timestamp()
        ]
    }

    const cacheDomainEntries = (entries: EntryStructure) => {
        sessionStorage.setItem("entries", JSON.stringify(entries))
        sessionStorage.setItem("domainEntriesCacheTimestamp", timestamp())
    }

    const retrieveDomainEntries = (): RetrieveCacheResponse<EntryStructure> => {
        const cachedEntries = sessionStorage.getItem("entries")
        return [
            cachedEntries ? JSON.parse(cachedEntries) : null,
            sessionStorage.getItem("domainEntriesCacheTimestamp") || timestamp()
        ]
    }

    return {
        cacheUserTasks,
        retrieveUserTasks,
        cacheDomainEntries,
        retrieveDomainEntries
    }
}

export default useCache