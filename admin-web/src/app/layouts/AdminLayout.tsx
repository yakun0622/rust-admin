import {
  ApartmentOutlined,
  BgColorsOutlined,
  BellOutlined,
  BookOutlined,
  CloseCircleOutlined,
  CloseOutlined,
  CloudServerOutlined,
  DashboardOutlined,
  DatabaseOutlined,
  EllipsisOutlined,
  FileTextOutlined,
  FundProjectionScreenOutlined,
  HddOutlined,
  IdcardOutlined,
  LoginOutlined,
  LogoutOutlined,
  MenuOutlined,
  MessageOutlined,
  MonitorOutlined,
  NotificationOutlined,
  ProfileOutlined,
  QuestionCircleOutlined,
  ReloadOutlined,
  SearchOutlined,
  SettingOutlined,
  TeamOutlined,
  ToolOutlined,
  UserOutlined,
  ArrowLeftOutlined,
  ArrowRightOutlined,
  MinusCircleOutlined
} from "@ant-design/icons";
import {
  Avatar,
  Badge,
  Button,
  Dropdown,
  Input,
  Layout,
  Menu,
  Select,
  Space,
  Tabs,
  Typography,
  message
} from "antd";
import type { MenuProps } from "antd";
import type { ReactNode } from "react";
import { useEffect, useMemo, useState, useRef } from "react";
import { useLocation, useNavigate, useOutlet } from "react-router-dom";
import { hasPermission } from "../../core/permission";
import type { AuthMenuVo } from "../../modules/auth/services/authService";
import { APP_THEME_OPTIONS, useAppTheme, useAuthSession, type AppThemeMode } from "../providers";
import { useTabs } from "../providers/TabsProvider";
import "./AdminLayout.css";
import "./AdminLayoutTabs.css";

const { Header, Sider, Content } = Layout;

type QuickAction = {
  key: string;
  label: string;
  icon: ReactNode;
  requiredPerm?: string;
};

type MenuPathMeta = {
  path: string;
  title: string;
  icon?: string;
  parentKeys: string[];
};

const iconMap: Record<string, ReactNode> = {
  robot: <MessageOutlined />,
  setting: <SettingOutlined />,
  user: <UserOutlined />,
  team: <TeamOutlined />,
  menu: <MenuOutlined />,
  apartment: <ApartmentOutlined />,
  idcard: <IdcardOutlined />,
  book: <BookOutlined />,
  tool: <ToolOutlined />,
  notification: <NotificationOutlined />,
  "file-text": <FileTextOutlined />,
  profile: <ProfileOutlined />,
  login: <LoginOutlined />,
  monitor: <MonitorOutlined />,
  "usergroup-add": <UserOutlined />,
  schedule: <FundProjectionScreenOutlined />,
  database: <DatabaseOutlined />,
  "cloud-server": <CloudServerOutlined />,
  hdd: <HddOutlined />,
  bars: <MenuOutlined />,
  dashboard: <DashboardOutlined />
};

const quickActions: QuickAction[] = [
  { key: "/system/user", label: "新增用户", icon: <UserOutlined />, requiredPerm: "system:user:create" },
  { key: "/system/config", label: "系统配置", icon: <SettingOutlined />, requiredPerm: "system:config:update" },
  { key: "/log/oper", label: "查看日志", icon: <FileTextOutlined />, requiredPerm: "log:oper:view" },
  { key: "/monitor/datasource", label: "数据备份", icon: <DatabaseOutlined />, requiredPerm: "monitor:datasource:view" }
];

function normalizePath(path?: string): string {
  if (!path) {
    return "";
  }
  const trimmed = path.trim();
  if (!trimmed || !trimmed.startsWith("/")) {
    return "";
  }
  return trimmed === "/" ? "/" : trimmed.replace(/\/+$/, "");
}

function resolveMenuKey(menu: AuthMenuVo): string {
  return normalizePath(menu.path) || `menu-${menu.id}`;
}

function resolveMenuIcon(icon?: string): ReactNode {
  if (!icon) {
    return <MenuOutlined />;
  }
  return iconMap[icon] || <MenuOutlined />;
}

function buildAntdMenuItems(nodes: AuthMenuVo[]): NonNullable<MenuProps["items"]> {
  return [...nodes]
    .sort((a, b) => (a.order_num - b.order_num) || (a.id - b.id))
    .map((node) => {
      const children = buildAntdMenuItems(
        (node.children || []).filter((child) => child.menu_type !== 3)
      );
      const key = resolveMenuKey(node);
      if (children.length > 0) {
        return {
          key,
          icon: resolveMenuIcon(node.icon),
          label: node.name,
          children
        };
      }

      return {
        key,
        icon: resolveMenuIcon(node.icon),
        label: node.name
      };
    });
}

function collectMenuPathMeta(
  nodes: AuthMenuVo[],
  parentKeys: string[] = []
): MenuPathMeta[] {
  const result: MenuPathMeta[] = [];
  const sortedNodes = [...nodes].sort((a, b) => (a.order_num - b.order_num) || (a.id - b.id));

  sortedNodes.forEach((node) => {
    const key = resolveMenuKey(node);
    const nodePath = normalizePath(node.path);
    const children = (node.children || []).filter((child) => child.menu_type !== 3);
    if (nodePath && node.menu_type === 2) {
      result.push({
        path: nodePath,
        title: node.name,
        icon: node.icon,
        parentKeys
      });
    }
    if (children.length > 0) {
      result.push(...collectMenuPathMeta(children, [...parentKeys, key]));
    }
  });

  return result;
}

function resolveMatchedPathMeta(pathname: string, pathMetaList: MenuPathMeta[]): MenuPathMeta | null {
  const normalizedPathname = normalizePath(pathname);
  if (!normalizedPathname) {
    return null;
  }

  const matched = pathMetaList
    .filter(
      (item) =>
        normalizedPathname === item.path || normalizedPathname.startsWith(`${item.path}/`)
    )
    .sort((a, b) => b.path.length - a.path.length);

  return matched[0] || null;
}

export function AdminLayout() {
  const navigate = useNavigate();
  const location = useLocation();
  const outlet = useOutlet();
  const [messageApi, contextHolder] = message.useMessage();
  const { themeMode, setThemeMode } = useAppTheme();
  const { profile, menus, permissions, loading: authLoading, error: authError, logout, hasPerm } = useAuthSession();
  const { tabs, activeKey, addTab, removeTab, closeOthers, closeAll, closeLeft, closeRight } = useTabs();
  
  const [openKeys, setOpenKeys] = useState<string[]>([]);
  // Cache components for keep-alive
  const [componentCache, setComponentCache] = useState<Record<string, ReactNode>>({});

  const menuItems = useMemo(() => buildAntdMenuItems(menus), [menus]);
  const pathMetaList = useMemo(() => collectMenuPathMeta(menus), [menus]);
  const matchedMeta = useMemo(
    () => resolveMatchedPathMeta(location.pathname, pathMetaList),
    [location.pathname, pathMetaList]
  );

  // Sync route outlet to cache
  useEffect(() => {
    if (outlet && location.pathname !== "/") {
      setComponentCache((prev) => ({
        ...prev,
        [location.pathname]: outlet
      }));
    }
  }, [location.pathname, outlet]);

  // Sync tabs with navigation
  useEffect(() => {
    if (matchedMeta) {
      addTab({
        key: matchedMeta.path,
        label: matchedMeta.title,
        icon: matchedMeta.icon
      });
    }
  }, [matchedMeta, addTab]);

  // Purge cache for closed tabs
  useEffect(() => {
    setComponentCache((prev) => {
      const keys = Object.keys(prev);
      const newCache = { ...prev };
      let changed = false;
      keys.forEach((key) => {
        if (!tabs.find((t) => t.key === key)) {
          delete newCache[key];
          changed = true;
        }
      });
      return changed ? newCache : prev;
    });
  }, [tabs]);

  useEffect(() => {
    if (!matchedMeta) {
      return;
    }
    if (matchedMeta.parentKeys.length === 0) {
      return;
    }
    setOpenKeys((prev) => {
      const merged = [...new Set([...matchedMeta.parentKeys, ...prev])];
      return merged.slice(0, 6);
    });
  }, [matchedMeta]);

  useEffect(() => {
    if (authError) {
      messageApi.warning(authError);
    }
  }, [authError, messageApi]);

  const currentTitle = matchedMeta?.title || "后台管理系统";
  const selectedKeys = matchedMeta ? [matchedMeta.path] : [];
  const visibleQuickActions = useMemo(
    () => quickActions.filter((action) => hasPermission(permissions, action.requiredPerm || "")),
    [permissions]
  );

  const userMenu: MenuProps = {
    items: [
      { key: "profile", label: "个人中心" },
      { key: "reload", label: "刷新数据", icon: <ReloadOutlined /> },
      { type: "divider" },
      { key: "logout", label: "退出登录", icon: <LogoutOutlined />, danger: true }
    ],
    onClick: async ({ key }) => {
      if (key === "logout") {
        logout();
        navigate("/login", { replace: true });
        return;
      }

      if (key === "profile") {
        messageApi.info("个人中心功能正在建设中");
        return;
      }

      messageApi.info("正在刷新权限与菜单...");
      window.location.reload();
    }
  };

  const getContextMenuItems = (tab: any): MenuProps["items"] => [
    { key: "close", label: "关闭当前", icon: <CloseOutlined />, disabled: !tab.closable },
    { key: "others", label: "关闭其他", icon: <CloseCircleOutlined /> },
    { key: "left", label: "关闭左侧", icon: <ArrowLeftOutlined /> },
    { key: "right", label: "关闭右侧", icon: <ArrowRightOutlined /> },
  ];

  const handleTabAction = (key: string, action: string) => {
    switch (action) {
      case "close": removeTab(key); break;
      case "others": closeOthers(key); break;
      case "left": closeLeft(key); break;
      case "right": closeRight(key); break;
    }
  };

  const toolMenu: MenuProps = {
    items: [
      { key: "closeAll", label: "关闭所有", icon: <MinusCircleOutlined /> },
      { key: "reload", label: "刷新当前", icon: <ReloadOutlined /> },
    ],
    onClick: ({ key }) => {
      if (key === "closeAll") closeAll();
      if (key === "reload") window.location.reload();
    }
  };

  return (
    <>
      {contextHolder}
      <Layout className="admin-layout">
        <Sider width={256} className="admin-sider" theme="dark">
          <div className="admin-brand">
            <div className="admin-brand__logo">R</div>
            <div>
              <div className="admin-brand__title">RustAdmin</div>
              <div className="admin-brand__version">v2.1.0 企业版</div>
            </div>
          </div>

          <div className="admin-sider-section">主菜单</div>
          <Menu
            mode="inline"
            theme="dark"
            className="admin-nav-menu"
            selectedKeys={selectedKeys}
            openKeys={openKeys}
            items={menuItems}
            onOpenChange={(keys) => setOpenKeys(keys as string[])}
            onClick={({ key }) => {
              const path = normalizePath(String(key));
              if (path) {
                navigate(path);
              }
            }}
          />

          <div className="admin-quick-actions">
            <div className="admin-sider-section">快捷操作</div>
            {visibleQuickActions.map((action) => (
              <Button
                key={action.key}
                block
                icon={action.icon}
                className="admin-quick-action-btn"
                onClick={() => navigate(action.key)}
              >
                {action.label}
              </Button>
            ))}
            {!authLoading && visibleQuickActions.length === 0 ? (
              <Typography.Text className="admin-brand__version">暂无可用快捷操作</Typography.Text>
            ) : null}
          </div>
        </Sider>

        <Layout className="admin-main">
          <Header className="admin-header">
            <div className="admin-header__left">
              <DashboardOutlined />
              <span>{currentTitle}</span>
            </div>

            <div className="admin-header__right">
              <Space size={8} className="admin-header__theme-wrap">
                <BgColorsOutlined className="admin-header__theme-icon" />
                <Select<AppThemeMode>
                  value={themeMode}
                  options={APP_THEME_OPTIONS}
                  onChange={setThemeMode}
                  className="admin-header__theme-select"
                  popupClassName="admin-theme-dropdown"
                  size="small"
                  bordered={false}
                />
              </Space>

              <Input
                className="admin-header__search"
                prefix={<SearchOutlined />}
                placeholder="搜索功能、用户或数据..."
                allowClear
              />

              <Space size={6}>
                <Button className="admin-header-icon-btn" icon={<BellOutlined />} />
                <Button className="admin-header-icon-btn" icon={<QuestionCircleOutlined />} />
              </Space>

              <Dropdown menu={userMenu} trigger={["click"]}>
                <button className="admin-user-btn" type="button">
                  <Avatar size={32} icon={<UserOutlined />} />
                  <div className="admin-user-btn__text">
                    <Typography.Text className="admin-user-btn__name">
                      {profile?.user.nickname || "Admin"}
                    </Typography.Text>
                    <Typography.Text className="admin-user-btn__role">
                      {hasPerm("*:*:*") ? "超级管理员" : profile?.user.username || "管理员"}
                    </Typography.Text>
                  </div>
                  <Badge status="processing" />
                </button>
              </Dropdown>
            </div>
          </Header>

          {/* Multi-Tabs Bar */}
          <div className="admin-tabs-container">
            <Tabs
              className="admin-tabs"
              activeKey={activeKey}
              onChange={(key) => navigate(key)}
              type="editable-card"
              hideAdd
              onEdit={(key, action) => {
                if (action === "remove") removeTab(key as string);
              }}
              items={tabs.map(tab => ({
                key: tab.key,
                closable: tab.closable,
                label: (
                  <Dropdown
                    menu={{ 
                      items: getContextMenuItems(tab),
                      onClick: ({ key: actionKey }) => handleTabAction(tab.key, actionKey)
                    }}
                    trigger={["contextMenu"]}
                  >
                    <span>
                      <span className="admin-tab-icon">{resolveMenuIcon(tab.icon)}</span>
                      {tab.label}
                    </span>
                  </Dropdown>
                )
              }))}
            />
            <div className="admin-tabs-tools">
              <Dropdown menu={toolMenu} placement="bottomRight">
                <Button className="admin-tabs-tool-btn" icon={<EllipsisOutlined />} />
              </Dropdown>
            </div>
          </div>

          <Content className="app-content" style={{ padding: 0, overflow: 'hidden', display: 'flex', flexDirection: 'column' }}>
            <div className="keep-alive-wrapper">
              {tabs.map((tab) => (
                <div
                  key={tab.key}
                  className="keep-alive-page"
                  style={{ display: tab.key === activeKey ? "block" : "none" }}
                >
                  {componentCache[tab.key] || (
                    <div style={{ padding: 20 }}>
                      <Typography.Text type="secondary">页面加载中...</Typography.Text>
                    </div>
                  )}
                </div>
              ))}
            </div>
          </Content>
        </Layout>
      </Layout>
    </>
  );
}
