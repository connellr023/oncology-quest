import { useEffect, useRef } from "preact/hooks"
import { USERNAME_REGEX } from "../utility"

const useValidateUsername = () => {
    const username = useRef("");
    const usernameError = useRef("");

    useEffect(() => {
        if (!USERNAME_REGEX.test(username.current)) {
            usernameError.current = "Username must be between 1 and 25 characters and can only contain letters, numbers, underscores, dashes, and periods.";
        }
        else {
            usernameError.current = "";
        }
    }, [username.current]);

    return {
        username,
        usernameError
    }
}

export default useValidateUsername