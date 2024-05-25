import { Ref, ref } from "vue"
import { API_ENDPOINT } from "../utilities"
import { User, Session } from "../models/user"
import { UserTask } from "../models/task"
import useCache from "./useCache"
import { Domain } from "../models/domain"

const useSession = () => {
    const { retrieveOrCacheUserTasks, retrieveUserTasks } = useCache()

    const session = ref<User | null>(null)
    const tasks = ref<Map<number, UserTask>>(new Map())
    const domains = ref<Map<number, Domain>>(new Map())

    const loading = ref(true)
    const connectionError = ref(false)

    const updateSessionData = async (data: Session, session: Ref<User | null>, tasks: Ref<Map<number, UserTask>>, domains: Ref<Map<number, Domain>>) => {
        session.value = data.user;

        tasks.value = retrieveOrCacheUserTasks(
            data.tasks?.reduce((map, task) => map.set(task.id, task), new Map())
        );
        
        domains.value = data.domains.reduce(
            (map, domain) => map.set(domain.id, domain), new Map()
        );
    }

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
                updateSessionData(sessionData, session, tasks, domains)
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
        domains,
        loading,
        connectionError,
        fetchSession
    }
}

export default useSession;