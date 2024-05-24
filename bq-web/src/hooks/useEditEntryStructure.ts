import { Ref, inject, ref } from "vue"
import { API_ENDPOINT } from "../utilities"
import { EntryIndex, Task } from "../models/task"
import useCache from "./useCache"

const useEditEntryStructure = () => {
    const { updateCache } = useCache()

    const message = ref("")
    const entries = inject<Ref<Task[]>>("entries")!

    const requestPush = async (title: string, index: EntryIndex): Promise<boolean> => {
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

    const requestPop = async (index: EntryIndex): Promise<boolean> => {
        try {
            const response = await fetch(`${API_ENDPOINT}/api/entries/update/pop`, {
                method: "DELETE",
                credentials: "include",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({ index })
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

    const pushSupertaskHeading = async (title: string) => {
        if (await requestPush(title, [])) {
            entries.value.push({
                title,
                tasks: []
            })
            updateCache(entries.value)
        }
    }

    const popSupertaskHeading = async () => {
        if (await requestPop([])) {
            entries.value.pop()
            updateCache(entries.value)
        }
    }

    const pushTaskHeading = async (title: string, supertaskIndex: number) => {
        if (await requestPush(title, [supertaskIndex])) {
            entries.value[supertaskIndex].tasks.push({
                title,
                tasks: []
            })
            updateCache(entries.value)
        }
    }

    const popTaskHeading = async (supertaskIndex: number) => {
        if (await requestPop([supertaskIndex])) {
            entries.value[supertaskIndex].tasks.pop()
            updateCache(entries.value)
        }
    }

    const pushSubtaskEntry = async (title: string, taskIndex: [number, number]) => {
        if (await requestPush(title, taskIndex)) {
            entries.value[taskIndex[0]].tasks[taskIndex[1]].tasks.push(title)
            updateCache(entries.value)
        }
    }

    const popSubtaskEntry = async (taskIndex: [number, number]) => {
        if (await requestPop(taskIndex)) {
            entries.value[taskIndex[0]].tasks[taskIndex[1]].tasks.pop()
            updateCache(entries.value)
        }
    }

    return {
        message,
        pushSupertaskHeading,
        popSupertaskHeading,
        pushTaskHeading,
        popTaskHeading,
        pushSubtaskEntry,
        popSubtaskEntry
    }
}

export default useEditEntryStructure