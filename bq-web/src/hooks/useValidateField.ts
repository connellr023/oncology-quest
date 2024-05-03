import { useEffect, useState } from "preact/hooks"
import debounce from "lodash.debounce"

const useValidateField = (check: (field: string) => boolean, errorMessage: string) => {
    const [field, setField] = useState("")
    const [error, setError] = useState("")

    useEffect(() => {
        if (!check(field)) {
            setError(errorMessage)
        }
        else {
            setError("")
        }
    }, [field]);

    return {
        field,
        setField: debounce(setField, 350),
        error
    }
}

export default useValidateField