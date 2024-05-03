import { createContext } from "preact";
import { UserSession } from "../models/user";

type SessionContextType = { session: UserSession | null, setSession: (session: UserSession | null) => void } | null;
const SessionContext = createContext<SessionContextType>(null);

export default SessionContext;