import { API_ENDPOINT } from "../utilities"

interface AllowResetPasswordResponse {
    passwordResetTimestamp: string
}

const useAllowResetPassword = () => {
    const allowReset = async (userId: number): Promise<Date | false> => {
        const response = await fetch(`${API_ENDPOINT}/api/users/allow-reset-password`, {
            credentials: "include",
            method: "PATCH",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ userId })
        })

        if (response.ok) {
            const data: AllowResetPasswordResponse = await response.json()
            return new Date(data.passwordResetTimestamp)
        }
        
        return false
    }

    return {
        allowReset
    }
}

export default useAllowResetPassword