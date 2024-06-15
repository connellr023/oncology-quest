// In production, the API endpoint is the same origin of the current page due to the load balancer
export const API_ENDPOINT = import.meta.env.DEV ? "http://127.0.0.1:8080" : window.location.origin

export const USERNAME_REGEX = /^[a-zA-Z0-9\-\_\.]{1,25}$/
export const NAME_REGEX = /^[a-zA-Z\s]{1,35}$/
export const PASSWORD_REGEX = /^.{8,200}$/
export const COMMENT_REGEX = /^[a-zA-Z0-9\s.,!?'"()-]{0,150}$/
export const ENTRY_TITLE_REGEX = /^[a-zA-Z0-9+\-/()\s]{1,20}$/
export const RESET_TOKEN_REGEX = /^[a-zA-Z0-9]{4}$/

const BANNED_PASSWORD_PATTERNS = [
    /^123456.*$/,   // Matches any password starting with "123456"
    /^password.*$/, // Matches any password starting with "password"
    /^qwerty.*$/,   // Matches any password starting with "qwerty"
    /^[0-9]*$/,     // Matches any password containing only numbers
    /^[a-z]*$/,     // Matches any password containing only lowercase letters
    /^[A-Z]*$/,     // Matches any password containing only uppercase letters
]

export const isPasswordBanned = (password: string): boolean => {
    for (const pattern of BANNED_PASSWORD_PATTERNS) {
        if (pattern.test(password)) {
            return true
        }
    }

    return false
}