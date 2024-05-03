import { useEffect, useRef } from "preact/hooks"
import { PASSWORD_REGEX } from "../utility"

const useValidatePassword = () => {
    const password = useRef("");
    const passwordError = useRef("");

    useEffect(() => {
        if (!PASSWORD_REGEX.test(password.current)) {
            passwordError.current = "Password must be between 8 and 200 characters.";
        }
        else {
            passwordError.current = "";
        }
    }, [password.current]);

    return {
        password,
        passwordError
    }
}

export default useValidatePassword