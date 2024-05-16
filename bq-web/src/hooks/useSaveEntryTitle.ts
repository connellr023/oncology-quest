import { Ref, inject, ref } from "vue"
import { API_ENDPOINT } from "../utilities"
import useValidateTitle from "./useValidateTitle"
import { Task } from "../models/task"
import useStructureCache from "./useStructureCache"

const useSaveEntryTitle = () => {
    const { title, titleError } = useValidateTitle()
    const { updateCache } = useStructureCache()

    const entries = inject<Ref<Task[]>>("entries")!
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

            switch (index.length) {
                case 1:
                    entries.value[index[0]].title = title.value
                    break
                case 2:
                    entries.value[index[0]].tasks[index[1]].title = title.value
                    break
                case 3:
                    entries.value[index[0]].tasks[index[1]].tasks[index[2]] = title.value
                    break
                default:
                    message.value = "Invalid index."
                    return false
            }

            updateCache(entries.value)
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

export default useSaveEntryTitle