import { ref } from "vue"

interface Notification {
    message: string,
    success: boolean,
    isHidden: boolean
}

export const notification = ref<Notification>({
    message: "",
    success: false,
    isHidden: true
});

const useNotifications = () => {
    const pushNotification = (message: string, success: boolean = false) => {
        notification.value = {
            message,
            success,
            isHidden: false
        }

        const timeout = message.length * 65

        setTimeout(() => {
            notification.value.isHidden = true
        }, timeout)
    }

    return { pushNotification }
}

export default useNotifications