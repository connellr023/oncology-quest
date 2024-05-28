import { Ref, inject, watch } from "vue"
import { EntryStructure, UserTask } from "../models/tasks"

const useProgress = (userTasks: Record<number, UserTask>) => {
    const entries = inject<Ref<Record<number, EntryStructure>>>("entries")!

    const memoizedTaskProgress = new Map<number, number>()
    const memoizedSupertaskProgress = new Map<number, number>()

    watch(() => userTasks, () => {
        memoizedTaskProgress.clear()
        memoizedSupertaskProgress.clear()
    })

    const calculateTaskProgress = (domainId: number, taskId: number, supertaskIndex: number, taskIndex: number): number => {
        if (memoizedTaskProgress.has(taskId)) {
            return memoizedTaskProgress.get(taskId)!
        }
        
        let completedTasks = 0
        let totalTasks = 0

        entries.value[domainId][supertaskIndex].children[taskIndex].children.forEach((subtask) => {
            const userTask = userTasks[subtask.id]

            if (!userTask) {
                return
            }

            if (userTask.isComplete) {
                completedTasks++
            }

            totalTasks++
        })

        const progress = (completedTasks / totalTasks) || 0
        memoizedTaskProgress.set(taskId, progress)

        return progress
    }

    const calculateSupertaskProgress = (domainId: number, supertaskId: number, supertaskIndex: number): number => {
        if (memoizedSupertaskProgress.has(supertaskId)) {
            return memoizedSupertaskProgress.get(supertaskId)!
        }
        
        let completedTasks = 0
        let totalTasks = 0

        entries.value[domainId][supertaskIndex].children.forEach((task, taskIndex) => {
            const progress = calculateTaskProgress(domainId, task.entry.id, supertaskIndex, taskIndex)

            if (progress === 1) {
                completedTasks++
            }

            totalTasks++
        })

        const progress = (completedTasks / totalTasks) || 0
        memoizedSupertaskProgress.set(supertaskId, progress)

        return progress
    }

    return {
        calculateTaskProgress,
        calculateSupertaskProgress
    }
}

export default useProgress