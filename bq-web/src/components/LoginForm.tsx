import { useState } from "preact/hooks"

const LoginForm = () => {
  const [username, setUsername] = useState("")
  const [password, setPassword] = useState("")

  const handleSubmit = (e: Event) => {
    e.preventDefault()
    console.log("Login", { username, password })
  }

  return (
    <>
      <h3>Login</h3>
      <form onSubmit={handleSubmit}>
        <input
          type="text"
          placeholder="Username"
          value={username}
          onInput={(e) => setUsername(e.currentTarget.value)}
        />
        <input
          type="password"
          placeholder="Password"
          value={password}
          onInput={(e) => setPassword(e.currentTarget.value)}
        />
        <button type="submit">Login</button>
      </form>
    </>
  )
}

export default LoginForm