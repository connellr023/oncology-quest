import { ref, watch } from "vue"

type ValidateFieldCallback = (field: string) => boolean

/**
 * Custom hook for validating a field.
 * @param check - A function that checks if the field is valid.
 * @param errorMessage - The error message to display if the field is invalid.
 * @returns An object containing the field and error properties.
 */
const useValidateField = (check: ValidateFieldCallback, errorMessage: string) => {
    const field = ref("")
    const error = ref("")

    watch(field, (newField) => {
        if (!check(newField)) {
            error.value = errorMessage
        }
        else {
            error.value = ""
        }
    })

    return {
        field,
        error
    }
}

export default useValidateField