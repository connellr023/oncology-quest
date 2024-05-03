import { useRef, useState } from "preact/hooks"
import { USERNAME_REGEX } from "../utility"

const useValidateUsername = () => {
    const [username, setUsername] = useState("")
    const usernameError = useRef("")

    if (!USERNAME_REGEX.test(username)) {
        usernameError.current = "Username must be between 1 and 25 characters and can only contain letters, numbers, underscores, dashes, and periods."
    }
    else {
        usernameError.current = ""
    }

    return {
        username,
        setUsername,
        usernameError
    }
}

export default useValidateUsername