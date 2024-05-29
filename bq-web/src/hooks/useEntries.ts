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

            cacheDomainEntries(domainId, entries.value[domainId])
            return true
        }

        return false
    }

    const updateSupertask = async (domainId: number, supertaskIndex: number, supertaskId: number, title: string): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/supertasks/update`, {
            credentials: "include",
            headers: {
                "Content-Type": "application/json"
            },
            method: "PATCH",
            body: JSON.stringify({
                entryId: supertaskId,
                title
            })
        })

        if (response.ok) {
            entries.value[domainId][supertaskIndex].entry.title = title
            cacheDomainEntries(domainId, entries.value[domainId])
            return true
        }

        return false
    }

    const deleteSupertask = async (domainId: number, supertaskIndex: number, supertaskId: number): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/supertasks/delete`, {
            credentials: "include",
            headers: {
                "Content-Type": "application/json"
            },
            method: "DELETE",
            body: JSON.stringify({
                entryId: supertaskId
            })
        })

        if (response.ok) {
            entries.value[domainId].splice(supertaskIndex, 1);
            cacheDomainEntries(domainId, entries.value[domainId])
            return true
        }

        return false
    }

    const createTask = async (title: string, domainId: number, supertaskId: number, supertaskIndex: number): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/tasks/create`, {
            credentials: "include",
            headers: {
                "Content-Type": "application/json"
            },
            method: "POST",
            body: JSON.stringify({
                title,
                domainId,
                parentId: supertaskId
            })
        })

        if (response.ok) {
            const data: CreateEntryResponse = await response.json()

            entries.value[domainId][supertaskIndex].children.push({
                entry: {
                    id: data.entryId,
                    title,
                    domainId,
                    supertaskId
                },
                children: []
            })

            cacheDomainEntries(domainId, entries.value[domainId])
            return true
        }

        return false
    }

    const updateTask = async (domainId: number, supertaskIndex: number, taskIndex: number, taskId: number, title: string): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/tasks/update`, {
            credentials: "include",
            headers: {
                "Content-Type": "application/json"
            },
            method: "PATCH",
            body: JSON.stringify({
                entryId: taskId,
                title
            })
        })

        if (response.ok) {
            entries.value[domainId][supertaskIndex].children[taskIndex].entry.title = title
            cacheDomainEntries(domainId, entries.value[domainId])
            return true
        }

        return false
    }

    const deleteTask = async (domainId: number, supertaskIndex: number, taskIndex: number, taskId: number): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/tasks/delete`, {
            credentials: "include",
            headers: {
                "Content-Type": "application/json"
            },
            method: "DELETE",
            body: JSON.stringify({
                entryId: taskId
            })
        })

        if (response.ok) {
            entries.value[domainId][supertaskIndex].children.splice(taskIndex, 1);
            cacheDomainEntries(domainId, entries.value[domainId])
            return true
        }

        return false
    }

    const createSubtask = async (title: string, domainId: number, taskId: number, supertaskIndex: number, taskIndex: number): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/subtasks/create`, {
            credentials: "include",
            headers: {
                "Content-Type": "application/json"
            },
            method: "POST",
            body: JSON.stringify({
                title,
                domainId,
                parentId: taskId
            })
        })

        if (response.ok) {
            const data: CreateEntryResponse = await response.json()

            entries.value[domainId][supertaskIndex].children[taskIndex].children.push({
                id: data.entryId,
                title,
                domainId,
                taskId
            })

            cacheDomainEntries(domainId, entries.value[domainId])
            return true
        }

        return false
    }

    const updateSubtask = async (domainId: number, supertaskIndex: number, taskIndex: number, subtaskIndex: number, subtaskId: number, title: string): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/subtasks/update`, {
            credentials: "include",
            headers: {
                "Content-Type": "application/json"
            },
            method: "PATCH",
            body: JSON.stringify({
                entryId: subtaskId,
                title
            })
        })

        if (response.ok) {
            entries.value[domainId][supertaskIndex].children[taskIndex].children[subtaskIndex].title = title
            cacheDomainEntries(domainId, entries.value[domainId])
            return true
        }

        return false
    }

    const deleteSubtask = async (domainId: number, supertaskIndex: number, taskIndex: number, subtaskIndex: number, subtaskId: number): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/subtasks/delete`, {
            credentials: "include",
            headers: {
                "Content-Type": "application/json"
            },
            method: "DELETE",
            body: JSON.stringify({
                entryId: subtaskId
            })
        })

        if (response.ok) {
            entries.value[domainId][supertaskIndex].children[taskIndex].children.splice(subtaskIndex, 1);
            cacheDomainEntries(domainId, entries.value[domainId])
            return true
        }

        return false
    }

    const fetchEntriesWithCaching = async (domainId: number): Promise<boolean> => {
        const [cachedEntries, cacheTimestamp] = retrieveDomainEntries(domainId)
        const query = cacheTimestamp ? `?entriesCacheTimestamp=${cacheTimestamp}` : ""

        const response = await fetch(`${API_ENDPOINT}/api/domains/${domainId}${query}`, {
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
        createTask,
        updateTask,
        deleteTask,
        createSubtask,
        updateSubtask,
        deleteSubtask,
        fetchEntriesWithCaching
    }
}

export default useEntries