import type { ReactNode } from "react";
import { createContext, useContext, useEffect, useMemo, useState } from "react";
import { useLocation, useNavigate } from "react-router-dom";

export type TabItem = {
  key: string;
  label: string;
  icon?: string;
  closable?: boolean;
};

type TabsContextValue = {
  tabs: TabItem[];
  activeKey: string;
  addTab: (tab: TabItem) => void;
  removeTab: (key: string) => void;
  closeOthers: (key: string) => void;
  closeAll: () => void;
  closeLeft: (key: string) => void;
  closeRight: (key: string) => void;
  setActiveKey: (key: string) => void;
};

const TabsContext = createContext<TabsContextValue | null>(null);

const DASHBOARD_KEY = "/dashboard";
const FIXED_TABS: TabItem[] = [
  { key: DASHBOARD_KEY, label: "首页", closable: false }
];

export function TabsProvider({ children }: { children: ReactNode }) {
  const [tabs, setTabs] = useState<TabItem[]>(() => {
    const cached = sessionStorage.getItem("admin_tabs");
    return cached ? JSON.parse(cached) : FIXED_TABS;
  });
  const [activeKey, setActiveKey] = useState<string>(DASHBOARD_KEY);
  const navigate = useNavigate();
  const location = useLocation();

  // Persist tabs
  useEffect(() => {
    sessionStorage.setItem("admin_tabs", JSON.stringify(tabs));
  }, [tabs]);

  // Sync activeKey with URL
  useEffect(() => {
    setActiveKey(location.pathname);
  }, [location.pathname]);

  const addTab = (tab: TabItem) => {
    setTabs((prev) => {
      const exists = prev.find((t) => t.key === tab.key);
      if (exists) {
        return prev;
      }
      return [...prev, { ...tab, closable: tab.closable ?? true }];
    });
  };

  const removeTab = (targetKey: string) => {
    let newActiveKey = activeKey;
    let lastIndex = -1;
    tabs.forEach((item, i) => {
      if (item.key === targetKey) {
        lastIndex = i - 1;
      }
    });

    const newTabs = tabs.filter((item) => item.key !== targetKey);
    if (newTabs.length && activeKey === targetKey) {
      if (lastIndex >= 0) {
        newActiveKey = newTabs[lastIndex].key;
      } else {
        newActiveKey = newTabs[0].key;
      }
    }

    setTabs(newTabs);
    if (activeKey === targetKey) {
      navigate(newActiveKey);
    }
  };

  const closeOthers = (key: string) => {
    const newTabs = tabs.filter((t) => !t.closable || t.key === key);
    setTabs(newTabs);
    if (activeKey !== key) {
      navigate(key);
    }
  };

  const closeAll = () => {
    const newTabs = tabs.filter((t) => !t.closable);
    setTabs(newTabs);
    navigate(DASHBOARD_KEY);
  };

  const closeLeft = (key: string) => {
    const index = tabs.findIndex(t => t.key === key);
    const newTabs = tabs.filter((t, i) => i >= index || !t.closable);
    setTabs(newTabs);
    if (tabs.findIndex(t => t.key === activeKey) < index) {
      navigate(key);
    }
  };

  const closeRight = (key: string) => {
    const index = tabs.findIndex(t => t.key === key);
    const newTabs = tabs.filter((t, i) => i <= index || !t.closable);
    setTabs(newTabs);
    if (tabs.findIndex(t => t.key === activeKey) > index) {
      navigate(key);
    }
  };

  const value = useMemo(
    () => ({
      tabs,
      activeKey,
      addTab,
      removeTab,
      closeOthers,
      closeAll,
      closeLeft,
      closeRight,
      setActiveKey
    }),
    [tabs, activeKey]
  );

  return <TabsContext.Provider value={value}>{children}</TabsContext.Provider>;
}

export function useTabs() {
  const context = useContext(TabsContext);
  if (!context) {
    throw new Error("useTabs must be used within a TabsProvider");
  }
  return context;
}
