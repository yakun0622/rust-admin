import type { RouteObject } from "react-router-dom";
import { AiChatPage } from "./pages/AiChatPage";

export const aiRoutes: RouteObject[] = [
  {
    path: "ai/chat",
    element: <AiChatPage />
  }
];
