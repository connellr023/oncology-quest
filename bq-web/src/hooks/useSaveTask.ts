import { Ref, inject, ref } from "vue"
import { UserTask } from "../models/task"
import { API_ENDPOINT } from "../utilities"
import { UserSession } from "../models/user"

const useSaveTask = () => {
    const session = inject<Ref<UserSession>>("session")!

    const completed = ref(false)
    const comment = ref("")
    const message = ref("")
    const loading = ref(false)

    const save = async (index: [number, number, number]) => {
        loading.value = true

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
                session.value.user.tasks[index[0]][index[1]][index[2]] = task
            }
            else {
                message.value = `Server responded with ${response.status}.`
            }
        }
        catch (error) {
            message.value = "Failed to save task."
        }

        loading.value = false
    }

    return {
        completed,
        comment,
        message,
        loading,
        save
    }
}

export default useSaveTask