import { ref } from "vue"
import { API_ENDPOINT } from "../utilities"

const useSaveEntries = () => {
    const message = ref("")
    const saveError = ref(false)

    const save = async (title: string, index: number[]) => {
        saveError.value = false
        message.value = "Loading..."

        try {
            const response = await fetch(`${API_ENDPOINT}/api/entries/update`, {
                method: "PATCH",
                credentials: "include",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({
                    title,
                    index
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