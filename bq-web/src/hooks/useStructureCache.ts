import { EntryStructureResponse, Task } from "../models/task"

const useStructureCache = () => {
    const cache = (structure: EntryStructureResponse) => {
        sessionStorage.setItem("structure", JSON.stringify(structure))
    }

    const updateCache = (entries: Task[]) => {
        const cachedStructure = sessionStorage.getItem("structure")
        
        if (cachedStructure) {
            let cachedEntries: EntryStructureResponse = JSON.parse(cachedStructure)

            cachedEntries.entries = entries
            cachedEntries.lastUpdated = new Date().toISOString()

            cache(cachedEntries)
        }
    }

    const retrieve = (): EntryStructureResponse | null => {
        const cachedStructure = sessionStorage.getItem("structure")
        return cachedStructure ? JSON.parse(cachedStructure) : null
    }

    return {
        cache,
        updateCache,
        retrieve
    }
}

export default useStructureCache