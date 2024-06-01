import { ref } from "vue"
import { UserWithTasks } from "../models/user"
import { API_ENDPOINT } from "../utilities"

const useUserSearch = () => {
    const results = ref<Record<number, UserWithTasks>>({})
    const loading = ref(false)
    const searchError = ref(false)

    const search = async (query: string) => {
        loading.value = true
        searchError.value = false

        try {
            searchError.value = false
            const response = await fetch(`${API_ENDPOINT}/api/user/search/${query}`, {
                credentials: "include",
                headers: {
                    "Content-Type": "application/json"
                }
            })

            if (!response.ok) {
                searchError.value = true
            }
            else {
                try {
                    const users: Record<number, UserWithTasks> = await response.json()
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