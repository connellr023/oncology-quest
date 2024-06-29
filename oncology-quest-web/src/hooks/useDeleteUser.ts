import { API_ENDPOINT } from "../utilities"

import useJwt from "./useJwt"

const useDeleteUser = () => {
    const { defaultHeaders } = useJwt()

    const deleteSelf = async (password: string): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/users/delete-self`, {
            credentials: "include",
            method: "DELETE",
            headers: defaultHeaders(),
            body: JSON.stringify({ password })
        })

        return response.ok
    }

    const deleteUser = async (userId: number): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/users/delete-other-user`, {
            credentials: "include",
            method: "DELETE",
            headers: defaultHeaders(),
            body: JSON.stringify({ userId })
        })

        return response.ok
    }

    return {
        deleteSelf,
        deleteUser
    }
}

export default useDeleteUser