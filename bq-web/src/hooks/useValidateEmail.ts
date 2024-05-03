import { EMAIL_REGEX } from "../utility"
import useValidateField from "./useValidateField"

const useValidateEmail = () => {
    const { field, setField, error } = useValidateField(
        (field: string) => EMAIL_REGEX.test(field),
        "Please enter a valid email address."
    )

    return {
        email: field,
        setEmail: setField,
        emailError: error
    }
}

export default useValidateEmail