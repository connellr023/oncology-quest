import { EntryStructure } from "../models/tasks"

type RetrieveCacheResponse<T> = [T | null, string | null]

const useCache = () => {
    const timestamp = () => new Date().toISOString()

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
        cacheRotationEntries,
        retrieveRotationEntries
    }
}

export default useCache