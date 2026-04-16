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

export type AuthMenuVo = {
  id: number;
  parent_id: number;
  menu_type: number;
  name: string;
  route_name: string;
  path: string;
  component: string;
  permission: string;
  icon: string;
  order_num: number;
  status: string;
  visible: string;
  children?: AuthMenuVo[];
};

export type AuthProfileVo = {
  user: {
    user_id: number;
    username: string;
    nickname: string;
  };
  permissions: string[];
  menus: AuthMenuVo[];
};

export async function login(payload: LoginReq): Promise<LoginVo> {
  const res = await http.post<ApiResponse<LoginVo>>("/system/auth/login", payload);
  if (res.data.code !== 200 || !res.data.data) {
    throw new Error(res.data.message || "登录失败");
  }
  return res.data.data;
}

export async function getProfile(): Promise<AuthProfileVo> {
  const res = await http.get<ApiResponse<AuthProfileVo>>("/system/auth/profile");
  if (res.data.code !== 200 || !res.data.data) {
    throw new Error(res.data.message || "加载用户权限失败");
  }
  return res.data.data;
}
