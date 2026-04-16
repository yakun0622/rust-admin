import { Alert, Button, Card, Input, Space, Table } from "antd";
import type { ColumnsType } from "antd/es/table";
import { useEffect, useState } from "react";
import { useDocumentTitle } from "../../../shared/hooks/useDocumentTitle";
import { getLoginLogs, getOperLogs, type LoginLogItem, type OperLogItem } from "../services/logService";

type LogPageProps = {
  title: string;
  type: "oper" | "login";
};

type RowItem = OperLogItem | LoginLogItem;

function formatTime(millis: number) {
  return new Date(millis).toLocaleString("zh-CN", { hour12: false });
}

export function LogPage({ title, type }: LogPageProps) {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [rows, setRows] = useState<RowItem[]>([]);
  const [total, setTotal] = useState(0);
  const [keywordInput, setKeywordInput] = useState("");
  const [keyword, setKeyword] = useState("");

  useDocumentTitle(`${title} - Rust Admin`);

  useEffect(() => {
    async function loadData() {
      setLoading(true);
      setError(null);
      try {
        if (type === "oper") {
          const data = await getOperLogs(keyword);
          setRows(data.items);
          setTotal(data.total);
        } else {
          const data = await getLoginLogs(keyword);
          setRows(data.items);
          setTotal(data.total);
        }
      } catch (err) {
        setError(err instanceof Error ? err.message : "加载日志失败");
      } finally {
        setLoading(false);
      }
    }

    void loadData();
  }, [keyword, type]);

  const operColumns: ColumnsType<OperLogItem> = [
    { title: "ID", dataIndex: "id", width: 80 },
    { title: "模块", dataIndex: "module", width: 160 },
    { title: "业务类型", dataIndex: "business_type", width: 130 },
    { title: "请求方式", dataIndex: "request_method", width: 120 },
    { title: "操作人", dataIndex: "oper_name", width: 140 },
    { title: "IP", dataIndex: "ip", width: 140 },
    { title: "操作地点", dataIndex: "location", width: 180 },
    { title: "状态", dataIndex: "status", width: 110 },
    { title: "耗时(ms)", dataIndex: "duration_ms", width: 110 },
    {
      title: "操作时间",
      dataIndex: "oper_at",
      width: 190,
      render: (value: number) => formatTime(value)
    }
  ];

  const loginColumns: ColumnsType<LoginLogItem> = [
    { title: "ID", dataIndex: "id", width: 80 },
    { title: "用户名", dataIndex: "username", width: 140 },
    { title: "类型", dataIndex: "login_type", width: 120 },
    { title: "IP", dataIndex: "ip", width: 160 },
    { title: "登录地点", dataIndex: "location", width: 180 },
    { title: "状态", dataIndex: "status", width: 100 },
    { title: "消息", dataIndex: "message", width: 220, ellipsis: true },
    {
      title: "登录时间",
      dataIndex: "login_at",
      width: 190,
      render: (value: number) => formatTime(value)
    }
  ];

  return (
    <div className="biz-page">
      {error ? (
        <Alert
          type="error"
          showIcon
          message={`${title}加载失败`}
          description={error}
          style={{ marginBottom: 16 }}
        />
      ) : null}
      <Card>
        <Space style={{ marginBottom: 16 }}>
          <Input.Search
            allowClear
            value={keywordInput}
            placeholder="按用户、IP、状态等关键字检索"
            style={{ width: 320 }}
            onChange={(event) => setKeywordInput(event.target.value)}
            onSearch={(value) => setKeyword(value.trim())}
          />
          <Button
            onClick={() => {
              setKeywordInput("");
              setKeyword("");
            }}
          >
            重置
          </Button>
        </Space>
        <Table<RowItem>
          rowKey="id"
          loading={loading}
          dataSource={rows}
          columns={type === "oper" ? (operColumns as ColumnsType<RowItem>) : (loginColumns as ColumnsType<RowItem>)}
          pagination={{ total, pageSize: 10, showSizeChanger: false }}
          scroll={{ x: 1150 }}
        />
      </Card>
    </div>
  );
}
