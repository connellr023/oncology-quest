import { ref } from "vue"
import { API_ENDPOINT } from "../utilities"

import useValidateConfirmedPassword from "./validation/useValidateConfirmedPassword"
import useValidateUsername from "./validation/useValidateUsername"

const useResetPassword = () => {
    const resetError = ref("")
    const loading = ref(false)

    const { username, usernameError } = useValidateUsername()
    const {
        confirmedPassword,
        confirmedPasswordError,
        password,
        passwordError
    } = useValidateConfirmedPassword()

    const requestResetPassword = async () => {
        resetError.value = ""
        loading.value = true

        const response = await fetch(`${API_ENDPOINT}/api/users/reset-password`, {
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

        loading.value = false

        if (response.ok) {
            return true
        }
        else {
            switch (response.status) {
                case 403:
                    resetError.value = "That didn't work. Check your credentials and that you have a reset available."
                    break
                case 500:
                    resetError.value = "Internal server error."
                    break
                default:
                    resetError.value = "An unknown error occurred."
                    break
            }
        }
    }

    return {
        confirmedPassword,
        confirmedPasswordError,
        password,
        passwordError,
        requestResetPassword,
        username,
        usernameError,
        resetError,
        loading
    }
}

export default useResetPassword