import type { ReactNode } from "react";
import { BrowserRouter, Navigate, type RouteObject, useRoutes } from "react-router-dom";
import { AdminLayout } from "../layouts/AdminLayout";
import { getAccessToken } from "../../core/auth/token";
import { aiRoutes } from "../../modules/ai/routes";
import { authRoutes } from "../../modules/auth/routes";
import { dashboardRoutes } from "../../modules/dashboard/routes";
import { logRoutes } from "../../modules/log/routes";
import { monitorRoutes } from "../../modules/monitor/routes";
import { systemRoutes } from "../../modules/system/routes";

function RequireAuth({ children }: { children: ReactNode }) {
  const token = getAccessToken();
  if (!token) {
    return <Navigate to="/login" replace />;
  }
  return children;
}

const routes: RouteObject[] = [
  ...authRoutes,
  {
    path: "/",
    element: (
      <RequireAuth>
        <AdminLayout />
      </RequireAuth>
    ),
    children: [
      { index: true, element: <Navigate to="/dashboard" replace /> },
      ...dashboardRoutes,
      ...systemRoutes,
      ...logRoutes,
      ...monitorRoutes,
      ...aiRoutes
    ]
  },
  { path: "*", element: <Navigate to="/dashboard" replace /> }
];

function RouterView() {
  return useRoutes(routes);
}

export function AppRouter() {
  return (
    <BrowserRouter>
      <RouterView />
    </BrowserRouter>
  );
}
