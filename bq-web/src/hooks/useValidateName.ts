import { NAME_REGEX } from "../utility"
import useValidateField from "./useValidateField"

const useValidateName = () => {
    const { field, setField, error } = useValidateField(
        (field: string) => NAME_REGEX.test(field),
        "Names must consist of only letters and spaces."
    )

    return {
        name: field,
        setName: setField,
        nameError: error
    }
}

export default useValidateName