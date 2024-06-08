import { ENTRY_TITLE_REGEX } from "../../utilities"
import useValidateField from "./useValidateField"

const useValidateTitle = () => {
    const { field, error } = useValidateField(
        (title: string) => ENTRY_TITLE_REGEX.test(title),
        "Title can only contain letters, numbers, and the characters +, -, (, ), and / and be within 1 and 20 characters."
    )

    return {
        title: field,
        titleError: error
    }
}

export default useValidateTitle