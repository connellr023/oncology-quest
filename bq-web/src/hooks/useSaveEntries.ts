import { ref } from "vue"
import { API_ENDPOINT } from "../utilities"
import useValidateTitle from "./useValidateTitle"

const useSaveEntries = () => {
    const { title, titleError } = useValidateTitle()
    const message = ref("")

    const save = async (index: number[]): Promise<boolean> => {
        if (titleError.value) {
            return false
        }

        message.value = "Loading..."

        try {
            const response = await fetch(`${API_ENDPOINT}/api/entries/update`, {
                method: "PATCH",
                credentials: "include",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({
                    title: title.value,
                    index
                }),
            })

            if (!response.ok) {
                message.value = `Server responded with ${response.status}.`
                return false
            }

            message.value = "Saved!"
            return true
        } catch (_) {
            message.value = "Failed to save entries."
            return false
        }
    }

    return {
        message,
        save,
        title,
        titleError
    }
}

export default useSaveEntries