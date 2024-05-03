import { createContext } from "preact";
import { UserSession } from "../models/user";

export type SessionContextType = { session: UserSession | null, setSession: (session: UserSession | null) => void } | null;
const SessionContext = createContext<SessionContextType>(null);

export default SessionContext;