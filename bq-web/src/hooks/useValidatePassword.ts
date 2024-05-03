import { useRef, useState } from "preact/hooks"
import { PASSWORD_REGEX } from "../utility"

const useValidatePassword = () => {
    const [password, setPassword] = useState("")
    const passwordError = useRef("")

    if (!PASSWORD_REGEX.test(password)) {
        passwordError.current = "Pasword must be between 8 and 200 characters."
    }
    else {
        passwordError.current = ""
    }

    return {
        password,
        setPassword,
        passwordError
    }
}

export default useValidatePassword