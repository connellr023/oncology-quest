import { Ref, inject } from "vue"
import { EntryStructure } from "../models/tasks"
import { API_ENDPOINT } from "../utilities"
import useCache from "./useCache"

interface CreateEntryResponse {
    entryId: number
}

const useEntries = () => {
    const { cacheDomainEntries, retrieveDomainEntries } = useCache()
    const entries = inject<Ref<Record<number, EntryStructure>>>("entries")!

    const createSupertask = async (title: string, domainId: number): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/supertasks/create`, {
            credentials: "include",
            headers: {
                "Content-Type": "application/json"
            },
            method: "POST", 
            body: JSON.stringify({
                title,
                domainId
            })
        })

        if (response.ok) {
            const data: CreateEntryResponse = await response.json()

            entries.value[domainId].push({
                entry: {
                    id: data.entryId,
                    title,
                    domainId
                },
                children: []
            })

            return true
        }

        return false
    }

    const updateSupertask = async (domainId: number, entryIndex: number, entryId: number, title: string): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/supertasks/update`, {
            credentials: "include",
            headers: {
                "Content-Type": "application/json"
            },
            method: "PATCH",
            body: JSON.stringify({
                entryId,
                title
            })
        })

        if (response.ok) {
            entries.value[domainId][entryIndex].entry.title = title
        }

        return false
    }

    const deleteSupertask = async (domainId: number, entryIndex: number, entryId: number): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/supertasks/delete`, {
            credentials: "include",
            headers: {
                "Content-Type": "application/json"
            },
            method: "DELETE",
            body: JSON.stringify({
                entryId
            })
        })

        if (response.ok) {
            delete entries.value[domainId][entryIndex]
            return true
        }

        return false
    }

    const fetchEntriesWithCaching = async (domainId: number): Promise<boolean> => {
        const [cachedEntries, cacheTimestamp] = retrieveDomainEntries(domainId)
        const endpoint = cacheTimestamp ? `${domainId}/${cacheTimestamp}` : `${domainId}/`

        const response = await fetch(`${API_ENDPOINT}/api/domains/${endpoint}`, {
            credentials: "include",
            headers: {
                "Content-Type": "application/json"
            }
        })

        if (response.status === 304) {
            entries.value[domainId] = cachedEntries || []
            return true
        }

        if (response.ok) {
            try {
                const data: EntryStructure = await response.json()

                cacheDomainEntries(domainId, data)
                entries.value[domainId] = data
                return true
            }
            catch (_) {
                return false
            }
        }

        return false
    }

    return {
        createSupertask,
        updateSupertask,
        deleteSupertask,
        fetchEntriesWithCaching
    }
}

export default useEntries