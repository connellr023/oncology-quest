import { Ref, inject, ref } from "vue"
import { UserSession } from "../models/user"
import { API_ENDPOINT } from "../utilities"

const useLogout = () => {
    const sessionContext = inject<Ref<UserSession | null>>("session")!
    const message = ref("")

    const logout = async () => {
        message.value = "Logging out..."

        const response = await fetch(`${API_ENDPOINT}/api/user/logout`, {
            method: "POST",
            credentials: "include",
        })

        if (!response.ok) {
            message.value = "Failed to log out"
            return
        }

        sessionContext.value = null
        message.value = "Logged out"
    }

    return {
        message,
        logout
    }
}

export default useLogout