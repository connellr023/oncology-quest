import { Ref, ref } from "vue"
import { API_ENDPOINT } from "../utilities"
import { User, Session } from "../models/user"
import { UserTask } from "../models/tasks"
import { Domain } from "../models/domain"
import useCache from "./useCache"

const useSession = () => {
    const { retrieveOrCacheUserTasks, retrieveUserTasks } = useCache()

    const session = ref<User | null>(null)
    const tasks = ref<Record<number, UserTask>>({})
    const domains = ref<Record<number, Domain>>({})

    const loading = ref(true)
    const connectionError = ref(false)

    const updateSessionData = async (data: Session, session: Ref<User | null>, tasks: Ref<Record<number, UserTask>>, domains: Ref<Record<number, Domain>>) => {
        session.value = data.user;
    
        let recordedTasks: Record<number, UserTask> | undefined = {}
        data.tasks?.forEach((task: UserTask) => {
            recordedTasks![task.subtaskId] = task
        })

        let recordedDomains: Record<number, Domain> = {}
        data.domains.forEach((domain: Domain) => {
            recordedDomains[domain.id] = domain
        })

        tasks.value = retrieveOrCacheUserTasks(data.user.id, recordedTasks)
        domains.value = recordedDomains
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