import useValidatePassword from "./useValidatePassword"
import useValidateField from "./useValidateField"

const useValidateConfirmationPassword = () => {
    const { password, setPassword, passwordError } = useValidatePassword()
    const { field, setField, error } = useValidateField(
        (field: string) => password === field,
        "Passwords do not match."
    )

    return {
        password,
        setPassword,
        passwordError,
        confirmPassword: field,
        setConfirmPassword: setField,
        confirmPasswordError: error
    }
}

export default useValidateConfirmationPassword