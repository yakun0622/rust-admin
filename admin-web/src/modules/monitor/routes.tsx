import type { RouteObject } from "react-router-dom";
import { MonitorPage } from "./pages/MonitorPage";

export const monitorRoutes: RouteObject[] = [
  { path: "monitor/online", element: <MonitorPage title="在线用户" kind="online" /> },
  { path: "monitor/job", element: <MonitorPage title="定时任务" kind="job" /> },
  { path: "monitor/datasource", element: <MonitorPage title="数据监控" kind="datasource" /> },
  { path: "monitor/server", element: <MonitorPage title="服务监控" kind="server" /> },
  { path: "monitor/cache", element: <MonitorPage title="缓存监控" kind="cache" /> },
  { path: "monitor/cache-list", element: <MonitorPage title="缓存列表" kind="cache-list" /> }
];
