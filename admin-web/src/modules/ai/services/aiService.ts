import { http } from "../../../core/request/http";
import type { ApiResponse } from "../../../shared/types/api";

export type AiSessionItem = {
  id: number;
  title: string;
  status: string;
  last_active_at: number;
};

export type AiMessageItem = {
  id: number;
  session_id: number;
  role: "user" | "assistant" | string;
  content: string;
  created_at: number;
};

type SessionListResult = {
  total: number;
  items: AiSessionItem[];
};

type MessageListResult = {
  session_id: number;
  total: number;
  items: AiMessageItem[];
};

type SendMessageResult = {
  session_id: number;
  user_message: AiMessageItem;
  assistant_message: AiMessageItem;
};

function unwrapResponse<T>(response: ApiResponse<T>, fallbackMessage: string): T {
  if (response.code !== 200 || !response.data) {
    throw new Error(response.message || fallbackMessage);
  }
  return response.data;
}

export async function getAiSessions(): Promise<SessionListResult> {
  const res = await http.get<ApiResponse<SessionListResult>>("/ai/sessions");
  return unwrapResponse(res.data, "加载会话失败");
}

export async function createAiSession(title?: string): Promise<AiSessionItem> {
  const res = await http.post<ApiResponse<AiSessionItem>>("/ai/sessions", { title });
  return unwrapResponse(res.data, "创建会话失败");
}

export async function getAiMessages(sessionId: number): Promise<MessageListResult> {
  const res = await http.get<ApiResponse<MessageListResult>>(`/ai/sessions/${sessionId}/messages`);
  return unwrapResponse(res.data, "加载消息失败");
}

export async function sendAiMessage(
  sessionId: number,
  content: string
): Promise<SendMessageResult> {
  const res = await http.post<ApiResponse<SendMessageResult>>(
    `/ai/sessions/${sessionId}/messages`,
    { content }
  );
  return unwrapResponse(res.data, "发送消息失败");
}
