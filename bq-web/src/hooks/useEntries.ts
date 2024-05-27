import { Ref, inject } from "vue"
import { EntryStructure } from "../models/tasks"
import { API_ENDPOINT } from "../utilities"
import useCache from "./useCache"

const useEntries = () => {
    const { cacheDomainEntries, retrieveDomainEntries } = useCache()
    const entries = inject<Ref<Record<number, EntryStructure>>>("entries")!

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
        fetchEntriesWithCaching
    }
}

export default useEntries