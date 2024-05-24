import { ref, onMounted } from "vue"
import { API_ENDPOINT } from "../utilities"
import { User, Session } from "../models/user"
import { UserTask } from "../models/task"
import useCache from "./useCache"

const useFetchSession = () => {
    const { cacheUserTasks, retrieveUserTasks } = useCache()

    const session = ref<User | null>(null)
    const tasks = ref<UserTask[]>([])

    const loading = ref(true)
    const connectionError = ref(false)

    const checkSession = async () => {
        try {
            const [cachedTasks, taskCacheTimestamp] = retrieveUserTasks()
            const route = cachedTasks ? `session?taskCacheTimestamp=${taskCacheTimestamp}` : "session"
            const response = await fetch(`${API_ENDPOINT}/api/user/${route}`, {
                credentials: "include",
                headers: {
                    "Content-Type": "application/json"
                }
            })
    
            if (response.ok) {
                const sessionData: Session = await response.json()
                session.value = sessionData.user

                if (sessionData.tasks) {
                    tasks.value = sessionData.tasks
                    cacheUserTasks(sessionData.tasks)
                }
                else if (cachedTasks) {
                    tasks.value = cachedTasks
                }
            }
        }
        catch (_) {
            connectionError.value = true
        }

        loading.value = false
    }

    onMounted(checkSession);

    return {
        session,
        tasks,
        loading,
        connectionError
    }
}

export default useFetchSession;