import { Ref, inject, ref } from "vue"
import { UserSession } from "../models/user"

const useSaveEntries = () => {
    const message = ref("")
    const saveError = ref("")

    const sessionContext = inject<Ref<UserSession>>("session")!

    const save = async (title: string, index: number[]) => {
        message.value = "Loading..."
        saveError.value = ""

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
                return saveError.value = "Invalid index."
        }

        try {
            const response = await fetch("/api/entries", {
                method: "PATCH",
                credentials: "include",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(sessionContext.value.entries),
            })

            if (!response.ok) {
                saveError.value = "Failed to save entries."
            }
            else {
                message.value = "Saved!"
            }
        } catch (_) {
            saveError.value = "Failed to save entries."
        }
    }

    return {
        message,
        saveError,
        save
    }
}

export default useSaveEntries