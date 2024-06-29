import { ref } from "vue"
import { User } from "../models/user"
import { API_ENDPOINT } from "../utilities"

import useJwt from "./useJwt"

const useUserSearch = () => {
    const results = ref<Record<number, User>>({})
    const loading = ref(false)
    const searchError = ref(false)

    const search = async (query: string) => {
        loading.value = true
        searchError.value = false

        const { defaultHeaders } = useJwt()

        try {
            searchError.value = false
            const response = await fetch(`${API_ENDPOINT}/api/users/search/${query}`, {
                credentials: "include",
                headers: defaultHeaders()
            })

            if (!response.ok) {
                searchError.value = true
            }
            else {
                try {
                    const users: Record<number, User> = await response.json()
                    results.value = users
                }
                catch (_) {
                    searchError.value = true
                }
            }
        }
        catch (_) {
            searchError.value = true
        }

        loading.value = false
    }

    return {
        search,
        results,
        loading,
        searchError
    }
}

export default useUserSearch