import { Ref, inject } from "vue"
import { EntryStructure } from "../models/tasks"
import { API_ENDPOINT } from "../utilities"
import useCache from "./useCache"

interface CreateEntryResponse {
    entryId: number
}

const useEntries = () => {
    const { cacheRotationEntries, retrieveRotationEntries } = useCache()
    const entries = inject<Ref<Record<number, EntryStructure>>>("entries")!

    const createSupertask = async (title: string, rotationId: number): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/entries/supertasks/create`, {
            credentials: "include",
            headers: {
                "Content-Type": "application/json"
            },
            method: "POST", 
            body: JSON.stringify({
                title,
                rotationId
            })
        })

        if (response.ok) {
            const data: CreateEntryResponse = await response.json()

            entries.value[rotationId].push({
                entry: {
                    id: data.entryId,
                    title,
                    rotationId
                },
                children: []
            })

            cacheRotationEntries(rotationId, entries.value[rotationId])
            return true
        }

        return false
    }

    const updateSupertask = async (rotationId: number, supertaskIndex: number, supertaskId: number, title: string): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/entries/supertasks/update`, {
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
            entries.value[rotationId][supertaskIndex].entry.title = title
            cacheRotationEntries(rotationId, entries.value[rotationId])
            return true
        }

        return false
    }

    const deleteSupertask = async (rotationId: number, supertaskIndex: number, supertaskId: number): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/entries/supertasks/delete`, {
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
            entries.value[rotationId].splice(supertaskIndex, 1);
            cacheRotationEntries(rotationId, entries.value[rotationId])
            return true
        }

        return false
    }

    const createTask = async (title: string, rotationId: number, supertaskId: number, supertaskIndex: number): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/entries/tasks/create`, {
            credentials: "include",
            headers: {
                "Content-Type": "application/json"
            },
            method: "POST",
            body: JSON.stringify({
                title,
                rotationId,
                parentId: supertaskId
            })
        })

        if (response.ok) {
            const data: CreateEntryResponse = await response.json()

            entries.value[rotationId][supertaskIndex].children.push({
                entry: {
                    id: data.entryId,
                    title,
                    rotationId,
                    supertaskId
                },
                children: []
            })

            cacheRotationEntries(rotationId, entries.value[rotationId])
            return true
        }

        return false
    }

    const updateTask = async (rotationId: number, supertaskIndex: number, taskIndex: number, taskId: number, title: string): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/entries/tasks/update`, {
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
            entries.value[rotationId][supertaskIndex].children[taskIndex].entry.title = title
            cacheRotationEntries(rotationId, entries.value[rotationId])
            return true
        }

        return false
    }

    const deleteTask = async (rotationId: number, supertaskIndex: number, taskIndex: number, taskId: number): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/entries/tasks/delete`, {
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
            entries.value[rotationId][supertaskIndex].children.splice(taskIndex, 1);
            cacheRotationEntries(rotationId, entries.value[rotationId])
            return true
        }

        return false
    }

    const createSubtask = async (title: string, rotationId: number, taskId: number, supertaskIndex: number, taskIndex: number): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/entries/subtasks/create`, {
            credentials: "include",
            headers: {
                "Content-Type": "application/json"
            },
            method: "POST",
            body: JSON.stringify({
                title,
                rotationId,
                parentId: taskId
            })
        })

        if (response.ok) {
            const data: CreateEntryResponse = await response.json()

            entries.value[rotationId][supertaskIndex].children[taskIndex].children.push({
                id: data.entryId,
                title,
                rotationId,
                taskId
            })

            cacheRotationEntries(rotationId, entries.value[rotationId])
            return true
        }

        return false
    }

    const updateSubtask = async (rotationId: number, supertaskIndex: number, taskIndex: number, subtaskIndex: number, subtaskId: number, title: string): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/entries/subtasks/update`, {
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
            entries.value[rotationId][supertaskIndex].children[taskIndex].children[subtaskIndex].title = title
            cacheRotationEntries(rotationId, entries.value[rotationId])
            return true
        }

        return false
    }

    const deleteSubtask = async (rotationId: number, supertaskIndex: number, taskIndex: number, subtaskIndex: number, subtaskId: number): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/entries/subtasks/delete`, {
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
            entries.value[rotationId][supertaskIndex].children[taskIndex].children.splice(subtaskIndex, 1);
            cacheRotationEntries(rotationId, entries.value[rotationId])
            return true
        }

        return false
    }

    const fetchEntriesWithCaching = async (rotationId: number): Promise<boolean> => {
        const [cachedEntries, cacheTimestamp] = retrieveRotationEntries(rotationId)
        const url = new URL(`${API_ENDPOINT}/api/entries/${rotationId}`)

        if (cacheTimestamp) {
            url.searchParams.append("entriesCacheTimestamp", cacheTimestamp)
        }

        const response = await fetch(url, {
            credentials: "include",
            headers: {
                "Content-Type": "application/json"
            }
        })

        if (response.status === 304) {
            entries.value[rotationId] = cachedEntries || []
            return true
        }

        if (response.ok) {
            try {
                const data: EntryStructure = await response.json()

                cacheRotationEntries(rotationId, data)
                entries.value[rotationId] = data
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