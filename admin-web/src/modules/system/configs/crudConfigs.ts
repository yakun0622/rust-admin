export type CrudSelectOption = {
  label: string;
  value: string | number;
};

export type CrudFieldType = "input" | "textarea" | "select" | "number" | "tree-select";

export type CrudFieldConfig = {
  key: string;
  label: string;
  type?: CrudFieldType;
  required?: boolean;
  createOnly?: boolean;
  editOnly?: boolean;
  placeholder?: string;
  options?: CrudSelectOption[];
  defaultValue?: string | number;
};

export type CrudColumnConfig = {
  key: string;
  label: string;
  width?: number;
  ellipsis?: boolean;
};

export type CrudTreeConfig = {
  enabled: boolean;
  parentKey?: string;
  parentRootLabel?: string;
  expandAllByDefault?: boolean;
};

export type SystemCrudPageConfig = {
  resource: string;
  title: string;
  description: string;
  searchPlaceholder: string;
  columns: CrudColumnConfig[];
  fields: CrudFieldConfig[];
  searchFields?: CrudFieldConfig[];
  tree?: CrudTreeConfig;
  permissions?: {
    view?: string;
    create?: string;
    update?: string;
    delete?: string;
  };
};

const statusOptions: CrudSelectOption[] = [
  { label: "启用", value: "enabled" },
  { label: "停用", value: "disabled" }
];

export const systemCrudConfigs = {
  user: {
    resource: "user",
    title: "用户管理",
    description: "维护系统用户账号、联系方式和状态。",
    searchPlaceholder: "按用户名、昵称搜索",
    permissions: {
      view: "system:user:view",
      create: "system:user:create",
      update: "system:user:update",
      delete: "system:user:delete"
    },
    columns: [
      { key: "id", label: "ID", width: 80 },
      { key: "username", label: "用户名", width: 140 },
      { key: "nickname", label: "昵称", width: 160 },
      { key: "phone", label: "手机号", width: 160 },
      { key: "status", label: "状态", width: 120 }
    ],
    fields: [
      { key: "username", label: "用户名", required: true },
      { key: "nickname", label: "昵称", required: true },
      { key: "phone", label: "手机号", required: true },
      {
        key: "status",
        label: "状态",
        type: "select",
        required: true,
        options: statusOptions,
        defaultValue: "enabled"
      }
    ],
    searchFields: [
      { key: "username", label: "用户名" },
      { key: "nickname", label: "昵称" },
      { key: "phone", label: "手机号" },
      { key: "status", label: "状态", type: "select", options: statusOptions }
    ]
  },
  role: {
    resource: "role",
    title: "角色管理",
    description: "维护角色权限标识和展示顺序。",
    searchPlaceholder: "按角色名、标识搜索",
    permissions: {
      view: "system:role:view",
      create: "system:role:create",
      update: "system:role:update",
      delete: "system:role:delete"
    },
    columns: [
      { key: "id", label: "ID", width: 80 },
      { key: "name", label: "角色名称", width: 180 },
      { key: "key", label: "权限标识", width: 180 },
      { key: "sort", label: "排序", width: 120 },
      { key: "status", label: "状态", width: 120 }
    ],
    fields: [
      { key: "name", label: "角色名称", required: true },
      { key: "key", label: "权限标识", required: true },
      { key: "sort", label: "排序", type: "number", required: true, defaultValue: 1 },
      {
        key: "status",
        label: "状态",
        type: "select",
        required: true,
        options: statusOptions,
        defaultValue: "enabled"
      }
    ],
    searchFields: [
      { key: "name", label: "角色名称" },
      { key: "key", label: "权限标识" },
      { key: "status", label: "状态", type: "select", options: statusOptions }
    ]
  },
  menu: {
    resource: "menu",
    title: "菜单管理",
    description: "维护菜单类型、路由、组件和权限标识。",
    searchPlaceholder: "按菜单名、路由、权限标识搜索",
    permissions: {
      view: "system:menu:view",
      create: "system:menu:create",
      update: "system:menu:update",
      delete: "system:menu:delete"
    },
    tree: {
      enabled: true,
      parentKey: "parent_id",
      parentRootLabel: "顶级菜单",
      expandAllByDefault: true
    },
    columns: [
      { key: "id", label: "ID", width: 80 },
      { key: "menu_type", label: "类型", width: 90 },
      { key: "name", label: "菜单名称", width: 180 },
      { key: "path", label: "路由", width: 180 },
      { key: "component", label: "组件", width: 180 },
      { key: "permission", label: "权限标识", width: 220 },
      { key: "status", label: "状态", width: 120 },
      { key: "visible", label: "是否可见", width: 120 }
    ],
    fields: [
      {
        key: "parent_id",
        label: "上级菜单",
        type: "tree-select",
        required: true,
        defaultValue: 0
      },
      {
        key: "menu_type",
        label: "菜单类型",
        type: "select",
        required: true,
        options: [
          { label: "目录", value: 1 },
          { label: "菜单", value: 2 },
          { label: "按钮", value: 3 }
        ],
        defaultValue: 2
      },
      { key: "name", label: "菜单名称", required: true },
      { key: "route_name", label: "路由名称" },
      { key: "path", label: "路由地址" },
      { key: "component", label: "组件名" },
      { key: "permission", label: "权限标识" },
      { key: "icon", label: "菜单图标" },
      { key: "order_num", label: "显示排序", type: "number", defaultValue: 1 },
      {
        key: "status",
        label: "状态",
        type: "select",
        required: true,
        options: statusOptions,
        defaultValue: "enabled"
      },
      {
        key: "visible",
        label: "是否可见",
        type: "select",
        required: true,
        options: [
          { label: "是", value: "yes" },
          { label: "否", value: "no" }
        ],
        defaultValue: "yes"
      }
    ],
    searchFields: [
      { key: "name", label: "菜单名称" },
      { key: "status", label: "状态", type: "select", options: statusOptions }
    ]
  },
  dept: {
    resource: "dept",
    title: "部门管理",
    description: "维护部门负责人和联系电话。",
    searchPlaceholder: "按部门名、负责人搜索",
    permissions: {
      view: "system:dept:view",
      create: "system:dept:create",
      update: "system:dept:update",
      delete: "system:dept:delete"
    },
    tree: {
      enabled: true,
      parentKey: "parent_id",
      parentRootLabel: "顶级部门",
      expandAllByDefault: true
    },
    columns: [
      { key: "id", label: "ID", width: 80 },
      { key: "name", label: "部门名称", width: 180 },
      { key: "leader", label: "负责人", width: 160 },
      { key: "phone", label: "联系电话", width: 160 },
      { key: "status", label: "状态", width: 120 }
    ],
    fields: [
      {
        key: "parent_id",
        label: "上级部门",
        type: "tree-select",
        required: true,
        createOnly: true,
        defaultValue: 0
      },
      { key: "name", label: "部门名称", required: true },
      { key: "leader", label: "负责人", required: true },
      { key: "phone", label: "联系电话", required: true },
      {
        key: "status",
        label: "状态",
        type: "select",
        required: true,
        options: statusOptions,
        defaultValue: "enabled"
      }
    ],
    searchFields: [
      { key: "name", label: "部门名称" },
      { key: "status", label: "状态", type: "select", options: statusOptions }
    ]
  },
  post: {
    resource: "post",
    title: "岗位管理",
    description: "维护岗位编码、排序与启用状态。",
    searchPlaceholder: "按岗位名、编码搜索",
    permissions: {
      view: "system:post:view",
      create: "system:post:create",
      update: "system:post:update",
      delete: "system:post:delete"
    },
    columns: [
      { key: "id", label: "ID", width: 80 },
      { key: "name", label: "岗位名称", width: 180 },
      { key: "code", label: "岗位编码", width: 180 },
      { key: "sort", label: "排序", width: 120 },
      { key: "status", label: "状态", width: 120 }
    ],
    fields: [
      { key: "name", label: "岗位名称", required: true },
      { key: "code", label: "岗位编码", required: true },
      { key: "sort", label: "排序", type: "number", required: true, defaultValue: 1 },
      {
        key: "status",
        label: "状态",
        type: "select",
        required: true,
        options: statusOptions,
        defaultValue: "enabled"
      }
    ],
    searchFields: [
      { key: "name", label: "岗位名称" },
      { key: "code", label: "岗位编码" },
      { key: "status", label: "状态", type: "select", options: statusOptions }
    ]
  },
  dict: {
    resource: "dict",
    title: "字典管理",
    description: "维护字典类型、标签和值。",
    searchPlaceholder: "按字典类型、标签搜索",
    permissions: {
      view: "system:dict:view",
      create: "system:dict:create",
      update: "system:dict:update",
      delete: "system:dict:delete"
    },
    columns: [
      { key: "id", label: "ID", width: 80 },
      { key: "type", label: "字典类型", width: 180 },
      { key: "label", label: "字典标签", width: 180 },
      { key: "value", label: "字典值", width: 160 },
      { key: "status", label: "状态", width: 120 }
    ],
    fields: [
      { key: "type", label: "字典类型", required: true },
      { key: "label", label: "字典标签", required: true },
      { key: "value", label: "字典值", required: true },
      {
        key: "status",
        label: "状态",
        type: "select",
        required: true,
        options: statusOptions,
        defaultValue: "enabled"
      }
    ],
    searchFields: [
      { key: "dict_type", label: "字典类型" },
      { key: "dict_label", label: "字典标签" },
      { key: "status", label: "状态", type: "select", options: statusOptions }
    ]
  },
  config: {
    resource: "config",
    title: "参数设置",
    description: "维护系统运行参数和值。",
    searchPlaceholder: "按参数名、参数值搜索",
    permissions: {
      view: "system:config:view",
      create: "system:config:create",
      update: "system:config:update",
      delete: "system:config:delete"
    },
    columns: [
      { key: "id", label: "ID", width: 80 },
      { key: "name", label: "参数名称", width: 220 },
      { key: "value", label: "参数值", width: 220 },
      { key: "remark", label: "备注", width: 240 },
      { key: "status", label: "状态", width: 120 }
    ],
    fields: [
      { key: "name", label: "参数名称", required: true },
      { key: "value", label: "参数值", required: true },
      { key: "remark", label: "备注", type: "textarea" },
      {
        key: "status",
        label: "状态",
        type: "select",
        required: true,
        options: statusOptions,
        defaultValue: "enabled"
      }
    ],
    searchFields: [
      { key: "name", label: "参数名称" },
      { key: "key", label: "参数键名" },
      { key: "status", label: "状态", type: "select", options: statusOptions }
    ]
  },
  notice: {
    resource: "notice",
    title: "通知公告",
    description: "维护公告标题、发布状态和发布人。",
    searchPlaceholder: "按标题、发布人搜索",
    permissions: {
      view: "system:notice:view",
      create: "system:notice:create",
      update: "system:notice:update",
      delete: "system:notice:delete"
    },
    columns: [
      { key: "id", label: "ID", width: 80 },
      { key: "title", label: "标题", width: 260 },
      { key: "type", label: "类型", width: 140 },
      { key: "status", label: "状态", width: 120 },
      { key: "publisher", label: "发布人", width: 160 }
    ],
    fields: [
      { key: "title", label: "标题", required: true },
      {
        key: "type",
        label: "类型",
        type: "select",
        required: true,
        options: [
          { label: "公告", value: "公告" },
          { label: "通知", value: "通知" }
        ],
        defaultValue: "公告"
      },
      {
        key: "status",
        label: "状态",
        type: "select",
        required: true,
        options: [
          { label: "已发布", value: "published" },
          { label: "草稿", value: "draft" }
        ],
        defaultValue: "draft"
      },
      { key: "publisher", label: "发布人", required: true }
    ],
    searchFields: [
      { key: "title", label: "公告标题" },
      {
        key: "notice_type",
        label: "公告类型",
        type: "select",
        options: [
          { label: "通知", value: "1" },
          { label: "公告", value: "2" }
        ]
      },
      {
        key: "status",
        label: "公告状态",
        type: "select",
        options: [
          { label: "草稿", value: "0" },
          { label: "已发布", value: "1" },
          { label: "已下线", value: "2" }
        ]
      }
    ]
  }
} satisfies Record<string, SystemCrudPageConfig>;
