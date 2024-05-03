import { useState } from "preact/hooks"
import LoginForm from "./LoginForm"
import RegisterForm from "./RegisterForm"

enum Views {
  SELECT,
  LOGIN,
  REGISTER
}

const NoSession = () => {
  const [view, setView] = useState(Views.SELECT)

  const renderView = () => {
    switch (view) {
      case Views.SELECT:
        return (
          <>
            <button onClick={() => setView(Views.LOGIN)}>Login</button>
            <button onClick={() => setView(Views.REGISTER)}>Register</button>
            <button disabled>Reset Password</button>
          </>
        )
      case Views.LOGIN:
        return (
          <>
            <LoginForm />
            <button onClick={() => setView(Views.SELECT)}>Back</button> 
          </>
        )
      case Views.REGISTER:
        return (
          <>
            <RegisterForm />
            <button onClick={() => setView(Views.SELECT)}>Back</button> 
          </>
        )
    }
  }

  return (
    <div>
      <h1>bq</h1>
      <div className="form-container">
        {renderView()}
      </div>
    </div>
  )
}

export default NoSession