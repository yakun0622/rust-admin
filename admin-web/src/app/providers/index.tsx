import type { PropsWithChildren } from "react";
import { createContext, useCallback, useContext, useEffect, useMemo, useState } from "react";
import { ConfigProvider, theme } from "antd";
import type { ThemeConfig } from "antd";
import {
  AUTH_TOKEN_CHANGED_EVENT,
  clearAccessToken,
  getAccessToken,
  setAccessToken
} from "../../core/auth/token";
import { hasPermission } from "../../core/permission";
import { getProfile, type AuthProfileVo } from "../../modules/auth/services/authService";

export type AppThemeMode = "light" | "dark" | "tech";

type AppThemeContextValue = {
  themeMode: AppThemeMode;
  setThemeMode: (mode: AppThemeMode) => void;
};

type AppAuthContextValue = {
  profile: AuthProfileVo | null;
  permissions: string[];
  menus: AuthProfileVo["menus"];
  loading: boolean;
  error: string | null;
  applyLoginToken: (token: string) => Promise<void>;
  reloadProfile: () => Promise<void>;
  logout: () => void;
  hasPerm: (requiredPerm?: string) => boolean;
};

const THEME_STORAGE_KEY = "rust_admin_theme_mode";
const themeClassMap: Record<AppThemeMode, string> = {
  light: "app-theme-light",
  dark: "app-theme-dark",
  tech: "app-theme-tech"
};

const AppThemeContext = createContext<AppThemeContextValue | null>(null);
const AppAuthContext = createContext<AppAuthContextValue | null>(null);

export const APP_THEME_OPTIONS: Array<{ label: string; value: AppThemeMode }> = [
  { label: "明亮", value: "light" },
  { label: "暗黑", value: "dark" },
  { label: "科技", value: "tech" }
];

function isAppThemeMode(value: string | null): value is AppThemeMode {
  return value === "light" || value === "dark" || value === "tech";
}

function resolveInitialThemeMode(): AppThemeMode {
  if (typeof window === "undefined") {
    return "tech";
  }
  const cached = window.localStorage.getItem(THEME_STORAGE_KEY);
  return isAppThemeMode(cached) ? cached : "tech";
}

function buildAntdThemeConfig(mode: AppThemeMode): ThemeConfig {
  if (mode === "light") {
    return {
      algorithm: theme.defaultAlgorithm,
      token: {
        colorPrimary: "#1677ff",
        borderRadius: 10
      }
    };
  }

  if (mode === "dark") {
    return {
      algorithm: theme.darkAlgorithm,
      token: {
        colorPrimary: "#4c9dff",
        borderRadius: 10
      }
    };
  }

  return {
    algorithm: theme.defaultAlgorithm,
    token: {
      colorPrimary: "#2f9cff",
      borderRadius: 10
    }
  };
}

export function AppProviders({ children }: PropsWithChildren) {
  const [themeMode, setThemeMode] = useState<AppThemeMode>(() => resolveInitialThemeMode());
  const [profile, setProfile] = useState<AuthProfileVo | null>(null);
  const [authLoading, setAuthLoading] = useState(false);
  const [authError, setAuthError] = useState<string | null>(null);
  const antdTheme = useMemo(() => buildAntdThemeConfig(themeMode), [themeMode]);

  useEffect(() => {
    if (typeof document === "undefined") {
      return;
    }
    document.body.classList.remove(
      themeClassMap.light,
      themeClassMap.dark,
      themeClassMap.tech
    );
    document.body.classList.add(themeClassMap[themeMode]);
    document.body.setAttribute("data-theme", themeMode);
    window.localStorage.setItem(THEME_STORAGE_KEY, themeMode);
  }, [themeMode]);

  const contextValue = useMemo<AppThemeContextValue>(
    () => ({
      themeMode,
      setThemeMode
    }),
    [themeMode]
  );

  const reloadProfile = useCallback(async () => {
    const token = getAccessToken();
    if (!token) {
      setProfile(null);
      setAuthError(null);
      setAuthLoading(false);
      return;
    }

    setAuthLoading(true);
    try {
      const data = await getProfile();
      setProfile(data);
      setAuthError(null);
    } catch (error) {
      const message = error instanceof Error ? error.message : "加载用户权限失败";
      setAuthError(message);
      if (!getAccessToken()) {
        setProfile(null);
      }
    } finally {
      setAuthLoading(false);
    }
  }, []);

  const applyLoginToken = useCallback(
    async (token: string) => {
      setAccessToken(token);
      await reloadProfile();
    },
    [reloadProfile]
  );

  const logout = useCallback(() => {
    clearAccessToken();
    setProfile(null);
    setAuthError(null);
    setAuthLoading(false);
  }, []);

  useEffect(() => {
    void reloadProfile();
  }, [reloadProfile]);

  useEffect(() => {
    function handleTokenChanged() {
      void reloadProfile();
    }

    window.addEventListener(AUTH_TOKEN_CHANGED_EVENT, handleTokenChanged);
    return () => {
      window.removeEventListener(AUTH_TOKEN_CHANGED_EVENT, handleTokenChanged);
    };
  }, [reloadProfile]);

  const authContextValue = useMemo<AppAuthContextValue>(() => {
    const permissions = profile?.permissions ?? [];
    return {
      profile,
      permissions,
      menus: profile?.menus ?? [],
      loading: authLoading,
      error: authError,
      applyLoginToken,
      reloadProfile,
      logout,
      hasPerm: (requiredPerm?: string) => hasPermission(permissions, requiredPerm || "")
    };
  }, [applyLoginToken, authError, authLoading, logout, profile, reloadProfile]);

  return (
    <AppThemeContext.Provider value={contextValue}>
      <AppAuthContext.Provider value={authContextValue}>
        <ConfigProvider theme={antdTheme}>{children}</ConfigProvider>
      </AppAuthContext.Provider>
    </AppThemeContext.Provider>
  );
}

export function useAppTheme(): AppThemeContextValue {
  const context = useContext(AppThemeContext);
  if (!context) {
    throw new Error("useAppTheme must be used within AppProviders");
  }
  return context;
}

export function useAuthSession(): AppAuthContextValue {
  const context = useContext(AppAuthContext);
  if (!context) {
    throw new Error("useAuthSession must be used within AppProviders");
  }
  return context;
}
