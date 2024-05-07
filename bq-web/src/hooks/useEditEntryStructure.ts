import { Ref, inject, ref } from "vue"
import { UserSession } from "../models/user"
import { API_ENDPOINT } from "../utilities"

const useEditEntryStructure = () => {
    const message = ref("")
    const sessionContext = inject<Ref<UserSession>>("session")!

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
            sessionContext.value.entries.push({
                title,
                tasks: []
            })
        }
    }

    const popTaskHeading = async () => {
        if (await requestPop([])) {
            sessionContext.value.entries.pop()
        }
    }

    const pushSubTaskHeading = async (title: string, index: number) => {
        if (await requestPush(title, [index])) {
            sessionContext.value.entries[index].tasks.push({
                title,
                tasks: []
            })
        }
    }

    const popSubTaskHeading = async (index: number) => {
        if (await requestPop([index])) {
            sessionContext.value.entries[index].tasks.pop()
        }
    }

    const pushSubTaskEntry = async (title: string, index: number[]) => {
        if (await requestPush(title, index)) {
            sessionContext.value.entries[index[0]].tasks[index[1]].tasks.push(title)
        }
    }

    const popSubTaskEntry = async (index: number[]) => {
        if (await requestPop(index)) {
            sessionContext.value.entries[index[0]].tasks[index[1]].tasks.pop()
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