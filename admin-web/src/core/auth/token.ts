import { TOKEN_STORAGE_KEY } from "../config";

export const AUTH_TOKEN_CHANGED_EVENT = "rust-admin:auth-token-changed";

function emitTokenChanged() {
  if (typeof window !== "undefined") {
    window.dispatchEvent(new Event(AUTH_TOKEN_CHANGED_EVENT));
  }
}

export function getAccessToken() {
  return localStorage.getItem(TOKEN_STORAGE_KEY);
}

export function setAccessToken(token: string) {
  localStorage.setItem(TOKEN_STORAGE_KEY, token);
  emitTokenChanged();
}

export function clearAccessToken() {
  localStorage.removeItem(TOKEN_STORAGE_KEY);
  emitTokenChanged();
}
