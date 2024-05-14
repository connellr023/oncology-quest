import { Ref, inject } from "vue"
import { UserSession } from "../models/user"

const useTaskProgress = () => {
    const session = inject<Ref<UserSession>>("session")!.value

    const calculateSubtaskProgress = (index: [number, number]): number => {
        if (index.length !== 2) {
            return 0
        }

        if (!session.user.tasks[index[0]] || !session.user.tasks[index[0]][index[1]]) {
            return 0
        }

        let total = 0
        const completedTasks = session.user.tasks[index[0]][index[1]]
        const toComplete = session.entries[index[0]].tasks[index[1]].tasks.length

        for (const taskKey in completedTasks) {
            if (completedTasks[taskKey].completed) {
                total++
            }
        }

        return (total / toComplete) * 100
    }

    const calculateTaskProgress = (index: number): number => {
        if (!session.user.tasks[index]) {
            return 0
        }

        let progress = 0
        const toComplete = session.entries[index].tasks.length

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