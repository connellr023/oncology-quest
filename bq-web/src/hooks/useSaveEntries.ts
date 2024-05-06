import { Ref, inject, ref } from "vue"
import { UserSession } from "../models/user"
import { API_ENDPOINT } from "../utilities"

const useSaveEntries = () => {
    const message = ref("")
    const saveError = ref(false)
    const sessionContext = inject<Ref<UserSession>>("session")!

    const save = async (title: string, index: number[]) => {
        saveError.value = false
        message.value = "Loading..."

        switch (index.length) {
            case 1:
                sessionContext.value.entries[index[0]].title = title;
                break;
            case 2:
                sessionContext.value.entries[index[0]].tasks[index[1]].title = title;
                break;
            case 3:
                sessionContext.value.entries[index[0]].tasks[index[1]].tasks[index[2]] = title;
                break;
            case 0:
            default:
                return message.value = "Invalid index."
        }

        try {
            const response = await fetch(`${API_ENDPOINT}/api/entries/update`, {
                method: "PATCH",
                credentials: "include",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({
                    entries: sessionContext.value.entries
                }),
            })

            if (!response.ok) {
                message.value = `Server responded with ${response.status}.`
                saveError.value = true
            }
            else {
                message.value = "Saved!"
            }
        } catch (_) {
            message.value = "Failed to save entries."
            saveError.value = true
        }
    }

    return {
        message,
        saveError,
        save
    }
}

export default useSaveEntries