import { API_ENDPOINT } from "../utilities"

const useDeleteUser = () => {
    const deleteSelf = async (password: string): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/users/delete-self`, {
            credentials: "include",
            method: "DELETE",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ password })
        })

        return response.ok
    }

    const deleteUser = async (userId: number): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/users/delete-other-user`, {
            credentials: "include",
            method: "DELETE",
            headers: {
                "Content-Type": "application/json",
            },
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