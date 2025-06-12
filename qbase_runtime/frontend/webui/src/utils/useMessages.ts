// composables/useMessages.ts
import { ref } from 'vue'

type Message = {
    id: number
    text: string
    type?: 'info' | 'success' | 'error'
    time: number
}

const messages = ref<Message[]>([])
let idCounter = 0

export function useMessages() {
    const addMessage = (text: string, type: Message['type'] = 'info', time: number = 5) => {
        messages.value.push({ id: ++idCounter, text, type, time })
    }

    const removeMessage = (id: number) => {
        messages.value = messages.value.filter(m => m.id !== id)
    }

    const clearMessages = () => {
        messages.value = []
    }

    return {
        messages,
        addMessage,
        removeMessage,
        clearMessages,
    }
}
