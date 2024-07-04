import { RESET_TOKEN_REGEX } from "../../utilities"
import useValidateField from "./useValidateField"

const useValidateResetToken = () => {
    const { field, error } = useValidateField(
        (token: string) => RESET_TOKEN_REGEX.test(token),
        "Reset token must be 4 characters long and alphanumeric."
    )

    return {
        token: field,
        tokenError: error
    }
}

export default useValidateResetToken