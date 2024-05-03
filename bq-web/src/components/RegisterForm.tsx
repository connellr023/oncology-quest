import useValidateUsername from "../hooks/useValidateUsername"
import useValidateName from "../hooks/useValidateName"
import useValidateEmail from "../hooks/useValidateEmail"
import useValidateConfirmationPassword from "../hooks/useValidateConfirmationPassword"

const RegisterForm = () => {
  const { username, setUsername, usernameError } = useValidateUsername()
  const { name, setName, nameError } = useValidateName()
  const { email, setEmail, emailError } = useValidateEmail()
  const {
    password,
    setPassword,
    passwordError,
    confirmPassword,
    setConfirmPassword,
    confirmPasswordError
  } = useValidateConfirmationPassword()

  const handleSubmit = (e: Event) => {
    e.preventDefault()

    const isError = (usernameError || nameError || emailError || passwordError || confirmPasswordError)

    if (!isError) {
      // make request to register user
    }
  }

  return (
    <>
      <h3>Register</h3>
      <form onSubmit={handleSubmit}>
        <input
          type="text"
          placeholder="Username"
          value={username}
          onInput={(e) => setUsername(e.currentTarget.value)}
        />
        <input
          type="text"
          placeholder="Name"
          value={name}
          onInput={(e) => setName(e.currentTarget.value)}
        />
        <input
          type="email"
          placeholder="Email"
          value={email}
          onInput={(e) => setEmail(e.currentTarget.value)}
        />
        <input
          type="password"
          placeholder="Password"
          value={password}
          onInput={(e) => setPassword(e.currentTarget.value)}
        />
        <input
          type="password"
          placeholder="Confirm Password"
          value={confirmPassword}
          onInput={(e) => setConfirmPassword(e.currentTarget.value)}
        />
        <button type="submit">Register</button>
        <div>
          {usernameError.current && <p>{usernameError.current}</p>}
          {nameError.current && <p>{nameError.current}</p>}
          {emailError.current && <p>{emailError.current}</p>}
          {passwordError.current && <p>{passwordError.current}</p>}
          {confirmPasswordError.current && <p>{confirmPasswordError.current}</p>}
        </div>
      </form>
    </>
  )
}

export default RegisterForm