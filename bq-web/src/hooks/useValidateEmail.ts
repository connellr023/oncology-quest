import { useRef, useState } from "preact/hooks"
import { EMAIL_REGEX } from "../utility"

const useValidateEmail = () => {
    const [email, setEmail] = useState("")
    const emailError = useRef("")

    if (!EMAIL_REGEX.test(email)) {
        emailError.current = "Invalid email address."
    }
    else {
        emailError.current = ""
    }

    return {
        email,
        setEmail,
        emailError
    }
}

export default useValidateEmail