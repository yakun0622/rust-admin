import axios from "axios";
import { API_BASE_URL } from "../config";
import { clearAccessToken, getAccessToken } from "../auth/token";

const http = axios.create({
  baseURL: API_BASE_URL,
  timeout: 10000
});

http.interceptors.request.use((config) => {
  const token = getAccessToken();
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

http.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error?.response?.status === 401) {
      clearAccessToken();
      if (window.location.pathname !== "/login") {
        window.location.href = "/login";
      }
    }

    const message =
      error?.response?.data?.message || error?.response?.data?.error || error?.message || "请求失败";
    return Promise.reject(new Error(message));
  }
);

export { http };
