import { API_ENDPOINT } from "../utilities"

import useJwt from "./useJwt"

interface AllowResetPasswordResponse<D = string | Date> {
    passwordResetTimestamp: D,
    resetToken: string
}

const useAllowResetPassword = () => {
    const allowReset = async (userId: number): Promise<AllowResetPasswordResponse<Date> | false> => {
        const { defaultHeaders } = useJwt()
        
        const response = await fetch(`${API_ENDPOINT}/api/users/allow-reset-password`, {
            credentials: "include",
            method: "PATCH",
            headers: defaultHeaders(),
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