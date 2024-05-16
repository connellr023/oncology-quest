import { ref, onMounted } from "vue"
import { API_ENDPOINT } from "../utilities"
import { User, UserSessionResponse } from "../models/user"
import { Task } from "../models/task"
import useStructureCache from "./useStructureCache"

const useFetchSession = () => {
    const { cache, retrieve } = useStructureCache()

    const session = ref<User | null>(null)
    const entries = ref<Task[]>([])

    const loading = ref(true)
    const connectionError = ref(false)

    const checkSession = async () => {
        try {
            const cachedStructure = retrieve()
            const route = cachedStructure ? `session?structureCacheTimestamp=${cachedStructure.lastUpdated}` : "session"
            const response = await fetch(`${API_ENDPOINT}/api/user/${route}`, {
                credentials: "include",
                headers: {
                    "Content-Type": "application/json"
                }
            })
    
            if (response.ok) {
                const sessionData: UserSessionResponse = await response.json()
                session.value = sessionData.user

                if (sessionData.structure) {
                    entries.value = sessionData.structure.entries
                    cache(sessionData.structure)
                }
                else if (cachedStructure) {
                    entries.value = cachedStructure.entries
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
        entries,
        loading,
        connectionError
    }
}

export default useFetchSession;