import { ENTRY_TITLE_REGEX } from "../utilities"
import useValidateField from "./useValidateField"

const useValidateTitle = () => {
    const { field, error } = useValidateField(
        (title: string) => ENTRY_TITLE_REGEX.test(title),
        "Title must be from 1 to 100 characters long and contain only letters, numbers, and the characters +, -, and /."
    )

    return {
        title: field,
        titleError: error
    }
}

export default useValidateTitle