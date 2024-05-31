import { Ref, inject } from "vue"
import { EntryStructure, UserTask } from "../models/tasks"

const useProgress = (userTasks: Record<number, UserTask>) => {
    const entries = inject<Ref<Record<number, EntryStructure>>>("entries")!

    const calculateTaskProgress = (domainId: number, supertaskIndex: number, taskIndex: number): number => {
        let completedTasks = 0
        let totalTasks = 0

        entries.value[domainId][supertaskIndex].children[taskIndex].children.forEach((subtask) => {
            const userTask = userTasks[subtask.id]
            totalTasks++

            if (!userTask) {
                return
            }

            if (userTask.isCompleted) {
                completedTasks++
            }
        })

        const progress = ((completedTasks / totalTasks) * 100) || 0
        return progress
    }

    const calculateSupertaskProgress = (domainId: number, supertaskIndex: number): number => {
        let totalProgress = 0
        let totalTasks = 0

        entries.value[domainId][supertaskIndex].children.forEach((_, taskIndex) => {
            const progress = calculateTaskProgress(domainId, supertaskIndex, taskIndex)
            
            totalProgress += progress
            totalTasks++
        })

        const progress = (totalProgress / totalTasks) || 0
        return progress
    }

    return {
        calculateTaskProgress,
        calculateSupertaskProgress
    }
}

export default useProgress