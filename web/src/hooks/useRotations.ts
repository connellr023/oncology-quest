import { Ref, inject } from "vue"
import { API_ENDPOINT } from "../utilities"
import { Rotation } from "../models/rotation"

import useJwt from "./useJwt"

interface CreateRotationResponse {
    rotationId: number,
    lastUpdated: string
}

export const useRotations = () => {
    const { defaultHeaders } = useJwt()
    
    const rotations = inject<Ref<Record<number, Rotation>>>("rotations")!

    const createRotation = async (name: string): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/rotations/create`, {
            credentials: "include",
            method: "POST",
            headers: defaultHeaders(),
            body: JSON.stringify({ name })
        })

        if (response.ok) {
            const data: CreateRotationResponse = await response.json()
            const rotation: Rotation = {
                id: data.rotationId,
                name,
                lastUpdated: data.lastUpdated
            }

            rotations.value[data.rotationId] = rotation;
            return true
        }

        return false
    }

    const deleteRotation = async (rotationId: number): Promise<boolean> => {
        const response = await fetch(`${API_ENDPOINT}/api/rotations/delete`, {
            credentials: "include",
            method: "DELETE",
            headers: defaultHeaders(),
            body: JSON.stringify({ rotationId })
        })

        if (response.ok) {
            delete rotations.value[rotationId]
            return true
        }

        return false
    }

    return {
        createRotation,
        deleteRotation
    }
}

export default useRotations