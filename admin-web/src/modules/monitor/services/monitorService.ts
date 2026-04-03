import { http } from "../../../core/request/http";
import type { ApiResponse } from "../../../shared/types/api";

export type OnlineUserItem = {
  id: number;
  username: string;
  ip: string;
  browser: string;
  os: string;
  login_at: number;
  last_active_at: number;
  status: string;
};

export type JobItem = {
  id: number;
  job_name: string;
  job_group: string;
  invoke_target: string;
  cron_expression: string;
  status: string;
  remark: string;
  last_run_at: number | null;
  next_run_at: number | null;
};

export type JobUpsertPayload = {
  job_name: string;
  job_group: string;
  invoke_target: string;
  cron_expression: string;
  status?: string;
  remark?: string;
};

export type JobActionResult = {
  id: number;
  status: string;
  last_run_at: number | null;
  next_run_at: number | null;
  message: string;
};

export type DatasourceOverview = {
  database: string;
  mysql_url: string;
  max_connections: number;
  min_connections: number;
  ping_ok: boolean;
  ping_message: string;
};

export type ServerOverview = {
  app_name: string;
  env: string;
  uptime_secs: number;
  mysql_ok: boolean;
  redis_ok: boolean;
  now_millis: number;
};

export type CacheKeyItem = {
  key: string;
  data_type: string;
  ttl_secs: number;
  sample: string;
};

export type CacheSearchResult = {
  pattern: string;
  total: number;
  items: CacheKeyItem[];
};

export type CacheNamespaceItem = {
  namespace: string;
  key_count: number;
  example_key: string;
};

export type CacheNamespaceList = {
  total: number;
  items: CacheNamespaceItem[];
};

type ListResult<T> = {
  total: number;
  items: T[];
};

function unwrapResponse<T>(response: ApiResponse<T>, fallbackMessage: string): T {
  if (response.code !== 200 || !response.data) {
    throw new Error(response.message || fallbackMessage);
  }
  return response.data;
}

export async function getOnlineUsers(keyword?: string): Promise<ListResult<OnlineUserItem>> {
  const params: Record<string, string> = {};
  const trimmed = keyword?.trim();
  if (trimmed) {
    params.keyword = trimmed;
  }
  const res = await http.get<ApiResponse<ListResult<OnlineUserItem>>>("/monitor/online", { params });
  return unwrapResponse(res.data, "加载在线用户失败");
}

export async function getJobs(keyword?: string): Promise<ListResult<JobItem>> {
  const params: Record<string, string> = {};
  const trimmed = keyword?.trim();
  if (trimmed) {
    params.keyword = trimmed;
  }
  const res = await http.get<ApiResponse<ListResult<JobItem>>>("/monitor/job", { params });
  return unwrapResponse(res.data, "加载任务列表失败");
}

export async function createJob(payload: JobUpsertPayload): Promise<JobItem> {
  const res = await http.post<ApiResponse<JobItem>>("/monitor/job", payload);
  return unwrapResponse(res.data, "新增任务失败");
}

export async function updateJob(id: number, payload: JobUpsertPayload): Promise<JobItem> {
  const res = await http.put<ApiResponse<JobItem>>(`/monitor/job/${id}`, payload);
  return unwrapResponse(res.data, "更新任务失败");
}

export async function deleteJob(id: number): Promise<JobActionResult> {
  const res = await http.delete<ApiResponse<JobActionResult>>(`/monitor/job/${id}`);
  return unwrapResponse(res.data, "删除任务失败");
}

export async function runJob(id: number): Promise<JobActionResult> {
  const res = await http.post<ApiResponse<JobActionResult>>(`/monitor/job/${id}/run`);
  return unwrapResponse(res.data, "执行任务失败");
}

export async function pauseJob(id: number): Promise<JobActionResult> {
  const res = await http.post<ApiResponse<JobActionResult>>(`/monitor/job/${id}/pause`);
  return unwrapResponse(res.data, "暂停任务失败");
}

export async function resumeJob(id: number): Promise<JobActionResult> {
  const res = await http.post<ApiResponse<JobActionResult>>(`/monitor/job/${id}/resume`);
  return unwrapResponse(res.data, "恢复任务失败");
}

export async function getDatasourceOverview(): Promise<DatasourceOverview> {
  const res = await http.get<ApiResponse<DatasourceOverview>>("/monitor/datasource");
  return unwrapResponse(res.data, "加载数据源监控失败");
}

export async function getServerOverview(): Promise<ServerOverview> {
  const res = await http.get<ApiResponse<ServerOverview>>("/monitor/server");
  return unwrapResponse(res.data, "加载服务监控失败");
}

export async function searchCache(keyword?: string, limit = 50): Promise<CacheSearchResult> {
  const params: Record<string, string | number> = { limit };
  const trimmed = keyword?.trim();
  if (trimmed) {
    params.keyword = trimmed;
  }
  const res = await http.get<ApiResponse<CacheSearchResult>>("/monitor/cache", { params });
  return unwrapResponse(res.data, "缓存检索失败");
}

export async function getCacheNamespaces(): Promise<CacheNamespaceList> {
  const res = await http.get<ApiResponse<CacheNamespaceList>>("/monitor/cache-list");
  return unwrapResponse(res.data, "加载缓存列表失败");
}
