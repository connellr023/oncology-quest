import { useRef, useState } from "preact/hooks"
import useValidatePassword from "./useValidatePassword"

const useValidateConfirmationPassword = () => {
    const { password, setPassword, passwordError } = useValidatePassword()
    const [confirmPassword, setConfirmPassword] = useState("")
    const confirmPasswordError = useRef("")

    if (password !== confirmPassword) {
        confirmPasswordError.current = "Passwords do not match."
    }
    else {
        confirmPasswordError.current = ""
    }

    return {
        password,
        setPassword,
        passwordError,
        confirmPassword,
        setConfirmPassword,
        confirmPasswordError
    }
}

export default useValidateConfirmationPassword