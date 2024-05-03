import { USERNAME_REGEX } from "../utility"
import useValidateField from "./useValidateField"

const useValidateUsername = () => {
    const { field, setField, error } = useValidateField(
        (field: string) => USERNAME_REGEX.test(field),
        "Username must be between 1 and 25 characters and can only contain letters, numbers, underscores, dashes, and periods."
    )

    return {
        username: field,
        setUsername: setField,
        usernameError: error
    }
}

export default useValidateUsername