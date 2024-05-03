import { useEffect, useRef } from "preact/hooks"
import { NAME_REGEX } from "../utility"

const useValidateName = () => {
    const name = useRef("");
    const nameError = useRef("");

    useEffect(() => {
        if (!NAME_REGEX.test(name.current)) {
            nameError.current = "Name can only contain letters, and spaces.";
        }
        else {
            nameError.current = "";
        }
    }, [name.current]);

    return {
        name,
        nameError
    }
}

export default useValidateName