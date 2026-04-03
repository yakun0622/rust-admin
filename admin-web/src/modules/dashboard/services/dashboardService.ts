import { http } from "../../../core/request/http";
import type { ApiResponse } from "../../../shared/types/api";

export type DashboardOverviewVo = {
  admin_total: number;
  online_users: number;
  role_total: number;
  menu_total: number;
  today_logins: number;
  today_errors: number;
  login_trend: number[];
  action_trend: number[];
};

export async function getDashboardOverview(): Promise<DashboardOverviewVo> {
  const res = await http.get<ApiResponse<DashboardOverviewVo>>("/dashboard/overview");
  if (res.data.code !== 200 || !res.data.data) {
    throw new Error(res.data.message || "加载看板数据失败");
  }
  return res.data.data;
}
