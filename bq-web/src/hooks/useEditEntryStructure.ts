import { Ref, inject, ref } from "vue"
import { UserSession } from "../models/user"

const useEditEntryStructure = () => {
    const message = ref("")
    const sessionContext = inject<Ref<UserSession>>("session")!

    const pushTaskHeading = () => {
        sessionContext.value.entries.push({
            title: "New Task Heading",
            tasks: []
        })
    }

    const popTaskHeading = () => {
        sessionContext.value.entries.pop()
    }

    const pushSubTaskHeading = (index: number) => {
        sessionContext.value.entries[index].tasks.push({
            title: "New Sub Task Heading",
            tasks: []
        })
    }

    const popSubTaskHeading = (index: number) => {
        sessionContext.value.entries[index].tasks.pop()
    }

    const pushSubTaskEntry = (index: number[]) => {
        sessionContext.value.entries[index[0]].tasks[index[1]].tasks.push("New Sub Task Entry")
    }

    const popSubTaskEntry = (index: number[]) => {
        sessionContext.value.entries[index[0]].tasks[index[1]].tasks.pop()
    }

    return {
        message,
        pushTaskHeading,
        popTaskHeading,
        pushSubTaskHeading,
        popSubTaskHeading,
        pushSubTaskEntry,
        popSubTaskEntry
    }
}

export default useEditEntryStructure