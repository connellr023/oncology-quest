import { Ref, inject } from "vue"
import { API_ENDPOINT } from "../utilities"
import { Domain } from "../models/domain"

interface CreateDomainResponse {
    domainId: number,
    lastUpdated: string
}

export const useDomains = () => {
    const domains = inject<Ref<Record<number, Domain>>>("domains")!

    const createDomain = async (name: string): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/domains/create`, {
            credentials: "include",
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ name })
        })

        if (response.ok) {
            const data: CreateDomainResponse = await response.json()
            const domain: Domain = {
                id: data.domainId,
                name,
                lastUpdated: data.lastUpdated
            }

            domains.value[data.domainId] = domain;
            return true
        }

        return false
    }

    const deleteDomain = async (domainId: number): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/domains/delete`, {
            credentials: "include",
            method: "DELETE",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ domainId })
        })

        if (response.ok) {
            delete domains.value[domainId]
            return true
        }

        return false
    }

    return {
        createDomain,
        deleteDomain
    }
}

export default useDomains