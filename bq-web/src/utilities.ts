export let API_ENDPOINT = import.meta.env.DEV ? "http://127.0.0.1:8080" : "..."

export const USERNAME_REGEX = /^[a-zA-Z0-9\-\_\.]{1,25}$/
export const NAME_REGEX = /^[a-zA-Z]+(\s[a-zA-Z]+)*$/
export const EMAIL_REGEX = /^[\w\-\.]+@([\w-]+\.)+[\w-]{2,4}$/
export const PASSWORD_REGEX = /^.{8,200}$/
export const COMMENT_REGEX = /^[a-zA-Z0-9\s.,!?'"()-]{0,150}$/