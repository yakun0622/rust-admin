import { PlusOutlined } from "@ant-design/icons";
import { Alert, Button, Card, Empty, Input, List, Space, Spin, Typography, message } from "antd";
import { useEffect, useState } from "react";
import { useDocumentTitle } from "../../../shared/hooks/useDocumentTitle";
import {
  createAiSession,
  getAiMessages,
  getAiSessions,
  sendAiMessage,
  type AiMessageItem,
  type AiSessionItem
} from "../services/aiService";

function formatTime(millis: number) {
  return new Date(millis).toLocaleTimeString("zh-CN", { hour12: false });
}

export function AiChatPage() {
  const [messageApi, contextHolder] = message.useMessage();
  const [loadingSessions, setLoadingSessions] = useState(false);
  const [loadingMessages, setLoadingMessages] = useState(false);
  const [sending, setSending] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [sessions, setSessions] = useState<AiSessionItem[]>([]);
  const [activeSessionId, setActiveSessionId] = useState<number | null>(null);
  const [messages, setMessages] = useState<AiMessageItem[]>([]);
  const [prompt, setPrompt] = useState("");

  useDocumentTitle("AI 对话 - Rust Admin");

  async function loadSessions(preserveCurrent = true) {
    setLoadingSessions(true);
    setError(null);
    try {
      const data = await getAiSessions();
      setSessions(data.items);
      if (!data.items.length) {
        setActiveSessionId(null);
        setMessages([]);
        return;
      }

      if (preserveCurrent && activeSessionId && data.items.some((item) => item.id === activeSessionId)) {
        return;
      }
      setActiveSessionId(data.items[0].id);
    } catch (err) {
      setError(err instanceof Error ? err.message : "加载会话失败");
    } finally {
      setLoadingSessions(false);
    }
  }

  async function loadMessages(sessionId: number) {
    setLoadingMessages(true);
    setError(null);
    try {
      const data = await getAiMessages(sessionId);
      setMessages(data.items);
    } catch (err) {
      setError(err instanceof Error ? err.message : "加载消息失败");
    } finally {
      setLoadingMessages(false);
    }
  }

  useEffect(() => {
    void loadSessions(false);
  }, []);

  useEffect(() => {
    if (!activeSessionId) {
      return;
    }
    void loadMessages(activeSessionId);
  }, [activeSessionId]);

  async function handleCreateSession() {
    try {
      const created = await createAiSession(`新会话 ${new Date().toLocaleString("zh-CN")}`);
      await loadSessions(false);
      setActiveSessionId(created.id);
      messageApi.success("会话创建成功");
    } catch (err) {
      messageApi.error(err instanceof Error ? err.message : "创建会话失败");
    }
  }

  async function handleSend() {
    if (!activeSessionId) {
      messageApi.warning("请先创建会话");
      return;
    }
    const content = prompt.trim();
    if (!content) {
      return;
    }

    try {
      setSending(true);
      const result = await sendAiMessage(activeSessionId, content);
      setMessages((prev) => [...prev, result.user_message, result.assistant_message]);
      setPrompt("");
      await loadSessions(true);
    } catch (err) {
      messageApi.error(err instanceof Error ? err.message : "发送消息失败");
    } finally {
      setSending(false);
    }
  }

  return (
    <div className="biz-page">
      {contextHolder}
      {error ? (
        <Alert
          type="error"
          showIcon
          message="AI 模块加载失败"
          description={error}
          style={{ marginBottom: 16 }}
        />
      ) : null}
      <Card>
        <Space
          style={{
            width: "100%",
            alignItems: "flex-start",
            justifyContent: "space-between",
            gap: 16
          }}
        >
          <div style={{ width: 280 }}>
            <Space style={{ marginBottom: 12 }}>
              <Typography.Text strong>会话列表</Typography.Text>
              <Button
                size="small"
                icon={<PlusOutlined />}
                onClick={() => {
                  void handleCreateSession();
                }}
              >
                新建会话
              </Button>
            </Space>
            {loadingSessions ? (
              <Spin />
            ) : (
              <List
                bordered
                size="small"
                dataSource={sessions}
                locale={{ emptyText: <Empty image={Empty.PRESENTED_IMAGE_SIMPLE} description="暂无会话" /> }}
                renderItem={(item) => (
                  <List.Item
                    style={{
                      cursor: "pointer",
                      background: activeSessionId === item.id ? "rgba(34, 102, 212, 0.42)" : undefined
                    }}
                    onClick={() => setActiveSessionId(item.id)}
                  >
                    <Space direction="vertical" size={0}>
                      <Typography.Text>{item.title}</Typography.Text>
                      <Typography.Text type="secondary" style={{ fontSize: 12 }}>
                        {formatTime(item.last_active_at)}
                      </Typography.Text>
                    </Space>
                  </List.Item>
                )}
              />
            )}
          </div>

          <div style={{ flex: 1 }}>
            {activeSessionId ? (
              <>
                {loadingMessages ? (
                  <Spin />
                ) : (
                  <List
                    size="small"
                    bordered
                    dataSource={messages}
                    style={{ marginBottom: 12, minHeight: 320 }}
                    locale={{
                      emptyText: <Empty image={Empty.PRESENTED_IMAGE_SIMPLE} description="当前会话暂无消息" />
                    }}
                    renderItem={(item) => (
                      <List.Item>
                        <Space direction="vertical" size={0}>
                          <Typography.Text strong={item.role === "user"}>
                            {item.role === "user" ? "你" : "AI"}：
                            {item.content}
                          </Typography.Text>
                          <Typography.Text type="secondary" style={{ fontSize: 12 }}>
                            {formatTime(item.created_at)}
                          </Typography.Text>
                        </Space>
                      </List.Item>
                    )}
                  />
                )}
                <Space.Compact style={{ width: "100%" }}>
                  <Input
                    value={prompt}
                    onChange={(event) => setPrompt(event.target.value)}
                    placeholder="输入消息（Mock）"
                    onPressEnter={() => {
                      void handleSend();
                    }}
                  />
                  <Button
                    type="primary"
                    loading={sending}
                    onClick={() => {
                      void handleSend();
                    }}
                  >
                    发送
                  </Button>
                </Space.Compact>
              </>
            ) : (
              <Empty description="请先创建会话" />
            )}
            <Typography.Paragraph type="secondary" style={{ marginTop: 12, marginBottom: 0 }}>
              当前回复为后端 Mock 逻辑生成，后续可无缝替换为模型网关。
            </Typography.Paragraph>
          </div>
        </Space>
      </Card>
    </div>
  );
}
