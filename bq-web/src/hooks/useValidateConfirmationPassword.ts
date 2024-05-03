import { useRef, useEffect } from "preact/hooks"
import useValidatePassword from "./useValidatePassword"

const useValidateConfirmationPassword = () => {
    const { password, passwordError } = useValidatePassword()
    const confirmPassword = useRef("")
    const confirmPasswordError = useRef("")

    useEffect(() => {
        confirmPasswordError.current = passwordError.current

        if (password.current !== confirmPassword.current) {
            confirmPasswordError.current = "Passwords do not match."
        }
        else {
            confirmPasswordError.current = ""
        }
    }, [password.current, confirmPassword.current])


    return {
        password,
        confirmPassword,
        confirmPasswordError
    }
}

export default useValidateConfirmationPassword