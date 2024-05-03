import { useEffect, useRef } from "preact/hooks"
import { EMAIL_REGEX } from "../utility"

const useValidateEmail = () => {
    const email = useRef("");
    const emailError = useRef("");

    useEffect(() => {
        if (!EMAIL_REGEX.test(email.current)) {
            emailError.current = "Invalid email address.";
        }
        else {
            emailError.current = "";
        }
    }, [email.current]);

    return {
        email,
        emailError
    }
}

export default useValidateEmail