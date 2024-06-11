import { Ref, ref } from "vue"
import { API_ENDPOINT } from "../utilities"
import { User, Session } from "../models/user"
import { UserTask } from "../models/tasks"
import { Rotation } from "../models/rotation"
import useCache from "./useCache"

const useSession = () => {
    const { retrieveOrCacheUserTasks, retrieveUserTasks } = useCache()

    const session = ref<User | null>(null)
    const tasks = ref<Record<number, UserTask>>({})     // Maps subtask ID to UserTask
    const rotations = ref<Record<number, Rotation>>({}) // Maps rotation ID to Rotation

    const loading = ref(true)
    const connectionError = ref(false)

    const updateSessionData = async (data: Session, session: Ref<User | null>, tasks: Ref<Record<number, UserTask>>, rotations: Ref<Record<number, Rotation>>) => {
        session.value = data.user;
        rotations.value = data.rotations
        tasks.value = retrieveOrCacheUserTasks(data.user.id, data.tasks)
    }

    const fetchSession = async () => {
        try {
            const [_, taskCacheTimestamp] = retrieveUserTasks()
            const url = new URL(`${API_ENDPOINT}/api/users/session`)

            if (taskCacheTimestamp) {
                url.searchParams.append("taskCacheTimestamp", taskCacheTimestamp)
            }

            const response = await fetch(url, {
                credentials: "include",
                headers: {
                    "Content-Type": "application/json"
                }
            })
            
            if (response.ok) {
                const sessionData: Session = await response.json()
                updateSessionData(sessionData, session, tasks, rotations)
            }
        }
        catch (_) {
            connectionError.value = true
        }

        loading.value = false
    }

    return {
        updateSessionData,
        session,
        tasks,
        rotations,
        loading,
        connectionError,
        fetchSession
    }
}

export default useSession;