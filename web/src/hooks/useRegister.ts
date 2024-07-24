import { ref } from "vue"
import { API_ENDPOINT } from "../utilities"

import useValidateUsername from "./validation/useValidateUsername"
import useValidateName from "./validation/useValidateName"
import useValidateConfirmedPassword from "./validation/useValidateConfirmedPassword"

const useRegister = () => {
    const { username, usernameError } = useValidateUsername()
    const { name, nameError } = useValidateName()
    const {
        password,
        passwordError,
        confirmedPassword,
        confirmedPasswordError,
    } = useValidateConfirmedPassword()

    const serverError = ref("")
    const loading = ref(false)

    const register = async () => {
        loading.value = true
        serverError.value = ""

        try {
            const response = await fetch(`${API_ENDPOINT}/api/users/register`, {
                credentials: "include",
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({
                    username: username.value,
                    name: name.value,
                    password: password.value,
                }),
            })

            if (!response.ok) {
                switch (response.status) {
                    case 400:
                        serverError.value = "Invalid registration data. Please check your inputs."
                        break
                    case 409:
                        serverError.value = "That username is already taken. Please choose another."
                        break
                    case 429:
                        serverError.value = "Too many requests. Please try again later."
                        break
                    default:
                        serverError.value = "Server error. Please try again later."
                        break
                }
            }
        }
        catch (error) {
            serverError.value = "Server error. Please try again later."
        }

        loading.value = false
    }

    return {
        username,
        usernameError,
        name,
        nameError,
        password,
        passwordError,
        confirmedPassword,
        confirmedPasswordError,
        serverError,
        register,
        loading
    }
}

export default useRegister