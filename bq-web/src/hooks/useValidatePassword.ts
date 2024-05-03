import { PASSWORD_REGEX } from "../utility"
import useValidateField from "./useValidateField"

const useValidatePassword = () => {
    const { field, setField, error } = useValidateField(
        (field: string) => PASSWORD_REGEX.test(field),
        "Password must be from 8 to 200 characters long."
    )

    return {
        password: field,
        setPassword: setField,
        passwordError: error
    }
}

export default useValidatePassword