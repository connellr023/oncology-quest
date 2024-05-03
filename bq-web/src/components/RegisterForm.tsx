import { useState } from "preact/hooks"
import { API_ENDPOINT } from "../utility"
import useValidateUsername from "../hooks/useValidateUsername"
import useValidateName from "../hooks/useValidateName"
import useValidateEmail from "../hooks/useValidateEmail"
import useValidateConfirmationPassword from "../hooks/useValidateConfirmationPassword"

const RegisterForm = () => {
  const [message, setMessage] = useState("")
  const [serverError, setServerError] = useState("")
  //const [errors, setErrors] = useState({});
  
  const { username, usernameError } = useValidateUsername()
  const { name, nameError } = useValidateName()
  const { email, emailError } = useValidateEmail()
  const {
    password,
    confirmPassword,
    confirmPasswordError
  } = useValidateConfirmationPassword()

  const createUser = async () => {
    try {
      const response = await fetch(`${API_ENDPOINT}/api/user/create`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify({
          username: username.current,
          name: name.current,
          email: email.current,
          password: password.current
        })
      })

      if (response.status === 201) {
        setMessage("User created successfully.")
      }
      else {
        setServerError(`Received status code ${response.status}`)
      }
    }
    catch (error) {
      setServerError("Server error. Please try again later.")
    }
  }

  // const validateForm = () => {
  //   let newErrors: { email?: string, password?: string, username?: string, name?: string } = {}

  //   if (emailError.current) {
  //     newErrors.email = emailError.current
  //   }

  //   if (confirmPassword.current) {
  //     newErrors.password = confirmPassword.current
  //   }

  //   if (usernameError.current) {
  //     newErrors.username = usernameError.current
  //   }
  //   if (nameError.current) {
  //     newErrors.name = nameError.current
  //   }

  //   setErrors(newErrors)
  //   return Object.keys(errors).length === 0
  // }

  const handleSubmit = (e: Event) => {
    e.preventDefault()
    createUser()
  }

  return (
    <>
      <h3>Register</h3>
      <form onSubmit={handleSubmit}>
        <input
          type="text"
          placeholder="Username"
          value={username.current}
          onInput={(e) => username.current = e.currentTarget.value}
        />
        <input
          type="text"
          placeholder="Name"
          value={name.current}
          onInput={(e) => name.current = e.currentTarget.value}
        />
        <input
          type="email"
          placeholder="Email"
          value={email.current}
          onInput={(e) => email.current = e.currentTarget.value}
        />
        <input
          type="password"
          placeholder="Password"
          value={password.current}
          onInput={(e) => password.current = e.currentTarget.value}
        />
        <input
          type="password"
          placeholder="Confirm Password"
          value={confirmPassword.current}
          onInput={(e) => confirmPassword.current = e.currentTarget.value}
        />
        <button type="submit">Register</button>
        {message && <p>{message}</p>}
        <div>
          {serverError && <p>{serverError}</p>}
          {usernameError.current && <p>{usernameError.current}</p>}
          {nameError.current && <p>{nameError.current}</p>}
          {emailError.current && <p>{emailError.current}</p>}
          {confirmPasswordError.current && <p>{confirmPasswordError.current}</p>}
        </div>
      </form>
    </>
  )
}

export default RegisterForm