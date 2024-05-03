import { useEffect, useState } from "preact/hooks"
import { API_ENDPOINT } from "../utility"
import { UserSession } from "../models/user"

const useApiConnection = () => {
    const [connectionError, setConnectionError] = useState(false)
    const [session, setSession] = useState<UserSession | null>(null)
    const [loading, setLoading] = useState(true)

    useEffect(() => {
        const checkSession = async () => {
            try {
                let response = await fetch(`${API_ENDPOINT}/api/user/session`)

                if (response.ok) {
                    let session = await response.json()
                    setSession(session)
                }
            }
            catch (e) {
                setConnectionError(true)
            }

            setLoading(false)
        }

        checkSession()
    })

    return {
        connectionError,
        loading,
        session,
        setSession
    }
}

export default useApiConnection;