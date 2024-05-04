import { USERNAME_REGEX } from "../utility";
import useValidateField from "./useValidateField";

const useValidateUsername = () => {
    const { field, error } = useValidateField(
        (username: string) => USERNAME_REGEX.test(username),
        "Invalid username."
    )

    return {
        username: field,
        usernameError: error
    }
}

export default useValidateUsername