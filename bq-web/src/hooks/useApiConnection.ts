import { API_ENDPOINT } from "../utility";
import { UserSession } from "../models/user";
import { ref, onMounted } from "vue";

const useApiConnection = () => {
    const session = ref<UserSession | null>(null);
    const loading = ref(true);
    const connectionError = ref(false);

    const checkSession = async () => {
        try {
            const response = await fetch(`${API_ENDPOINT}/api/user/session`);

            if (response.ok) {
                const sessionData = await response.json();
                session.value = sessionData;
            }
        }
        catch (e) {
            connectionError.value = true;
        }

        loading.value = false;
    }

    onMounted(checkSession);

    return {
        session,
        loading,
        connectionError
    }
}

export default useApiConnection;