import { Ref, ref } from "vue"
import { API_ENDPOINT } from "../utilities"
import { User, Session } from "../models/user"
import { UserTask } from "../models/tasks"
import { Domain } from "../models/domain"
import useCache from "./useCache"

const useSession = () => {
    const { retrieveOrCacheUserTasks, retrieveUserTasks } = useCache()

    const session = ref<User | null>(null)
    const tasks = ref<Record<number, UserTask>>({}) // Maps subtask ID to UserTask
    const domains = ref<Record<number, Domain>>({}) // Maps domain ID to Domain

    const loading = ref(true)
    const connectionError = ref(false)

    const updateSessionData = async (data: Session, session: Ref<User | null>, tasks: Ref<Record<number, UserTask>>, domains: Ref<Record<number, Domain>>) => {
        session.value = data.user;
        domains.value = data.domains
        tasks.value = retrieveOrCacheUserTasks(data.user.id, data.tasks)
    }

    const fetchSession = async () => {
        try {
            const [_, taskCacheTimestamp] = retrieveUserTasks()
            const endpoint = taskCacheTimestamp ? `${taskCacheTimestamp}` : ""

            const response = await fetch(`${API_ENDPOINT}/api/user/session/${endpoint}`, {
                credentials: "include",
                headers: {
                    "Content-Type": "application/json"
                }
            })
            
            if (response.ok) {
                const sessionData: Session = await response.json()
                updateSessionData(sessionData, session, tasks, domains)
            }
        }
        catch (err) {
            console.log(err)
            connectionError.value = true
        }

        loading.value = false
    }

    return {
        updateSessionData,
        session,
        tasks,
        domains,
        loading,
        connectionError,
        fetchSession
    }
}

export default useSession;