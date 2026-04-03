import {
  ApartmentOutlined,
  BgColorsOutlined,
  BellOutlined,
  BookOutlined,
  CloudServerOutlined,
  DashboardOutlined,
  DatabaseOutlined,
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
  UserOutlined
} from "@ant-design/icons";
import { Avatar, Badge, Button, Dropdown, Input, Layout, Menu, Select, Space, Typography, message } from "antd";
import type { MenuProps } from "antd";
import type { ReactNode } from "react";
import { useEffect, useMemo, useState } from "react";
import { Outlet, useLocation, useNavigate } from "react-router-dom";
import { clearAccessToken } from "../../core/auth/token";
import { APP_THEME_OPTIONS, useAppTheme, type AppThemeMode } from "../providers";
import "./AdminLayout.css";

const { Header, Sider, Content } = Layout;

type QuickAction = {
  key: string;
  label: string;
  icon: ReactNode;
};

const menuItems: MenuProps["items"] = [
  { key: "/dashboard", icon: <DashboardOutlined />, label: "仪表盘" },
  { key: "/ai/chat", icon: <MessageOutlined />, label: "AI 对话" },
  {
    key: "system",
    icon: <SettingOutlined />,
    label: "系统管理",
    children: [
      { key: "/system/user", icon: <UserOutlined />, label: "用户管理" },
      { key: "/system/role", icon: <TeamOutlined />, label: "角色管理" },
      { key: "/system/menu", icon: <MenuOutlined />, label: "菜单管理" },
      { key: "/system/dept", icon: <ApartmentOutlined />, label: "部门管理" },
      { key: "/system/post", icon: <IdcardOutlined />, label: "岗位管理" },
      { key: "/system/dict", icon: <BookOutlined />, label: "字典管理" },
      { key: "/system/config", icon: <ToolOutlined />, label: "参数设置" },
      { key: "/system/notice", icon: <NotificationOutlined />, label: "通知公告" }
    ]
  },
  {
    key: "log",
    icon: <FileTextOutlined />,
    label: "日志管理",
    children: [
      { key: "/log/oper", icon: <ProfileOutlined />, label: "操作日志" },
      { key: "/log/login", icon: <LoginOutlined />, label: "登录日志" }
    ]
  },
  {
    key: "monitor",
    icon: <MonitorOutlined />,
    label: "系统监控",
    children: [
      { key: "/monitor/online", icon: <UserOutlined />, label: "在线用户" },
      { key: "/monitor/job", icon: <FundProjectionScreenOutlined />, label: "定时任务" },
      { key: "/monitor/datasource", icon: <DatabaseOutlined />, label: "数据监控" },
      { key: "/monitor/server", icon: <CloudServerOutlined />, label: "服务监控" },
      { key: "/monitor/cache", icon: <HddOutlined />, label: "缓存监控" },
      { key: "/monitor/cache-list", icon: <MenuOutlined />, label: "缓存列表" }
    ]
  }
];

const routeTitleMap: Record<string, string> = {
  "/dashboard": "仪表盘",
  "/ai/chat": "AI 对话",
  "/system/user": "用户管理",
  "/system/role": "角色管理",
  "/system/menu": "菜单管理",
  "/system/dept": "部门管理",
  "/system/post": "岗位管理",
  "/system/dict": "字典管理",
  "/system/config": "参数设置",
  "/system/notice": "通知公告",
  "/log/oper": "操作日志",
  "/log/login": "登录日志",
  "/monitor/online": "在线用户",
  "/monitor/job": "定时任务",
  "/monitor/datasource": "数据监控",
  "/monitor/server": "服务监控",
  "/monitor/cache": "缓存监控",
  "/monitor/cache-list": "缓存列表"
};

const parentKeyByPath: Record<string, string | undefined> = {
  "/system/user": "system",
  "/system/role": "system",
  "/system/menu": "system",
  "/system/dept": "system",
  "/system/post": "system",
  "/system/dict": "system",
  "/system/config": "system",
  "/system/notice": "system",
  "/log/oper": "log",
  "/log/login": "log",
  "/monitor/online": "monitor",
  "/monitor/job": "monitor",
  "/monitor/datasource": "monitor",
  "/monitor/server": "monitor",
  "/monitor/cache": "monitor",
  "/monitor/cache-list": "monitor"
};

const quickActions: QuickAction[] = [
  { key: "/system/user", label: "新增用户", icon: <UserOutlined /> },
  { key: "/system/config", label: "系统配置", icon: <SettingOutlined /> },
  { key: "/log/oper", label: "查看日志", icon: <FileTextOutlined /> },
  { key: "/monitor/datasource", label: "数据备份", icon: <DatabaseOutlined /> }
];

function resolveCurrentTitle(pathname: string): string {
  if (routeTitleMap[pathname]) {
    return routeTitleMap[pathname];
  }

  const matchedRoute = Object.keys(routeTitleMap).find((route) => pathname.startsWith(route));
  return matchedRoute ? routeTitleMap[matchedRoute] : "后台管理系统";
}

export function AdminLayout() {
  const navigate = useNavigate();
  const location = useLocation();
  const [messageApi, contextHolder] = message.useMessage();
  const { themeMode, setThemeMode } = useAppTheme();
  const [openKeys, setOpenKeys] = useState<string[]>([]);
  const currentParentKey = parentKeyByPath[location.pathname];

  useEffect(() => {
    if (currentParentKey) {
      setOpenKeys((prev) =>
        prev.includes(currentParentKey) ? prev : [currentParentKey, ...prev].slice(0, 3)
      );
    }
  }, [currentParentKey]);

  const currentTitle = useMemo(() => resolveCurrentTitle(location.pathname), [location.pathname]);

  const userMenu: MenuProps = {
    items: [
      { key: "profile", label: "个人中心" },
      { key: "reload", label: "刷新数据", icon: <ReloadOutlined /> },
      { type: "divider" },
      { key: "logout", label: "退出登录", icon: <LogoutOutlined />, danger: true }
    ],
    onClick: ({ key }) => {
      if (key === "logout") {
        clearAccessToken();
        navigate("/login", { replace: true });
        return;
      }

      messageApi.info("该功能正在建设中");
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
            selectedKeys={[location.pathname]}
            openKeys={openKeys}
            items={menuItems}
            onOpenChange={(keys) => setOpenKeys(keys as string[])}
            onClick={({ key }) => {
              if (String(key).startsWith("/")) {
                navigate(String(key));
              }
            }}
          />

          <div className="admin-quick-actions">
            <div className="admin-sider-section">快捷操作</div>
            {quickActions.map((action) => (
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
                    <Typography.Text className="admin-user-btn__name">Admin</Typography.Text>
                    <Typography.Text className="admin-user-btn__role">管理员</Typography.Text>
                  </div>
                  <Badge status="processing" />
                </button>
              </Dropdown>
            </div>
          </Header>

          <Content className="app-content">
            <Outlet />
          </Content>
        </Layout>
      </Layout>
    </>
  );
}
