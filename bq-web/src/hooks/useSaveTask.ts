import { Ref, inject, ref } from "vue"
import { UserTask } from "../models/task"
import { API_ENDPOINT } from "../utilities"
import { User } from "../models/user"
import useValidateComment from "./useValidateComment"

const useSaveTask = () => {
    const session = inject<Ref<User>>("session")!

    const { comment, commentError } = useValidateComment()

    const completed = ref(false)
    const message = ref("")
    const loading = ref(false)

    const save = async (index: [number, number, number]): Promise<boolean> => {
        if (commentError.value) {
            message.value = commentError.value
            return false
        }

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
                    index
                })
            })

            if (response.ok) {
                if (!session.value.tasks[index[0]]) {
                    session.value.tasks[index[0]] = {}
                }
                
                if (!session.value.tasks[index[0]][index[1]]) {
                    session.value.tasks[index[0]][index[1]] = {}
                }
                
                if (!session.value.tasks[index[0]][index[1]][index[2]]) {
                    session.value.tasks[index[0]][index[1]][index[2]] = {
                        completed: false,
                        comment: ""
                    }
                }

                session.value.tasks[index[0]][index[1]][index[2]] = task

                message.value = "Saved!"
                loading.value = false
                return true
            }
            
            message.value = `Server responded with ${response.status}.`
        }
        catch (error) {
            console.error(error)
            message.value = "Failed to save task."
        }

        loading.value = false
        return false
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