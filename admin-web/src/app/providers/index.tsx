import type { PropsWithChildren } from "react";
import { createContext, useContext, useEffect, useMemo, useState } from "react";
import { ConfigProvider, theme } from "antd";
import type { ThemeConfig } from "antd";

export type AppThemeMode = "light" | "dark" | "tech";

type AppThemeContextValue = {
  themeMode: AppThemeMode;
  setThemeMode: (mode: AppThemeMode) => void;
};

const THEME_STORAGE_KEY = "rust_admin_theme_mode";
const themeClassMap: Record<AppThemeMode, string> = {
  light: "app-theme-light",
  dark: "app-theme-dark",
  tech: "app-theme-tech"
};

const AppThemeContext = createContext<AppThemeContextValue | null>(null);

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

  return (
    <AppThemeContext.Provider value={contextValue}>
      <ConfigProvider theme={antdTheme}>{children}</ConfigProvider>
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
