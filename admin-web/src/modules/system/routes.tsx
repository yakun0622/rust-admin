import type { RouteObject } from "react-router-dom";
import { SystemPage } from "./pages/SystemPage";
import { systemCrudConfigs } from "./configs/crudConfigs";

export const systemRoutes: RouteObject[] = [
  { path: "system/user", element: <SystemPage config={systemCrudConfigs.user} /> },
  { path: "system/role", element: <SystemPage config={systemCrudConfigs.role} /> },
  { path: "system/menu", element: <SystemPage config={systemCrudConfigs.menu} /> },
  { path: "system/dept", element: <SystemPage config={systemCrudConfigs.dept} /> },
  { path: "system/post", element: <SystemPage config={systemCrudConfigs.post} /> },
  { path: "system/dict", element: <SystemPage config={systemCrudConfigs.dict} /> },
  { path: "system/config", element: <SystemPage config={systemCrudConfigs.config} /> },
  { path: "system/notice", element: <SystemPage config={systemCrudConfigs.notice} /> }
];
