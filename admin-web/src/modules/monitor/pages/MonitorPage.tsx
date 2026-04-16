import { PauseCircleOutlined, PlayCircleOutlined, PlusOutlined } from "@ant-design/icons";
import {
  Alert,
  Button,
  Card,
  Descriptions,
  Form,
  Input,
  Modal,
  Popconfirm,
  Select,
  Space,
  Statistic,
  Table,
  Tag,
  message
} from "antd";
import type { ColumnsType } from "antd/es/table";
import { useEffect, useState } from "react";
import { useDocumentTitle } from "../../../shared/hooks/useDocumentTitle";
import {
  createJob,
  deleteJob,
  getCacheNamespaces,
  getDatasourceOverview,
  getJobs,
  getOnlineUsers,
  getServerOverview,
  pauseJob,
  resumeJob,
  runJob,
  searchCache,
  updateJob,
  type CacheKeyItem,
  type CacheNamespaceItem,
  type DatasourceOverview,
  type JobItem,
  type OnlineUserItem,
  type ServerOverview
} from "../services/monitorService";

type MonitorPageProps = {
  title: string;
  kind: "online" | "job" | "datasource" | "server" | "cache" | "cache-list";
};

type JobFormValues = {
  job_name: string;
  job_group: string;
  invoke_target: string;
  cron_expression: string;
  status: string;
  remark?: string;
};

function formatTime(millis?: number | null) {
  if (!millis) {
    return "-";
  }
  return new Date(millis).toLocaleString("zh-CN", { hour12: false });
}

export function MonitorPage({ title, kind }: MonitorPageProps) {
  const [messageApi, contextHolder] = message.useMessage();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const [keywordInput, setKeywordInput] = useState("");
  const [keyword, setKeyword] = useState("");

  const [onlineRows, setOnlineRows] = useState<OnlineUserItem[]>([]);
  const [jobRows, setJobRows] = useState<JobItem[]>([]);
  const [cacheRows, setCacheRows] = useState<CacheKeyItem[]>([]);
  const [namespaceRows, setNamespaceRows] = useState<CacheNamespaceItem[]>([]);
  const [datasource, setDatasource] = useState<DatasourceOverview | null>(null);
  const [server, setServer] = useState<ServerOverview | null>(null);

  const [jobModalOpen, setJobModalOpen] = useState(false);
  const [jobSaving, setJobSaving] = useState(false);
  const [editingJob, setEditingJob] = useState<JobItem | null>(null);
  const [jobForm] = Form.useForm<JobFormValues>();

  useDocumentTitle(`${title} - Rust Admin`);

  async function loadData(currentKeyword: string) {
    setLoading(true);
    setError(null);
    try {
      if (kind === "online") {
        const data = await getOnlineUsers(currentKeyword);
        setOnlineRows(data.items);
      } else if (kind === "job") {
        const data = await getJobs(currentKeyword);
        setJobRows(data.items);
      } else if (kind === "datasource") {
        const data = await getDatasourceOverview();
        setDatasource(data);
      } else if (kind === "server") {
        const data = await getServerOverview();
        setServer(data);
      } else if (kind === "cache") {
        const data = await searchCache(currentKeyword, 100);
        setCacheRows(data.items);
      } else if (kind === "cache-list") {
        const data = await getCacheNamespaces();
        setNamespaceRows(data.items);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : "加载监控数据失败");
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    void loadData(keyword);
  }, [kind, keyword]);

  function openCreateJob() {
    setEditingJob(null);
    jobForm.resetFields();
    jobForm.setFieldsValue({
      job_group: "DEFAULT",
      cron_expression: "0 */5 * * * *",
      status: "running"
    });
    setJobModalOpen(true);
  }

  function openEditJob(job: JobItem) {
    setEditingJob(job);
    jobForm.setFieldsValue({
      job_name: job.job_name,
      job_group: job.job_group,
      invoke_target: job.invoke_target,
      cron_expression: job.cron_expression,
      status: job.status,
      remark: job.remark
    });
    setJobModalOpen(true);
  }

  async function submitJob() {
    try {
      const values = await jobForm.validateFields();
      setJobSaving(true);
      if (editingJob) {
        await updateJob(editingJob.id, values);
        messageApi.success("任务更新成功");
      } else {
        await createJob(values);
        messageApi.success("任务创建成功");
      }
      setJobModalOpen(false);
      setEditingJob(null);
      jobForm.resetFields();
      await loadData(keyword);
    } catch (err) {
      if (err instanceof Error) {
        messageApi.error(err.message || "保存任务失败");
      }
    } finally {
      setJobSaving(false);
    }
  }

  async function handleJobAction(
    action: "run" | "pause" | "resume" | "delete",
    job: JobItem
  ) {
    try {
      if (action === "run") {
        const result = await runJob(job.id);
        messageApi.success(result.message);
      } else if (action === "pause") {
        const result = await pauseJob(job.id);
        messageApi.success(result.message);
      } else if (action === "resume") {
        const result = await resumeJob(job.id);
        messageApi.success(result.message);
      } else {
        const result = await deleteJob(job.id);
        messageApi.success(result.message);
      }
      await loadData(keyword);
    } catch (err) {
      messageApi.error(err instanceof Error ? err.message : "任务操作失败");
    }
  }

  const onlineColumns: ColumnsType<OnlineUserItem> = [
    { title: "ID", dataIndex: "id", width: 80 },
    { title: "用户名", dataIndex: "username", width: 140 },
    { title: "IP", dataIndex: "ip", width: 150 },
    { title: "浏览器", dataIndex: "browser", width: 150 },
    { title: "系统", dataIndex: "os", width: 120 },
    {
      title: "登录时间",
      dataIndex: "login_at",
      width: 180,
      render: (value: number) => formatTime(value)
    },
    {
      title: "最近活跃",
      dataIndex: "last_active_at",
      width: 180,
      render: (value: number) => formatTime(value)
    },
    {
      title: "状态",
      dataIndex: "status",
      width: 100,
      render: (value: string) => (
        <Tag color={value === "online" ? "success" : "default"}>
          {value === "online" ? "在线" : "离线"}
        </Tag>
      )
    }
  ];

  const jobColumns: ColumnsType<JobItem> = [
    { title: "ID", dataIndex: "id", width: 80 },
    { title: "任务名称", dataIndex: "job_name", width: 180 },
    { title: "任务组", dataIndex: "job_group", width: 140 },
    { title: "调用目标", dataIndex: "invoke_target", width: 220, ellipsis: true },
    { title: "Cron", dataIndex: "cron_expression", width: 170 },
    {
      title: "状态",
      dataIndex: "status",
      width: 110,
      render: (value: string) => (
        <Tag color={value === "running" ? "success" : "warning"}>
          {value === "running" ? "运行中" : "已暂停"}
        </Tag>
      )
    },
    {
      title: "上次执行",
      dataIndex: "last_run_at",
      width: 180,
      render: (value: number | null) => formatTime(value)
    },
    {
      title: "下次执行",
      dataIndex: "next_run_at",
      width: 180,
      render: (value: number | null) => formatTime(value)
    },
    {
      title: "操作",
      key: "actions",
      width: 300,
      fixed: "right",
      render: (_, row) => (
        <Space size={8}>
          <Button type="link" size="small" onClick={() => openEditJob(row)}>
            编辑
          </Button>
          <Button type="link" size="small" onClick={() => void handleJobAction("run", row)}>
            执行一次
          </Button>
          {row.status === "running" ? (
            <Button
              type="link"
              size="small"
              icon={<PauseCircleOutlined />}
              onClick={() => void handleJobAction("pause", row)}
            >
              暂停
            </Button>
          ) : (
            <Button
              type="link"
              size="small"
              icon={<PlayCircleOutlined />}
              onClick={() => void handleJobAction("resume", row)}
            >
              恢复
            </Button>
          )}
          <Popconfirm
            title="确认删除该任务吗？"
            onConfirm={() => void handleJobAction("delete", row)}
          >
            <Button type="link" size="small" danger>
              删除
            </Button>
          </Popconfirm>
        </Space>
      )
    }
  ];

  const cacheColumns: ColumnsType<CacheKeyItem> = [
    { title: "Key", dataIndex: "key", width: 320, ellipsis: true },
    { title: "类型", dataIndex: "data_type", width: 120 },
    { title: "TTL(s)", dataIndex: "ttl_secs", width: 120 },
    { title: "示例值", dataIndex: "sample", width: 420, ellipsis: true }
  ];

  const namespaceColumns: ColumnsType<CacheNamespaceItem> = [
    { title: "命名空间", dataIndex: "namespace", width: 220 },
    { title: "Key 数量", dataIndex: "key_count", width: 140 },
    { title: "示例 Key", dataIndex: "example_key", width: 380, ellipsis: true }
  ];

  return (
    <div className="biz-page">
      {contextHolder}
      {error ? (
        <Alert
          type="error"
          showIcon
          message={`${title}加载失败`}
          description={error}
          style={{ marginBottom: 16 }}
        />
      ) : null}

      {(kind === "online" || kind === "job" || kind === "cache") ? (
        <Card style={{ marginBottom: 16 }}>
          <Space style={{ width: "100%", justifyContent: "space-between" }}>
            <Space>
              <Input.Search
                allowClear
                value={keywordInput}
                placeholder={
                  kind === "online"
                    ? "按用户名、IP 搜索"
                    : kind === "job"
                    ? "按任务名、目标、状态搜索"
                    : "按 Key 关键字搜索"
                }
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
            {kind === "job" ? (
              <Button type="primary" icon={<PlusOutlined />} onClick={openCreateJob}>
                新建任务
              </Button>
            ) : null}
          </Space>
        </Card>
      ) : null}

      {kind === "online" ? (
        <Card>
          <Table
            rowKey="id"
            loading={loading}
            columns={onlineColumns}
            dataSource={onlineRows}
            pagination={{ pageSize: 10, showSizeChanger: false }}
            scroll={{ x: 1150 }}
          />
        </Card>
      ) : null}

      {kind === "job" ? (
        <>
          <Card>
            <Table
              rowKey="id"
              loading={loading}
              columns={jobColumns}
              dataSource={jobRows}
              pagination={{ pageSize: 10, showSizeChanger: false }}
              scroll={{ x: 1600 }}
            />
          </Card>
          <Modal
            title={editingJob ? "编辑任务" : "新建任务"}
            open={jobModalOpen}
            wrapClassName="biz-modal"
            confirmLoading={jobSaving}
            onCancel={() => {
              setJobModalOpen(false);
              setEditingJob(null);
              jobForm.resetFields();
            }}
            onOk={() => {
              void submitJob();
            }}
            destroyOnHidden
          >
            <Form<JobFormValues> form={jobForm} layout="vertical">
              <Form.Item
                label="任务名称"
                name="job_name"
                rules={[{ required: true, message: "请输入任务名称" }]}
              >
                <Input />
              </Form.Item>
              <Form.Item
                label="任务组"
                name="job_group"
                rules={[{ required: true, message: "请输入任务组" }]}
              >
                <Input />
              </Form.Item>
              <Form.Item
                label="调用目标"
                name="invoke_target"
                rules={[{ required: true, message: "请输入调用目标" }]}
              >
                <Input />
              </Form.Item>
              <Form.Item
                label="Cron 表达式"
                name="cron_expression"
                rules={[{ required: true, message: "请输入 Cron 表达式" }]}
              >
                <Input />
              </Form.Item>
              <Form.Item label="状态" name="status" rules={[{ required: true }]}>
                <Select
                  options={[
                    { label: "运行中", value: "running" },
                    { label: "已暂停", value: "paused" }
                  ]}
                />
              </Form.Item>
              <Form.Item label="备注" name="remark">
                <Input.TextArea rows={3} />
              </Form.Item>
            </Form>
          </Modal>
        </>
      ) : null}

      {kind === "datasource" ? (
        <Card
          extra={
            <Button onClick={() => void loadData(keyword)} loading={loading}>
              刷新
            </Button>
          }
        >
          {datasource ? (
            <>
              <Descriptions column={1} bordered>
                <Descriptions.Item label="数据库">{datasource.database}</Descriptions.Item>
                <Descriptions.Item label="连接串">{datasource.mysql_url}</Descriptions.Item>
                <Descriptions.Item label="最大连接数">{datasource.max_connections}</Descriptions.Item>
                <Descriptions.Item label="最小连接数">{datasource.min_connections}</Descriptions.Item>
                <Descriptions.Item label="健康状态">
                  <Tag color={datasource.ping_ok ? "success" : "error"}>
                    {datasource.ping_ok ? "正常" : "异常"}
                  </Tag>
                </Descriptions.Item>
                <Descriptions.Item label="健康信息">{datasource.ping_message}</Descriptions.Item>
              </Descriptions>
            </>
          ) : null}
        </Card>
      ) : null}

      {kind === "server" ? (
        <Card
          extra={
            <Button onClick={() => void loadData(keyword)} loading={loading}>
              刷新
            </Button>
          }
        >
          {server ? (
            <Space size={24} wrap>
              <Statistic title="服务名" value={server.app_name} />
              <Statistic title="环境" value={server.env} />
              <Statistic title="运行时长(秒)" value={server.uptime_secs} />
              <Statistic
                title="MySQL"
                value={server.mysql_ok ? "正常" : "异常"}
                valueStyle={{ color: server.mysql_ok ? "#3f8600" : "#cf1322" }}
              />
              <Statistic
                title="Redis"
                value={server.redis_ok ? "正常" : "异常"}
                valueStyle={{ color: server.redis_ok ? "#3f8600" : "#cf1322" }}
              />
              <Statistic title="当前时间" value={formatTime(server.now_millis)} />
            </Space>
          ) : null}
        </Card>
      ) : null}

      {kind === "cache" ? (
        <Card>
          <Table
            rowKey="key"
            loading={loading}
            columns={cacheColumns}
            dataSource={cacheRows}
            pagination={{ pageSize: 10, showSizeChanger: false }}
            scroll={{ x: 1100 }}
          />
        </Card>
      ) : null}

      {kind === "cache-list" ? (
        <Card
          extra={
            <Button onClick={() => void loadData(keyword)} loading={loading}>
              刷新
            </Button>
          }
        >
          <Table
            rowKey="namespace"
            loading={loading}
            columns={namespaceColumns}
            dataSource={namespaceRows}
            pagination={{ pageSize: 10, showSizeChanger: false }}
            scroll={{ x: 900 }}
          />
        </Card>
      ) : null}
    </div>
  );
}
