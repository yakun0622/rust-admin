import type { RouteObject } from "react-router-dom";
import { LogPage } from "./pages/LogPage";

export const logRoutes: RouteObject[] = [
  { path: "log/oper", element: <LogPage title="操作日志" type="oper" /> },
  { path: "log/login", element: <LogPage title="登录日志" type="login" /> }
];
