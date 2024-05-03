import { useRef, useState } from "preact/hooks"
import { NAME_REGEX } from "../utility"

const useValidateName = () => {
    const [name, setName] = useState("")
    const nameError = useRef("")

    if (!NAME_REGEX.test(name)) {
        nameError.current = "Name can only contain letters, and spaces."
    }
    else {
        nameError.current = ""
    }
    return {
        name,
        setName,
        nameError
    }
}

export default useValidateName