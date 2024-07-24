import { ref, watch } from "vue"
import { isPasswordBanned } from "../../utilities"

import useValidatePassword from "./useValidatePassword"

const useValidateConfirmedPassword = () => {
    const confirmedPassword = ref("")
    const confirmedPasswordError = ref("")

    const { password, passwordError } = useValidatePassword()

    watch(() => [password.value, confirmedPassword.value], () => {
        if (password.value !== confirmedPassword.value) {
            confirmedPasswordError.value = "Passwords do not match."
        }
        else if (isPasswordBanned(confirmedPassword.value)) {
            confirmedPasswordError.value = "Password is too weak. Please try a different one."
        }
        else {
            confirmedPasswordError.value = ""
        }
    })

    return {
        password,
        passwordError,
        confirmedPassword,
        confirmedPasswordError
    }
}

export default useValidateConfirmedPassword