import { Ref, inject, ref } from "vue"
import { User } from "../models/user"

import useJwt from "./useJwt"

const useLogout = () => {
    const { removeToken } = useJwt()

    const session = inject<Ref<User | null>>("session")!
    const message = ref("")

    const logout = async () => {
        message.value = "Logging out..."

        removeToken()

        session.value = null
        message.value = "Logged out"
    }

    return {
        message,
        logout
    }
}

export default useLogout