import { Ref, inject } from "vue"
import { UserTaskStructure } from "../models/tasks"
import { API_ENDPOINT } from "../utilities"
import { User } from "../models/user"

interface CreateUserTaskResponse {
    id: number
}

const useUserTasks = () => {
    const tasks = inject<Ref<Record<number, UserTaskStructure>>>("tasks")!
    const session = inject<Ref<User>>("session")!
    const selectedUserTasks = inject<Ref<UserTaskStructure | null>>("selectedUserTasks")!

    const userTasksMemo = new Map<string, UserTaskStructure>()
    const genKey = (userId: number, rotationId: number) => `${userId}.${rotationId}`

    const fetchOwnTasksWithMemo = async (rotationId: number): Promise<boolean> => {
        const key = genKey(session.value.id, rotationId)
        const memo = userTasksMemo.get(key)
        
        if (memo) {
            selectedUserTasks.value = memo
            return true
        }

        const response = await fetch(`${API_ENDPOINT}/api/tasks/${rotationId}`, {
            credentials: "include",
            headers: {
                "Content-Type": "application/json"
            }
        })

        if (response.ok) {
            const data: UserTaskStructure = await response.json()

            tasks.value[rotationId] = data
            userTasksMemo.set(key, data)

            return true
        }

        return false
    }

    const fetchUserTasksWithMemo = async (rotationId: number, userId: number): Promise<boolean> => {
        const key = genKey(userId, rotationId)
        const memo = userTasksMemo.get(key)
        
        if (memo) {
            selectedUserTasks.value = memo
            return true
        }
        
        const response = await fetch(`${API_ENDPOINT}/api/tasks/${userId}/${rotationId}`, {
            credentials: "include",
            headers: {
                "Content-Type": "application/json"
            }
        })

        if (response.ok) {
            const data: UserTaskStructure = await response.json()

            selectedUserTasks.value = data
            userTasksMemo.set(key, data)

            return true
        }

        return false
    }

    const updateTask = async (rotationId: number, subtaskId: number, isCompleted: boolean, comment: string): Promise<boolean> => {
        const key = genKey(session.value.id, rotationId)
        
        if (tasks.value[rotationId][subtaskId]) {
            const response = await fetch(`${API_ENDPOINT}/api/tasks/update`, {
                credentials: "include",
                headers: {
                    "Content-Type": "application/json"
                },
                method: "PATCH",
                body: JSON.stringify({
                    id: tasks.value[rotationId][subtaskId].id,
                    isCompleted,
                    comment
                })
            })

            if (response.ok) {
                tasks.value[rotationId][subtaskId].isCompleted = isCompleted
                tasks.value[rotationId][subtaskId].comment = comment

                userTasksMemo.set(key, tasks.value[rotationId])
                return true
            }
        }
        else {
            const response = await fetch(`${API_ENDPOINT}/api/tasks/create`, {
                credentials: "include",
                headers: {
                    "Content-Type": "application/json"
                },
                method: "POST",
                body: JSON.stringify({
                    subtaskId,
                    rotationId,
                    isCompleted,
                    comment
                })
            })

            if (response.ok) {
                const data: CreateUserTaskResponse = await response.json()

                tasks.value[rotationId][subtaskId] = {
                    id: data.id,
                    userId: session.value.id,
                    subtaskId,
                    isCompleted,
                    comment
                }

                userTasksMemo.set(key, tasks.value[rotationId])
                return true
            }
        }

        return false
    }

    return {
        fetchOwnTasksWithMemo,
        fetchUserTasksWithMemo,
        updateTask
    }
}

export default useUserTasks