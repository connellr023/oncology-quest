import { ref } from "vue"
import { API_ENDPOINT } from "../utilities"
import { User, Session } from "../models/user"
import { UserTask } from "../models/task"
import useCache from "./useCache"
import { Domain } from "../models/domain"

const useFetchSession = () => {
    const { retrieveOrCacheUserTasks, retrieveUserTasks } = useCache()

    const session = ref<User | null>(null)
    const tasks = ref<UserTask[]>([])
    const domains = ref<Domain[]>([])

    const loading = ref(true)
    const connectionError = ref(false)

    const fetchSession = async () => {
        try {
            const [cachedTasks, taskCacheTimestamp] = retrieveUserTasks()
            const query_param = cachedTasks ? taskCacheTimestamp : ""
            const response = await fetch(`${API_ENDPOINT}/api/user/session/${query_param}`, {
                credentials: "include",
                headers: {
                    "Content-Type": "application/json"
                }
            })
    
            if (response.ok) {
                const sessionData: Session = await response.json()
                
                session.value = sessionData.user
                tasks.value = retrieveOrCacheUserTasks(sessionData.tasks)
                domains.value = sessionData.domains
            }
        }
        catch (_) {
            connectionError.value = true
        }

        loading.value = false
    }

    return {
        session,
        tasks,
        domains,
        loading,
        connectionError,
        fetchSession
    }
}

export default useFetchSession;