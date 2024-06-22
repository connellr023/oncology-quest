import { Ref, inject, ref } from "vue"
import { User, Session } from "../models/user"
import { Rotation } from "../models/rotation"
import { API_ENDPOINT } from "../utilities"

import useValidateUsername from "./validation/useValidateUsername"
import useValidatePassword from "./validation/useValidatePassword"

const useLogin = () => {
    const { username, usernameError } = useValidateUsername()
    const { password, passwordError } = useValidatePassword()

    const loading = ref(false)
    const loginError = ref("")

    const session = inject<Ref<User | null>>("session")!
    const rotations = inject<Ref<Record<number, Rotation>>>("rotations")!

    const login = async () => {
        loading.value = true

        try {
            const response = await fetch(`${API_ENDPOINT}/api/users/login`, {
                credentials: "include",
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({
                    username: username.value,
                    password: password.value
                })
            })

            if (response.ok) {
                const data: Session = await response.json()
                
                session.value = data.user;
                rotations.value = data.rotations
            }
            else {
                switch (response.status) {
                    case 401:
                        loginError.value = "That username and password combination is incorrect."
                        break
                    case 429:
                        loginError.value = "Too many requests. Please try again later."
                        break
                    case 500:
                        loginError.value = "Internal server error."
                        break
                    default:
                        loginError.value = "An unknown error occurred."
                        break
                }
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