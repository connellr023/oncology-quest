import { API_ENDPOINT } from "../utilities"

interface AllowResetPasswordResponse<D = string | Date> {
    passwordResetTimestamp: D,
    resetToken: string
}

const useAllowResetPassword = () => {
    const allowReset = async (userId: number): Promise<AllowResetPasswordResponse<Date> | false> => {
        const response = await fetch(`${API_ENDPOINT}/api/users/allow-reset-password`, {
            credentials: "include",
            method: "PATCH",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ userId })
        })

        if (response.ok) {
            const data: AllowResetPasswordResponse<string> = await response.json()
            return {
                passwordResetTimestamp: new Date(data.passwordResetTimestamp),
                resetToken: data.resetToken
            }
        }
        
        return false
    }

    return {
        allowReset
    }
}

export default useAllowResetPassword