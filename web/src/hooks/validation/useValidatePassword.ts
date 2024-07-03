import { PASSWORD_REGEX } from "../../utilities"
import useValidateField from "./useValidateField"

const useValidatePassword = () => {
    const { field, error } = useValidateField(
        (password: string) => PASSWORD_REGEX.test(password),
        "Password must be from 8 to 200 characters long."
    )

    return {
        password: field,
        passwordError: error
    }
}

export default useValidatePassword