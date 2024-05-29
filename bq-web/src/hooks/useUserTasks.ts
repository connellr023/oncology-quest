import { Ref } from "vue"
import { UserTask } from "../models/tasks"
import { API_ENDPOINT } from "../utilities"
import useCache from "./useCache"

interface CreateUserTaskResponse {
    id: number
}

const useUserTasks = (tasks: Ref<Record<number, UserTask>>, userId: number) => {
    const { cacheUserTasks } = useCache()

    const updateTask = async (subtaskId: number, isCompleted: boolean, comment: string): Promise<boolean> => {
        if (tasks.value[subtaskId]) {
            const response = await fetch(`${API_ENDPOINT}/api/user/tasks/update`, {
                credentials: "include",
                headers: {
                    "Content-Type": "application/json"
                },
                method: "PATCH",
                body: JSON.stringify({
                    id: tasks.value[subtaskId].id,
                    isCompleted,
                    comment
                })
            })

            if (response.ok) {
                tasks.value[subtaskId].isCompleted = isCompleted
                tasks.value[subtaskId].comment = comment

                cacheUserTasks(userId, tasks.value)
                return true
            }
        }
        else {
            const response = await fetch(`${API_ENDPOINT}/api/user/tasks/create`, {
                credentials: "include",
                headers: {
                    "Content-Type": "application/json"
                },
                method: "POST",
                body: JSON.stringify({
                    subtaskId,
                    isCompleted,
                    comment
                })
            })

            if (response.ok) {
                const data: CreateUserTaskResponse = await response.json()

                tasks.value[subtaskId] = {
                    id: data.id,
                    userId,
                    subtaskId,
                    isCompleted,
                    comment
                }

                cacheUserTasks(userId, tasks.value)
                return true
            }
        }

        return false
    }

    return {
        updateTask
    }
}

export default useUserTasks