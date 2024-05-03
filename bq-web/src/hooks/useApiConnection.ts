import { useEffect, useState } from "preact/hooks"
import { API_ENDPOINT } from "../utility"
import { UserSession } from "../models/user"

const useApiConnection = () => {
    let [connectionError, setConnectionError] = useState(false)
    let [session, setSession] = useState<UserSession | null>(null)
    let [loading, setLoading] = useState(true)

    useEffect(() => {
        const checkSession = async () => {
            try {
                let response = await fetch(`${API_ENDPOINT}/api/session`)

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
        session,
        loading
    }
}

export default useApiConnection;