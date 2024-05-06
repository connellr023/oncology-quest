import { ref } from "vue"
import { UserTask } from "../models/task"
import { API_ENDPOINT } from "../utilities"

const useSaveTask = () => {
    const completed = ref(false)
    const comment = ref("")
    const message = ref("")

    const save = async (index: number[]) => {
        message.value = "Saving..."

        try {
            const task: UserTask = {
                completed: completed.value,
                comment: comment.value
            }

            const response = await fetch(`${API_ENDPOINT}/api/tasks/update`, {
                method: "PATCH",
                credentials: "include",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({
                    task,
                    index: index
                })
            })

            if (response.ok) {
                message.value = "Saved!"
            }
            else {
                message.value = `Server responded with ${response.status}.`
            }
        }
        catch (error) {
            message.value = "Failed to save task."
        }
    }

    return {
        completed,
        comment,
        message,
        save
    }
}

export default useSaveTask