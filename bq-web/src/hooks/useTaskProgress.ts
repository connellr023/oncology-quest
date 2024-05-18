import { Ref, inject } from "vue"
import { Task, UserTaskEntries } from "../models/task"

const useTaskProgress = (userTasks: UserTaskEntries) => {
    const entries = inject<Ref<Task[]>>("entries")!.value

    const calculateTaskProgress = (index: [number, number]): number => {
        if (index.length !== 2) {
            return 0
        }

        if (!userTasks[index[0]] || !userTasks[index[0]][index[1]]) {
            return 0
        }

        let total = 0
        const completedTasks = userTasks[index[0]][index[1]]
        const toComplete = entries[index[0]].tasks[index[1]].tasks.length

        if (toComplete === 0) {
            return 100
        }

        for (const taskKey in completedTasks) {
            if (completedTasks[taskKey].completed && entries[index[0]].tasks[index[1]].tasks[taskKey]) {
                total++
            }
        }

        return (total / toComplete) * 100
    }

    const calculateSupertaskProgress = (index: number): number => {
        if (!userTasks[index]) {
            return 0
        }

        let progress = 0
        const toComplete = entries[index].tasks.length

        if (toComplete === 0) {
            return 100
        }

        for (let i = 0; i < toComplete; i++) {
            if (entries[index].tasks[i]) {
                progress += calculateTaskProgress([index, i])
            }
        }

        return progress / toComplete
    }

    return {
        calculateTaskProgress,
        calculateSupertaskProgress
    }
}

export default useTaskProgress