import { DeleteOutlined, EditOutlined, PlusOutlined } from "@ant-design/icons";
import {
  Alert,
  Button,
  Card,
  Form,
  Input,
  InputNumber,
  Modal,
  Popconfirm,
  Select,
  Space,
  Table,
  message
} from "antd";
import type { TableProps } from "antd";
import { PageHeader } from "../../../shared/components/PageHeader";
import { useDocumentTitle } from "../../../shared/hooks/useDocumentTitle";
import {
  createSystemRecord,
  deleteSystemRecord,
  listSystemRecords,
  updateSystemRecord,
  type SystemCrudRecord
} from "../services/systemCrudService";
import type {
  CrudFieldConfig,
  CrudSelectOption,
  SystemCrudPageConfig
} from "../configs/crudConfigs";
import { useEffect, useState } from "react";

type SystemPageProps = {
  config: SystemCrudPageConfig;
};

type FormValues = Record<string, string | number>;
type TreeSystemCrudRecord = SystemCrudRecord & { children?: TreeSystemCrudRecord[] };
type MutableTreeSystemCrudRecord = SystemCrudRecord & { children: MutableTreeSystemCrudRecord[] };

function parseRecordId(value: unknown): number | null {
  if (typeof value === "number" && Number.isFinite(value)) {
    return value;
  }
  if (typeof value === "string") {
    const parsed = Number(value);
    return Number.isFinite(parsed) ? parsed : null;
  }
  return null;
}

function compactTreeNode(node: MutableTreeSystemCrudRecord): TreeSystemCrudRecord {
  if (node.children.length === 0) {
    const { children: _children, ...rest } = node;
    return rest;
  }

  const { children, ...rest } = node;
  return {
    ...rest,
    children: children.map((child) => compactTreeNode(child))
  };
}

function buildTreeRecords(records: SystemCrudRecord[], parentKey: string): TreeSystemCrudRecord[] {
  const nodeMap = new Map<number, MutableTreeSystemCrudRecord>();

  records.forEach((record) => {
    nodeMap.set(record.id, {
      ...record,
      children: []
    });
  });

  const roots: MutableTreeSystemCrudRecord[] = [];
  records.forEach((record) => {
    const node = nodeMap.get(record.id);
    if (!node) {
      return;
    }

    const parentId = parseRecordId(record[parentKey]);
    if (parentId === null || parentId === 0 || parentId === record.id) {
      roots.push(node);
      return;
    }

    const parentNode = nodeMap.get(parentId);
    if (!parentNode) {
      roots.push(node);
      return;
    }

    parentNode.children.push(node);
  });

  return roots.map((node) => compactTreeNode(node));
}

function buildParentSelectOptions(
  records: SystemCrudRecord[],
  parentKey: string,
  rootLabel: string,
  excludeId?: number
): CrudSelectOption[] {
  const tree = buildTreeRecords(records, parentKey);
  const options: CrudSelectOption[] = [{ label: rootLabel, value: 0 }];

  function walk(nodes: TreeSystemCrudRecord[], depth: number) {
    nodes.forEach((node) => {
      if (excludeId !== undefined && node.id === excludeId) {
        return;
      }

      const prefix = depth > 0 ? `${"--".repeat(depth)} ` : "";
      const rawName = node.name;
      const label = typeof rawName === "string" && rawName.trim() ? rawName : `ID:${node.id}`;

      options.push({
        label: `${prefix}${label}`,
        value: node.id
      });

      if (node.children && node.children.length > 0) {
        walk(node.children, depth + 1);
      }
    });
  }

  walk(tree, 0);
  return options;
}

export function SystemPage({ config }: SystemPageProps) {
  const [form] = Form.useForm<FormValues>();
  const [messageApi, contextHolder] = message.useMessage();
  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [records, setRecords] = useState<SystemCrudRecord[]>([]);
  const [total, setTotal] = useState(0);
  const [keywordInput, setKeywordInput] = useState("");
  const [keyword, setKeyword] = useState("");
  const [editingRecord, setEditingRecord] = useState<SystemCrudRecord | null>(null);
  const [modalOpen, setModalOpen] = useState(false);
  const treeConfig = config.tree;
  const isTreeMode = treeConfig?.enabled ?? false;
  const treeParentKey = treeConfig?.parentKey || "parent_id";
  const treeParentRootLabel = treeConfig?.parentRootLabel || "顶级";
  const dataSource = isTreeMode ? buildTreeRecords(records, treeParentKey) : records;
  const activeFields = config.fields.filter((field) => {
    if (field.createOnly && editingRecord) {
      return false;
    }
    if (field.editOnly && !editingRecord) {
      return false;
    }
    return true;
  });
  const parentSelectOptions =
    isTreeMode && activeFields.some((field) => field.key === treeParentKey)
      ? buildParentSelectOptions(records, treeParentKey, treeParentRootLabel, editingRecord?.id)
      : [];

  useDocumentTitle(`${config.title} - Rust Admin`);

  async function loadData(currentKeyword: string) {
    setLoading(true);
    setError(null);
    try {
      const data = await listSystemRecords(config.resource, currentKeyword);
      setRecords(data.items);
      setTotal(data.total);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : "加载列表失败";
      setError(errorMessage);
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    void loadData(keyword);
  }, [config.resource, keyword]);

  function openCreateModal() {
    setEditingRecord(null);
    const defaults = config.fields.reduce<Record<string, string | number>>((acc, field) => {
      if (field.editOnly) {
        return acc;
      }
      if (field.defaultValue !== undefined) {
        acc[field.key] = field.defaultValue;
      }
      return acc;
    }, {});
    form.resetFields();
    form.setFieldsValue(defaults);
    setModalOpen(true);
  }

  function openEditModal(record: SystemCrudRecord) {
    setEditingRecord(record);
    form.setFieldsValue(record as FormValues);
    setModalOpen(true);
  }

  function closeModal() {
    setModalOpen(false);
    setEditingRecord(null);
    form.resetFields();
  }

  async function handleDelete(record: SystemCrudRecord) {
    if (!Number.isFinite(record.id)) {
      messageApi.error("删除失败：记录ID无效");
      return;
    }

    try {
      await deleteSystemRecord(config.resource, record.id);
      messageApi.success("删除成功");
      await loadData(keyword);
    } catch (err) {
      messageApi.error(err instanceof Error ? err.message : "删除失败");
    }
  }

  async function handleSubmit() {
    try {
      const values = await form.validateFields(activeFields.map((field) => field.key));
      setSaving(true);
      if (editingRecord) {
        await updateSystemRecord(config.resource, editingRecord.id, values);
        messageApi.success("更新成功");
      } else {
        await createSystemRecord(config.resource, values);
        messageApi.success("新增成功");
      }
      closeModal();
      await loadData(keyword);
    } catch (err) {
      if (err instanceof Error) {
        messageApi.error(err.message || "提交失败");
      }
    } finally {
      setSaving(false);
    }
  }

  function renderFormControl(field: CrudFieldConfig) {
    if (field.type === "textarea") {
      return <Input.TextArea rows={4} placeholder={field.placeholder || `请输入${field.label}`} />;
    }
    if (field.type === "select") {
      const isParentField = isTreeMode && field.key === treeParentKey;
      return (
        <Select
          options={isParentField ? parentSelectOptions : field.options}
          placeholder={field.placeholder || `请选择${field.label}`}
          allowClear={!field.required}
        />
      );
    }
    if (field.type === "number") {
      return (
        <InputNumber
          style={{ width: "100%" }}
          placeholder={field.placeholder || `请输入${field.label}`}
          min={0}
        />
      );
    }

    return <Input placeholder={field.placeholder || `请输入${field.label}`} />;
  }

  const columns: TableProps<SystemCrudRecord>["columns"] = [
    ...config.columns.map((column) => ({
      title: column.label,
      dataIndex: column.key,
      key: column.key,
      width: column.width,
      ellipsis: column.ellipsis ?? true,
      render: (value: unknown) => {
        if (value === undefined || value === null) {
          return "-";
        }
        return String(value);
      }
    })),
    {
      title: "操作",
      key: "actions",
      width: 180,
      fixed: "right",
      render: (_, record) => (
        <Space size={12}>
          <Button
            type="link"
            size="small"
            icon={<EditOutlined />}
            onClick={() => openEditModal(record)}
          >
            编辑
          </Button>
          <Popconfirm title="确认删除这条记录吗？" onConfirm={() => handleDelete(record)}>
            <Button type="link" size="small" danger icon={<DeleteOutlined />}>
              删除
            </Button>
          </Popconfirm>
        </Space>
      )
    }
  ];

  return (
    <div className="biz-page">
      {contextHolder}
      <PageHeader title={config.title} description={config.description} />
      {error ? (
        <Alert
          type="error"
          showIcon
          message={`${config.title}加载失败`}
          description={error}
          style={{ marginBottom: 16 }}
        />
      ) : null}
      <Card>
        <Space style={{ width: "100%", justifyContent: "space-between", marginBottom: 16 }}>
          <Space>
            <Input.Search
              allowClear
              value={keywordInput}
              placeholder={config.searchPlaceholder}
              onChange={(event) => setKeywordInput(event.target.value)}
              onSearch={(value) => setKeyword(value.trim())}
              style={{ width: 280 }}
            />
            <Button onClick={() => {
              setKeywordInput("");
              setKeyword("");
            }}>
              重置
            </Button>
          </Space>
          <Button type="primary" icon={<PlusOutlined />} onClick={openCreateModal}>
            新增
          </Button>
        </Space>
        <Table<SystemCrudRecord>
          rowKey="id"
          loading={loading}
          columns={columns}
          dataSource={dataSource}
          pagination={isTreeMode ? false : { total, pageSize: 10, showSizeChanger: false }}
          scroll={{ x: 1000 }}
          defaultExpandAllRows={isTreeMode ? treeConfig?.expandAllByDefault ?? true : false}
        />
      </Card>

      <Modal
        title={editingRecord ? `编辑${config.title}` : `新增${config.title}`}
        open={modalOpen}
        wrapClassName="biz-modal"
        onCancel={closeModal}
        onOk={() => {
          void handleSubmit();
        }}
        confirmLoading={saving}
        destroyOnHidden
      >
        <Form<FormValues> form={form} layout="vertical">
          {activeFields.map((field) => (
            <Form.Item
              key={field.key}
              label={field.label}
              name={field.key}
              rules={
                field.required
                  ? [{ required: true, message: `${field.type === "select" ? "请选择" : "请输入"}${field.label}` }]
                  : undefined
              }
            >
              {renderFormControl(field)}
            </Form.Item>
          ))}
        </Form>
      </Modal>
    </div>
  );
}
