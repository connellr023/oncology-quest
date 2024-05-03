import useApiConnection from "../hooks/useApiConnection"
import SessionContext from "../contexts/SessionContext"
import AdminDashboard from "./AdminDashboard"
import Dashboard from "./Dashboard"
import NoSession from "./NoSession"

const App = () => {
  let { connectionError, loading, session, setSession } = useApiConnection()

  if (loading) {
    return (
      <main>
        <h1>Loading...</h1>
      </main>
    )
  }

  if (connectionError) {
    return (
      <main>
        <h1>Connection Error</h1>
      </main>
    )
  }

  return (
    <main>
      <SessionContext.Provider value={{ session, setSession }}>
        {session ? (
          session.user.isAdmin ? (
            <AdminDashboard />
          ) : (
            <Dashboard />
          )
        ) : (
          <NoSession />
        )}
      </SessionContext.Provider>
    </main>
  )
}

export default App