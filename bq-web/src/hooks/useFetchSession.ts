import { ref, onMounted } from "vue";
import { API_ENDPOINT } from "../utilities";
import { User, UserSessionResponse } from "../models/user";
import { Task } from "../models/task";

const useFetchSession = () => {
    const session = ref<User | null>(null)
    const entries = ref<Task[]>([])

    const loading = ref(true)
    const connectionError = ref(false)

    const checkSession = async () => {
        try {
            const response = await fetch(`${API_ENDPOINT}/api/user/session`, {
                credentials: "include",
                headers: {
                    "Content-Type": "application/json"
                }
            });
    
            if (response.ok) {
                const sessionData: UserSessionResponse = await response.json()

                session.value = sessionData.user
                entries.value = sessionData.entries
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