import { ref } from "vue"
import { API_ENDPOINT } from "../utilities"
import { User, Session } from "../models/user"
import { Rotation } from "../models/rotation"

const useSession = () => {
    const session = ref<User | null>(null)
    const rotations = ref<Record<number, Rotation>>({}) // Maps rotation ID to Rotation

    const loading = ref(true)
    const connectionError = ref(false)

    const fetchSession = async () => {
        try {
            const url = new URL(`${API_ENDPOINT}/api/users/session`)

            const response = await fetch(url, {
                credentials: "include",
                headers: {
                    "Content-Type": "application/json"
                }
            })
            
            if (response.ok) {
                const data: Session = await response.json()
                
                session.value = data.user;
                rotations.value = data.rotations
            }
        }
        catch (_) {
            connectionError.value = true
        }

        loading.value = false
    }

    return {
        session,
        rotations,
        loading,
        connectionError,
        fetchSession
    }
}

export default useSession;