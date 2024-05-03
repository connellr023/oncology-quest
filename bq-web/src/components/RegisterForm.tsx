import { useState } from "preact/hooks"

const RegisterForm = () => {
  const [username, setUsername] = useState("")
  const [email, setEmail] = useState("")
  const [password, setPassword] = useState("")
  const [confirmPassword, setConfirmPassword] = useState("")

  const handleSubmit = (e: Event) => {
    e.preventDefault()
    console.log("Register", { username, email, password, confirmPassword })
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
      </form>
    </>
  )
}

export default RegisterForm