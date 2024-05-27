import { EntryStructure } from "../models/task"
import { API_ENDPOINT } from "../utilities"
import useCache from "./useCache"

const useEntries = () => {
    const { cacheDomainEntries, retrieveDomainEntries } = useCache()

    const fetchEntriesWithCaching = async (domainId: number): Promise<EntryStructure | null> => {
        const [cachedEntries, cacheTimestamp] = retrieveDomainEntries(domainId)
        const endpoint = cacheTimestamp ? `/${domainId}/${cacheTimestamp}` : `/${domainId}`

        const response = await fetch(`${API_ENDPOINT}/api/domains/${endpoint}`, {
            credentials: "include",
            headers: {
                "Content-Type": "application/json"
            }
        })

        if (response.status === 304) {
            return cachedEntries
        }

        if (response.ok) {
            try {
                const data: EntryStructure = await response.json()

                cacheDomainEntries(domainId, data)
                return data
            }
            catch (_) {
                return null
            }
        }

        return null
    }

    return {
        fetchEntriesWithCaching
    }
}

export default useEntries