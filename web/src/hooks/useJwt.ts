const useJwt = () => {
    const setToken = (token: string) => {
        localStorage.setItem("token", token)
    }

    const retrieveToken = () => {
        return localStorage.getItem("token")
    }

    const removeToken = () => {
        localStorage.removeItem("token")
    }

    const defaultHeaders = (): HeadersInit => {
        const token = retrieveToken()

        if (token) {
            return {
                "Content-Type": "application/json",
                "Authorization": token
            }
        }
        else {
            return {
                "Content-Type": "application/json"
            }
        }
    }

    return {
        setToken,
        retrieveToken,
        removeToken,
        defaultHeaders
    }
}

export default useJwt;