import { API_ENDPOINT } from "../utilities";
import { UserSession } from "../models/user";
import { ref, onMounted, Ref } from "vue";

const useApiConnection = (session: Ref<UserSession | null>) => {
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
                const sessionData = await response.json()
                session.value = sessionData
            }
        }
        catch (e) {
            connectionError.value = true
        }

        loading.value = false
    }

    onMounted(checkSession);

    return {
        loading,
        connectionError
    }
}

export default useApiConnection;