import { COMMENT_REGEX } from "../../utilities"
import useValidateField from "./useValidateField"

const useValidateComment = () => {
    const { field, error } = useValidateField(
        (comment: string) => COMMENT_REGEX.test(comment),
        "Comment contains invalid characters or is too long.",
    )

    return {
        comment: field,
        commentError: error
    }
}

export default useValidateComment