import { Ref, inject, ref } from "vue"
import { API_ENDPOINT } from "../utilities"
import { Task } from "../models/task"
import useStructureCache from "./useStructureCache"

const useEditEntryStructure = () => {
    const { updateCache } = useStructureCache()

    const message = ref("")
    const entries = inject<Ref<Task[]>>("entries")!

    const requestPush = async (title: string, index: number[]): Promise<boolean> => {
        message.value = "Loading..."

        try {
            const response = await fetch(`${API_ENDPOINT}/api/entries/update/push`, {
                method: "PATCH",
                credentials: "include",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({
                    title,
                    index
                })
            })

            if (!response.ok) {
                message.value = `Received status code ${response.status}`
                return false
            }

            message.value = "Pushed."
            return true
        }
        catch (_) {
            message.value = "Failed to push."
            return false
        }
    }

    const requestPop = async (index: number[]): Promise<boolean> => {
        try {
            const response = await fetch(`${API_ENDPOINT}/api/entries/update/pop`, {
                method: "DELETE",
                credentials: "include",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({
                    index
                })
            })

            if (!response.ok) {
                message.value = `Received status code ${response.status}`
                return false
            }

            message.value = "Popped."
            return true
        }
        catch (_) {
            message.value = "Failed to pop."
            return false
        }
    }

    const pushTaskHeading = async (title: string) => {
        if (await requestPush(title, [])) {
            entries.value.push({
                title,
                tasks: []
            })
            updateCache(entries.value)
        }
    }

    const popTaskHeading = async () => {
        if (await requestPop([])) {
            entries.value.pop()
            updateCache(entries.value)
        }
    }

    const pushSubTaskHeading = async (title: string, index: number) => {
        if (await requestPush(title, [index])) {
            entries.value[index].tasks.push({
                title,
                tasks: []
            })
            updateCache(entries.value)
        }
    }

    const popSubTaskHeading = async (index: number) => {
        if (await requestPop([index])) {
            entries.value[index].tasks.pop()
            updateCache(entries.value)
        }
    }

    const pushSubTaskEntry = async (title: string, index: number[]) => {
        if (await requestPush(title, index)) {
            entries.value[index[0]].tasks[index[1]].tasks.push(title)
            updateCache(entries.value)
        }
    }

    const popSubTaskEntry = async (index: number[]) => {
        if (await requestPop(index)) {
            entries.value[index[0]].tasks[index[1]].tasks.pop()
            updateCache(entries.value)
        }
    }

    return {
        message,
        pushTaskHeading,
        popTaskHeading,
        pushSubTaskHeading,
        popSubTaskHeading,
        pushSubTaskEntry,
        popSubTaskEntry
    }
}

export default useEditEntryStructure