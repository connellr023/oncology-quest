import { Ref, inject, ref } from "vue"
import { User, Session } from "../models/user"
import { Task } from "../models/task"
import { API_ENDPOINT } from "../utilities"

import useValidateUsername from "./validation/useValidateUsername"
import useValidatePassword from "./validation/useValidatePassword"
import useCache from "./useCache"

const useLogin = () => {
    const { username, usernameError } = useValidateUsername()
    const { password, passwordError } = useValidatePassword()
    const { cache, retrieve } = useCache()

    const loading = ref(false)
    const loginError = ref("")

    const session = inject<Ref<User | null>>("session")!
    const entries = inject<Ref<Task[]>>("entries")!

    const login = async () => {
        loading.value = true

        try {
            const cachedStructure = retrieve()
            const response = await fetch(`${API_ENDPOINT}/api/user/login`, {
                credentials: "include",
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({
                    username: username.value,
                    password: password.value,
                    structureCacheTimestamp: cachedStructure ? cachedStructure.lastUpdated : null
                })
            })

            if (response.ok) {
                const sessionData: Session = await response.json()
                session.value = sessionData.user

                if (sessionData.structure) {
                    entries.value = sessionData.structure.entries
                    cache(sessionData.structure)
                }
                else if (cachedStructure) {
                    entries.value = cachedStructure.entries
                }
            }
            else if (response.status === 401) {
                loginError.value = "That username and password combination is incorrect."
            }
            else {
                loginError.value = `Received status code ${response.status}`
            }
        }
        catch (error) {
            loginError.value = "Server error. Please try again later."
        }

        loading.value = false
    }

    return {
        username,
        usernameError,
        password,
        passwordError,
        loading,
        loginError,
        login
    }
}

export default useLogin