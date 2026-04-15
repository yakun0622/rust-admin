import { http } from "../../../core/request/http";
import type { ApiResponse } from "../../../shared/types/api";

export type LoginReq = {
  username: string;
  password: string;
};

export type LoginVo = {
  access_token: string;
  token_type: string;
  expires_in: number;
  username: string;
  nickname: string;
};

export async function login(payload: LoginReq): Promise<LoginVo> {
  const res = await http.post<ApiResponse<LoginVo>>("/system/auth/login", payload);
  if (res.data.code !== 200 || !res.data.data) {
    throw new Error(res.data.message || "登录失败");
  }
  return res.data.data;
}
