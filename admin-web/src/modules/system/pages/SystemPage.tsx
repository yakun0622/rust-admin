import { DeleteOutlined, EditOutlined, PlusOutlined } from "@ant-design/icons";
import {
  Alert,
  Button,
  Card,
  Drawer,
  Form,
  Input,
  InputNumber,
  Modal,
  Popconfirm,
  Select,
  Spin,
  Space,
  Table,
  Tag,
  Tree,
  TreeSelect,
  message,
  notification
} from "antd";
import type { TableProps, TreeProps } from "antd";
import type { Key } from "react";
import { useEffect, useState } from "react";
import { useAuthSession } from "../../../app/providers";
import { hasPermission } from "../../../core/permission";
import { useDocumentTitle } from "../../../shared/hooks/useDocumentTitle";
import {
  createSystemRecord,
  deleteSystemRecord,
  listSystemRecords,
  updateSystemRecord,
  listRoleMenuIds,
  updateRoleMenuIds,
  type SystemCrudRecord
} from "../services/systemCrudService";
import type {
  CrudFieldConfig,
  CrudSelectOption,
  SystemCrudPageConfig
} from "../configs/crudConfigs";

type SystemPageProps = {
  config: SystemCrudPageConfig;
};

type FormValues = Record<string, string | number>;
type TreeSystemCrudRecord = SystemCrudRecord & { children?: TreeSystemCrudRecord[] };
type MutableTreeSystemCrudRecord = SystemCrudRecord & { children: MutableTreeSystemCrudRecord[] };
const menuTypeLabelMap: Record<number, string> = {
  1: "目录",
  2: "菜单",
  3: "按钮"
};

const statusLabelMap: Record<string, { label: string; color: string }> = {
  enabled: { label: "启用", color: "success" },
  disabled: { label: "停用", color: "error" },
  published: { label: "已发布", color: "success" },
  draft: { label: "草稿", color: "default" },
  offline: { label: "已下线", color: "warning" }
};

const visibleLabelMap: Record<string, { label: string; color: string }> = {
  yes: { label: "显示", color: "processing" },
  no: { label: "隐藏", color: "default" }
};

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

function buildParentTreeData(
  records: SystemCrudRecord[],
  resource: string,
  parentKey: string,
  rootLabel: string,
  excludeId?: number
): any[] {
  // Filter out buttons if it's menu resource
  let filteredRecords = records;
  if (resource === "menu") {
    filteredRecords = records.filter((r) => r.menu_type !== 3);
  }

  const tree = buildTreeRecords(filteredRecords, parentKey);

  function mapToTreeSelect(nodes: TreeSystemCrudRecord[]): any[] {
    return nodes
      .filter((node) => node.id !== excludeId)
      .map((node) => ({
        title: node.name,
        value: node.id,
        children: node.children ? mapToTreeSelect(node.children) : []
      }));
  }

  return [
    {
      title: rootLabel,
      value: 0,
      children: mapToTreeSelect(tree)
    }
  ];
}

function normalizeOptionalText(value: unknown): string | undefined {
  if (typeof value !== "string") {
    return undefined;
  }
  const normalized = value.trim();
  return normalized ? normalized : undefined;
}

function parseNumberValue(value: unknown, fallback: number): number {
  if (typeof value === "number" && Number.isFinite(value)) {
    return value;
  }
  if (typeof value === "string") {
    const parsed = Number(value.trim());
    if (Number.isFinite(parsed)) {
      return parsed;
    }
  }
  return fallback;
}

function buildSubmitPayload(resource: string, values: FormValues): Record<string, unknown> {
  if (resource !== "menu") {
    return values;
  }

  const menuType = parseNumberValue(values.menu_type, 2);
  const parentId = parseNumberValue(values.parent_id, 0);
  const orderNum = parseNumberValue(values.order_num, 1);

  const name = normalizeOptionalText(values.name);
  if (!name) {
    throw new Error("菜单名称不能为空");
  }

  const routeName = normalizeOptionalText(values.route_name);
  const path = normalizeOptionalText(values.path);
  const component = normalizeOptionalText(values.component);
  const permission = normalizeOptionalText(values.permission);
  const icon = normalizeOptionalText(values.icon);

  if (menuType === 2 && (!path || !component)) {
    throw new Error("菜单类型为“菜单”时，路由地址和组件名不能为空");
  }
  if (menuType === 3 && !permission) {
    throw new Error("菜单类型为“按钮”时，权限标识不能为空");
  }

  return {
    parent_id: parentId,
    menu_type: menuType,
    name,
    route_name: menuType === 3 ? undefined : routeName,
    path: menuType === 3 ? undefined : path,
    component: menuType === 3 ? undefined : component,
    permission,
    icon,
    order_num: orderNum,
    status: normalizeOptionalText(values.status) || "enabled",
    visible: normalizeOptionalText(values.visible) || "yes"
  };
}

export function SystemPage({ config }: SystemPageProps) {
  const [form] = Form.useForm<FormValues>();
  const [searchForm] = Form.useForm();
  const [messageApi, messageContext] = message.useMessage();
  const [notificationApi, notificationContext] = notification.useNotification();
  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [records, setRecords] = useState<SystemCrudRecord[]>([]);
  const [total, setTotal] = useState(0);
  const [searchParams, setSearchParams] = useState<Record<string, unknown>>({});
  const [editingRecord, setEditingRecord] = useState<SystemCrudRecord | null>(null);
  const [modalOpen, setModalOpen] = useState(false);

  const [configuringRole, setConfiguringRole] = useState<SystemCrudRecord | null>(null);
  const [permissionDrawerOpen, setPermissionDrawerOpen] = useState(false);
  const [menuTreeData, setMenuTreeData] = useState<TreeSystemCrudRecord[]>([]);
  const [checkedMenuIds, setCheckedMenuIds] = useState<Key[]>([]);
  const [permissionLoading, setPermissionLoading] = useState(false);

  const { permissions: userPerms, loading: authLoading } = useAuthSession();
  const permissionConfig = config.permissions || {};
  const canView = hasPermission(userPerms, permissionConfig.view || "");
  const canCreate = hasPermission(userPerms, permissionConfig.create || "");
  const canUpdate = hasPermission(userPerms, permissionConfig.update || "");
  const canDelete = hasPermission(userPerms, permissionConfig.delete || "");
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

  const parentTreeData =
    isTreeMode && activeFields.some((field) => field.key === treeParentKey)
      ? buildParentTreeData(
          records,
          config.resource,
          treeParentKey,
          treeParentRootLabel,
          editingRecord?.id
        )
      : [];

  useDocumentTitle(`${config.title} - Rust Admin`);

  async function loadData(params: Record<string, unknown>) {
    setLoading(true);
    setError(null);
    try {
      const data = await listSystemRecords(config.resource, params);
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
    if (authLoading) {
      return;
    }
    if (!canView) {
      setError(null);
      setRecords([]);
      setTotal(0);
      setLoading(false);
      return;
    }
    void loadData(searchParams);
  }, [authLoading, canView, config.resource, searchParams]);

  function openCreateModal() {
    if (!canCreate) {
      messageApi.warning("没有新增权限");
      return;
    }
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
    if (!canUpdate) {
      messageApi.warning("没有编辑权限");
      return;
    }
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
    if (!canDelete) {
      messageApi.warning("没有删除权限");
      return;
    }
    if (!Number.isFinite(record.id)) {
      messageApi.error("删除失败：记录ID无效");
      return;
    }

    try {
      await deleteSystemRecord(config.resource, record.id);
      messageApi.success("删除成功");
      await loadData(searchParams);
    } catch (err) {
      messageApi.error(err instanceof Error ? err.message : "删除失败");
    }
  }

  async function handleSubmit() {
    if (editingRecord && !canUpdate) {
      messageApi.warning("没有编辑权限");
      return;
    }
    if (!editingRecord && !canCreate) {
      messageApi.warning("没有新增权限");
      return;
    }

    try {
      const values = await form.validateFields(activeFields.map((field) => field.key));
      const payload = buildSubmitPayload(config.resource, values);
      setSaving(true);
      if (editingRecord) {
        await updateSystemRecord(config.resource, editingRecord.id, payload);
        messageApi.success("更新成功");
      } else {
        await createSystemRecord(config.resource, payload);
        messageApi.success("新增成功");
      }
      closeModal();
      await loadData(searchParams);
    } catch (err) {
      if (err instanceof Error) {
        messageApi.error(err.message || "提交失败");
      }
    } finally {
      setSaving(false);
    }
  }

  async function openPermissionDrawer(record: SystemCrudRecord) {
    setConfiguringRole(record);
    setPermissionDrawerOpen(true);
    setPermissionLoading(true);
    try {
      // Fetch all menus for the tree
      const menuRes = await listSystemRecords("menu");
      const tree = buildTreeRecords(menuRes.items, "parent_id");
      setMenuTreeData(tree);

      // Fetch current checked menu IDs for the role
      const checkedIds = await listRoleMenuIds(record.id);
      setCheckedMenuIds(checkedIds.map(id => id.toString()));
    } catch (err) {
      messageApi.error(err instanceof Error ? err.message : "获取权限数据失败");
    } finally {
      setPermissionLoading(false);
    }
  }

  async function handlePermissionSave() {
    if (!configuringRole) return;
    setPermissionLoading(true);
    try {
      const menuIds = checkedMenuIds.map(id => Number(id)).filter(id => !isNaN(id));
      await updateRoleMenuIds(configuringRole.id, menuIds);
      notificationApi.success({
        message: "权限分配成功",
        description: `角色「${configuringRole.role_name || configuringRole.name}」的权限已成功更新。`,
        placement: "topRight"
      });
      setPermissionDrawerOpen(false);
    } catch (err) {
      notificationApi.error({
        message: "保存权限失败",
        description: err instanceof Error ? err.message : "未知错误",
        placement: "topRight"
      });
    } finally {
      setPermissionLoading(false);
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
    if (field.type === "tree-select") {
      const isParentField = isTreeMode && field.key === treeParentKey;
      return (
        <TreeSelect
          treeData={isParentField ? parentTreeData : []}
          placeholder={field.placeholder || `请选择${field.label}`}
          allowClear={!field.required}
          treeDefaultExpandAll
          style={{ width: "100%" }}
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
 
  function handleSearch(values: any) {
    setSearchParams(values);
  }
 
  function handleReset() {
    searchForm.resetFields();
    setSearchParams({});
  }

  const columns: TableProps<SystemCrudRecord>["columns"] = [
    ...config.columns.map((column) => ({
      title: column.label,
      dataIndex: column.key,
      key: column.key,
      width: column.width,
      ellipsis: column.ellipsis ?? true,
      render: (value: unknown) => {
        if (config.resource === "menu" && column.key === "menu_type") {
          const menuType = parseNumberValue(value, 0);
          const label = menuTypeLabelMap[menuType] || "-";
          const color = menuType === 1 ? "blue" : menuType === 2 ? "green" : "orange";
          return <Tag color={color}>{label}</Tag>;
        }

        if (column.key === "type") {
          const strValue = String(value);
          if (strValue === "通知" || strValue === "公告") {
            return <Tag color={strValue === "公告" ? "volcano" : "cyan"}>{strValue}</Tag>;
          }
        }
        
        if (column.key === "status") {
          const strValue = String(value);
          const mapping = statusLabelMap[strValue];
          if (mapping) {
            return <Tag color={mapping.color}>{mapping.label}</Tag>;
          }
        }

        if (column.key === "visible") {
          const strValue = String(value);
          const mapping = visibleLabelMap[strValue];
          if (mapping) {
            return <Tag color={mapping.color}>{mapping.label}</Tag>;
          }
        }

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
      render: (_, record) => {
        if (!canUpdate && !canDelete) {
          return "-";
        }

        return (
          <Space size={12}>
            {canUpdate ? (
              <Button
                type="link"
                size="small"
                icon={<EditOutlined />}
                onClick={() => openEditModal(record)}
              >
                编辑
              </Button>
            ) : null}
            {config.resource === "role" && canUpdate ? (
              <Button
                type="link"
                size="small"
                onClick={() => openPermissionDrawer(record)}
              >
                权限
              </Button>
            ) : null}
            {canDelete ? (
              <Popconfirm title="确认删除这条记录吗？" onConfirm={() => handleDelete(record)}>
                <Button type="link" size="small" danger icon={<DeleteOutlined />}>
                  删除
                </Button>
              </Popconfirm>
            ) : null}
          </Space>
        );
      }
    }
  ];

  return (
    <div className="biz-page">
      {messageContext}
      {notificationContext}
      {error ? (
        <Alert
          type="error"
          showIcon
          message={`${config.title}加载失败`}
          description={error}
          style={{ marginBottom: 16 }}
        />
      ) : null}
      {!authLoading && !canView ? (
        <Alert
          type="warning"
          showIcon
          message="无访问权限"
          description={`当前账号缺少 ${permissionConfig.view || "该页面"} 权限`}
          style={{ marginBottom: 16 }}
        />
      ) : null}
      {authLoading || canView ? (
        <Card>
          <div style={{ marginBottom: 16 }}>
            {config.searchFields && config.searchFields.length > 0 ? (
              <Form
                form={searchForm}
                layout="inline"
                onFinish={handleSearch}
                style={{ rowGap: 12 }}
              >
                {config.searchFields.map((field) => (
                  <Form.Item key={field.key} name={field.key} label={field.label}>
                    {renderFormControl(field)}
                  </Form.Item>
                ))}
                <Form.Item>
                  <Space>
                    <Button type="primary" htmlType="submit">
                      查询
                    </Button>
                    <Button onClick={handleReset}>重置</Button>
                  </Space>
                </Form.Item>
                {canCreate ? (
                  <Form.Item style={{ marginLeft: "auto", marginRight: 0 }}>
                    <Button type="primary" icon={<PlusOutlined />} onClick={openCreateModal}>
                      新增
                    </Button>
                  </Form.Item>
                ) : null}
              </Form>
            ) : (
              <Space style={{ width: "100%", justifyContent: "space-between" }}>
                <Space>
                  <Input.Search
                    allowClear
                    placeholder={config.searchPlaceholder}
                    onSearch={(value) => setSearchParams({ keyword: value.trim() })}
                    style={{ width: 280 }}
                  />
                </Space>
                {canCreate ? (
                  <Button type="primary" icon={<PlusOutlined />} onClick={openCreateModal}>
                    新增
                  </Button>
                ) : null}
              </Space>
            )}
          </div>
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
      ) : null}

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

      <Drawer
        title={`分配权限 - ${configuringRole?.role_name || configuringRole?.name || ""}`}
        placement="right"
        width={420}
        onClose={() => setPermissionDrawerOpen(false)}
        open={permissionDrawerOpen}
        extra={
          <Space>
            <Button onClick={() => setPermissionDrawerOpen(false)}>取消</Button>
            <Button type="primary" loading={permissionLoading} onClick={handlePermissionSave}>
              提交
            </Button>
          </Space>
        }
      >
        <Spin spinning={permissionLoading}>
          <Tree
            checkable
            checkStrictly
            treeData={menuTreeData.map(node => {
              const wrapNode = (n: TreeSystemCrudRecord): any => ({
                title: n.menu_name || n.name,
                key: n.id.toString(),
                children: n.children?.map(wrapNode)
              });
              return wrapNode(node);
            })}
            checkedKeys={checkedMenuIds}
            onCheck={(checked) => {
              if (Array.isArray(checked)) {
                setCheckedMenuIds(checked);
              } else {
                setCheckedMenuIds(checked.checked);
              }
            }}
            defaultExpandAll
          />
        </Spin>
      </Drawer>
    </div>
  );
}
