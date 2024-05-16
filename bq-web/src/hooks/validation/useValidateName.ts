import { NAME_REGEX } from "../../utilities"
import useValidateField from "./useValidateField"

const useValidateName = () => {
    const { field, error } = useValidateField(
        (name: string) => NAME_REGEX.test(name),
        "Name must contain only letters and spaces."
    )

    return {
        name: field,
        nameError: error
    }
}

export default useValidateName