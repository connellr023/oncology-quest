import { ref, watch } from "vue";

const useValidateField = (check: (field: string) => boolean, errorMessage: string) => {
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

export default useValidateField;