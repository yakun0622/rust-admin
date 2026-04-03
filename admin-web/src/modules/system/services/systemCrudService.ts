import { http } from "../../../core/request/http";
import type { ApiResponse } from "../../../shared/types/api";

export type SystemCrudRecord = {
  id: number;
  [key: string]: unknown;
};

type SystemCrudListVo = {
  total: number;
  items: Array<Record<string, unknown>>;
};

type SystemCrudRecordVo = {
  item: Record<string, unknown>;
};

function normalizeRecord(raw: Record<string, unknown>): SystemCrudRecord {
  const id = Number(raw.id ?? 0);
  return {
    ...raw,
    id: Number.isFinite(id) ? id : 0
  };
}

function unwrapResponse<T>(response: ApiResponse<T>, fallbackMessage: string): T {
  if (response.code !== 200 || !response.data) {
    throw new Error(response.message || fallbackMessage);
  }
  return response.data;
}

export async function listSystemRecords(
  resource: string,
  keyword?: string
): Promise<{ total: number; items: SystemCrudRecord[] }> {
  const params: Record<string, string> = {};
  const trimmed = keyword?.trim();
  if (trimmed) {
    params.keyword = trimmed;
  }

  const res = await http.get<ApiResponse<SystemCrudListVo>>(`/system/${resource}`, { params });
  const data = unwrapResponse(res.data, "查询列表失败");
  return {
    total: data.total,
    items: data.items.map(normalizeRecord)
  };
}

export async function createSystemRecord(
  resource: string,
  payload: Record<string, unknown>
): Promise<SystemCrudRecord> {
  const res = await http.post<ApiResponse<SystemCrudRecordVo>>(`/system/${resource}`, payload);
  const data = unwrapResponse(res.data, "新增失败");
  return normalizeRecord(data.item);
}

export async function updateSystemRecord(
  resource: string,
  id: number,
  payload: Record<string, unknown>
): Promise<SystemCrudRecord> {
  const res = await http.put<ApiResponse<SystemCrudRecordVo>>(`/system/${resource}/${id}`, payload);
  const data = unwrapResponse(res.data, "更新失败");
  return normalizeRecord(data.item);
}

export async function deleteSystemRecord(resource: string, id: number): Promise<void> {
  const res = await http.delete<ApiResponse<{ id: number; deleted: boolean }>>(
    `/system/${resource}/${id}`
  );
  unwrapResponse(res.data, "删除失败");
}
