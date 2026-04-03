import {
  BarChartOutlined,
  LockOutlined,
  QrcodeOutlined,
  SafetyCertificateOutlined,
  UserOutlined
} from "@ant-design/icons";
import { Button, Checkbox, Form, Input, message } from "antd";
import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { setAccessToken } from "../../../core/auth/token";
import { useDocumentTitle } from "../../../shared/hooks/useDocumentTitle";
import { login, type LoginReq } from "../services/authService";
import "./LoginPage.css";

type LoginFormValues = LoginReq & {
  remember?: boolean;
};

export function LoginPage() {
  const navigate = useNavigate();
  const [submitting, setSubmitting] = useState(false);
  useDocumentTitle("登录 - Rust Admin");

  async function handleSubmit(values: LoginFormValues) {
    const payload: LoginReq = {
      username: values.username,
      password: values.password
    };

    setSubmitting(true);
    try {
      const result = await login(payload);
      setAccessToken(result.access_token);
      message.success(`欢迎，${result.nickname}`);
      navigate("/dashboard");
    } catch (error) {
      const msg = error instanceof Error ? error.message : "登录失败";
      message.error(msg);
    } finally {
      setSubmitting(false);
    }
  }

  function handleNotImplemented() {
    message.info("该登录方式暂未开放");
  }

  return (
    <div className="login-page">
      <div className="login-page__overlay" />
      <div className="login-shell">
        <aside className="login-shell__hero">
          <div className="login-holo-logo">
            <div className="login-brand__mark">R</div>
            <h1 className="login-brand__name">RustAdmin</h1>
            <p className="login-brand__desc">智能、高效、可靠的管理平台</p>
          </div>

          <div className="login-pedestal" aria-hidden="true">
            <div className="login-pedestal__beam" />
            <div className="login-ring login-ring--outer" />
            <div className="login-ring login-ring--middle" />
            <div className="login-ring login-ring--inner" />

            <div className="login-float-icon login-float-icon--left">
              <BarChartOutlined />
            </div>
            <div className="login-float-icon login-float-icon--center">
              <UserOutlined />
            </div>
            <div className="login-float-icon login-float-icon--right">
              <SafetyCertificateOutlined />
            </div>
          </div>
        </aside>

        <div className="login-shell__divider" />

        <section className="login-shell__panel">
          <h2 className="login-panel__title">欢迎登录</h2>
          <p className="login-panel__subtitle">请输入您的账号信息</p>

          <Form<LoginFormValues>
            layout="vertical"
            className="login-form"
            initialValues={{
              username: "admin",
              password: "admin123456",
              remember: true
            }}
            onFinish={handleSubmit}
          >
            <Form.Item name="username" rules={[{ required: true, message: "请输入用户名" }]}>
              <Input size="large" placeholder="用户名 / 邮箱" prefix={<UserOutlined />} />
            </Form.Item>

            <Form.Item name="password" rules={[{ required: true, message: "请输入密码" }]}>
              <Input.Password size="large" placeholder="密码" prefix={<LockOutlined />} />
            </Form.Item>

            <div className="login-form__extra">
              <Form.Item name="remember" valuePropName="checked" noStyle>
                <Checkbox>记住我</Checkbox>
              </Form.Item>
              <Button type="link" className="login-form__forgot" onClick={handleNotImplemented}>
                忘记密码?
              </Button>
            </div>

            <Form.Item>
              <Button type="primary" htmlType="submit" block size="large" loading={submitting}>
                登录
              </Button>
            </Form.Item>
          </Form>

          <div className="login-divider-text">其他登录方式</div>
          <div className="login-alt-actions">
            <Button icon={<SafetyCertificateOutlined />} block size="large" onClick={handleNotImplemented}>
              SSO 登录
            </Button>
            <Button icon={<QrcodeOutlined />} block size="large" onClick={handleNotImplemented}>
              扫码登录
            </Button>
          </div>
        </section>

        <footer className="login-shell__footer">© 2026 RustAdmin. 保留所有权利。</footer>
      </div>
    </div>
  );
}
