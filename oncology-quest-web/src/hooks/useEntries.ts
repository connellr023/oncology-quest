import { Ref, inject } from "vue"
import { EntryStructure } from "../models/tasks"
import { API_ENDPOINT } from "../utilities"

import useCache from "./useCache"
import useJwt from "./useJwt"

interface CreateEntryResponse {
    entryId: number
}

const useEntries = () => {
    const { cacheRotationEntries, retrieveRotationEntries } = useCache()
    const { defaultHeaders } = useJwt()

    const entries = inject<Ref<Record<number, EntryStructure>>>("entries")!

    const entriesMemo = new Map<number, EntryStructure>()
    const cacheAndMemoEntries = (rotationId: number, entries: EntryStructure) => {
        cacheRotationEntries(rotationId, entries)
        entriesMemo.set(rotationId, entries)
    }

    const createSupertask = async (title: string, rotationId: number): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/entries/supertasks/create`, {
            credentials: "include",
            headers: defaultHeaders(),
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

            cacheAndMemoEntries(rotationId, entries.value[rotationId])
            return true
        }

        return false
    }

    const updateSupertask = async (rotationId: number, supertaskIndex: number, supertaskId: number, title: string): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/entries/supertasks/update`, {
            credentials: "include",
            headers: defaultHeaders(),
            method: "PATCH",
            body: JSON.stringify({
                entryId: supertaskId,
                title
            })
        })

        if (response.ok) {
            entries.value[rotationId][supertaskIndex].entry.title = title
            cacheAndMemoEntries(rotationId, entries.value[rotationId])

            return true
        }

        return false
    }

    const deleteSupertask = async (rotationId: number, supertaskIndex: number, supertaskId: number): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/entries/supertasks/delete`, {
            credentials: "include",
            headers: defaultHeaders(),
            method: "DELETE",
            body: JSON.stringify({
                entryId: supertaskId
            })
        })

        if (response.ok) {
            entries.value[rotationId].splice(supertaskIndex, 1);
            cacheAndMemoEntries(rotationId, entries.value[rotationId])

            return true
        }

        return false
    }

    const createTask = async (title: string, rotationId: number, supertaskId: number, supertaskIndex: number): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/entries/tasks/create`, {
            credentials: "include",
            headers: defaultHeaders(),
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

            cacheAndMemoEntries(rotationId, entries.value[rotationId])
            return true
        }

        return false
    }

    const updateTask = async (rotationId: number, supertaskIndex: number, taskIndex: number, taskId: number, title: string): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/entries/tasks/update`, {
            credentials: "include",
            headers: defaultHeaders(),
            method: "PATCH",
            body: JSON.stringify({
                entryId: taskId,
                title
            })
        })

        if (response.ok) {
            entries.value[rotationId][supertaskIndex].children[taskIndex].entry.title = title
            cacheAndMemoEntries(rotationId, entries.value[rotationId])

            return true
        }

        return false
    }

    const deleteTask = async (rotationId: number, supertaskIndex: number, taskIndex: number, taskId: number): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/entries/tasks/delete`, {
            credentials: "include",
            headers: defaultHeaders(),
            method: "DELETE",
            body: JSON.stringify({
                entryId: taskId
            })
        })

        if (response.ok) {
            entries.value[rotationId][supertaskIndex].children.splice(taskIndex, 1);
            cacheAndMemoEntries(rotationId, entries.value[rotationId])

            return true
        }

        return false
    }

    const createSubtask = async (title: string, rotationId: number, taskId: number, supertaskIndex: number, taskIndex: number): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/entries/subtasks/create`, {
            credentials: "include",
            headers: defaultHeaders(),
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

            cacheAndMemoEntries(rotationId, entries.value[rotationId])
            return true
        }

        return false
    }

    const updateSubtask = async (rotationId: number, supertaskIndex: number, taskIndex: number, subtaskIndex: number, subtaskId: number, title: string): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/entries/subtasks/update`, {
            credentials: "include",
            headers: defaultHeaders(),
            method: "PATCH",
            body: JSON.stringify({
                entryId: subtaskId,
                title
            })
        })

        if (response.ok) {
            entries.value[rotationId][supertaskIndex].children[taskIndex].children[subtaskIndex].title = title
            cacheAndMemoEntries(rotationId, entries.value[rotationId])

            return true
        }

        return false
    }

    const deleteSubtask = async (rotationId: number, supertaskIndex: number, taskIndex: number, subtaskIndex: number, subtaskId: number): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/entries/subtasks/delete`, {
            credentials: "include",
            headers: defaultHeaders(),
            method: "DELETE",
            body: JSON.stringify({
                entryId: subtaskId
            })
        })

        if (response.ok) {
            entries.value[rotationId][supertaskIndex].children[taskIndex].children.splice(subtaskIndex, 1);
            cacheAndMemoEntries(rotationId, entries.value[rotationId])

            return true
        }

        return false
    }

    const fetchEntries = async (rotationId: number): Promise<boolean> => {
        const memo = entriesMemo.get(rotationId)

        if (memo) {
            entries.value[rotationId] = memo
            return true
        }

        const [cachedEntries, cacheTimestamp] = retrieveRotationEntries(rotationId)

        const url = new URL(`${API_ENDPOINT}/api/entries/${rotationId}`)

        if (cacheTimestamp) {
            url.searchParams.append("entriesCacheTimestamp", cacheTimestamp)
        }

        const response = await fetch(url, {
            credentials: "include",
            headers: defaultHeaders()
        })

        if (response.status === 304) {
            entries.value[rotationId] = cachedEntries || []
            cacheAndMemoEntries(rotationId, entries.value[rotationId])

            return true
        }

        if (response.ok) {
            const data: EntryStructure = await response.json()

            entries.value[rotationId] = data
            cacheAndMemoEntries(rotationId, entries.value[rotationId])

            return true
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
        fetchEntries
    }
}

export default useEntries