import { Ref, inject, ref } from "vue"
import { User } from "../models/user"
import { API_ENDPOINT } from "../utilities"

const useLogout = () => {
    const session = inject<Ref<User | null>>("session")!
    const message = ref("")

    const logout = async () => {
        message.value = "Logging out..."

        const response = await fetch(`${API_ENDPOINT}/api/users/logout`, {
            method: "POST",
            credentials: "include",
        })

        if (!response.ok) {
            message.value = "Failed to log out"
            return
        }

        session.value = null
        message.value = "Logged out"
    }

    return {
        message,
        logout
    }
}

export default useLogout