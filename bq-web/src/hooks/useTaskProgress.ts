import { Ref, inject } from "vue"
import { UserSession } from "../models/user"
import { UserTaskEntries } from "../models/task"

const useTaskProgress = (userTasks: UserTaskEntries) => {
    const entries = inject<Ref<UserSession>>("session")!.value.entries

    const calculateSubtaskProgress = (index: [number, number]): number => {
        if (index.length !== 2) {
            return 0
        }

        if (!userTasks[index[0]] || !userTasks[index[0]][index[1]]) {
            return 0
        }

        let total = 0
        const completedTasks = userTasks[index[0]][index[1]]
        const toComplete = entries[index[0]].tasks[index[1]].tasks.length

        for (const taskKey in completedTasks) {
            if (completedTasks[taskKey].completed) {
                total++
            }
        }

        return (total / toComplete) * 100
    }

    const calculateTaskProgress = (index: number): number => {
        if (!userTasks[index]) {
            return 0
        }

        let progress = 0
        const toComplete = entries[index].tasks.length

        for (let i = 0; i < toComplete; i++) {
            progress += calculateSubtaskProgress([index, i])
        }

        return progress / toComplete
    }

    return {
        calculateSubtaskProgress,
        calculateTaskProgress
    }
}

export default useTaskProgress