import { http } from "../../../core/request/http";
import type { ApiResponse } from "../../../shared/types/api";

export type OperLogItem = {
  id: number;
  module: string;
  business_type: string;
  request_method: string;
  oper_name: string;
  ip: string;
  status: string;
  duration_ms: number;
  oper_at: number;
};

export type LoginLogItem = {
  id: number;
  username: string;
  login_type: string;
  ip: string;
  status: string;
  message: string;
  login_at: number;
};

type LogListResponse<T> = {
  total: number;
  items: T[];
};

function unwrapResponse<T>(response: ApiResponse<T>, fallbackMessage: string): T {
  if (response.code !== 200 || !response.data) {
    throw new Error(response.message || fallbackMessage);
  }
  return response.data;
}

export async function getOperLogs(keyword?: string): Promise<LogListResponse<OperLogItem>> {
  const params: Record<string, string> = {};
  const trimmed = keyword?.trim();
  if (trimmed) {
    params.keyword = trimmed;
  }
  const res = await http.get<ApiResponse<LogListResponse<OperLogItem>>>("/log/oper", { params });
  return unwrapResponse(res.data, "加载操作日志失败");
}

export async function getLoginLogs(keyword?: string): Promise<LogListResponse<LoginLogItem>> {
  const params: Record<string, string> = {};
  const trimmed = keyword?.trim();
  if (trimmed) {
    params.keyword = trimmed;
  }
  const res = await http.get<ApiResponse<LogListResponse<LoginLogItem>>>("/log/login", { params });
  return unwrapResponse(res.data, "加载登录日志失败");
}
