import { EMAIL_REGEX } from "../../utilities"
import useValidateField from "./useValidateField"

const useValidateEmail = () => {
    const { field, error } = useValidateField(
        (email: string) => EMAIL_REGEX.test(email),
        "Invalid email address."
    )

    return {
        email: field,
        emailError: error
    }
}

export default useValidateEmail