import useValidateField from "./useValidateField"
import useValidatePassword from "./useValidatePassword"

const useValidateConfirmedPassword = () => {
    const { password, passwordError } = useValidatePassword()
    const { field, error } = useValidateField(
        (confirmedPassword: string) => confirmedPassword === password.value,
        "Passwords do not match."
    )

    return {
        password,
        passwordError,
        confirmedPassword: field,
        confirmedPasswordError: error
    }
}

export default useValidateConfirmedPassword