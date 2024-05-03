import Router from "preact-router"
import useApiConnection from "./hooks/useApiConnection"

const App = () => {
  let { connectionError, session, loading } = useApiConnection()

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
      <h1>hi</h1>
    </main>
  )
}

export default App