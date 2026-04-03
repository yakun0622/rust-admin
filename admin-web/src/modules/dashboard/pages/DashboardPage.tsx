import {
  CalendarOutlined,
  CloudServerOutlined,
  DatabaseOutlined,
  HddOutlined,
  ReloadOutlined,
  RiseOutlined,
  ThunderboltOutlined,
  UserOutlined
} from "@ant-design/icons";
import { Alert, Button, Select, Skeleton, Tag } from "antd";
import * as echarts from "echarts";
import type { EChartsOption } from "echarts";
import ReactECharts from "echarts-for-react";
import worldGeoJson from "geojson-world-map";
import { useEffect, useMemo, useState } from "react";
import { useDocumentTitle } from "../../../shared/hooks/useDocumentTitle";
import {
  getDashboardOverview,
  type DashboardOverviewVo
} from "../services/dashboardService";
import "./DashboardPage.css";

type TimeRange = "7d" | "30d" | "90d";

type RegionDistribution = {
  name: string;
  value: number;
  coord: [number, number];
};

const WORLD_MAP_NAME = "world";
const REGION_DISTRIBUTION_DESIGN: RegionDistribution[] = [
  { name: "亚洲", value: 45, coord: [105, 35] },
  { name: "欧洲", value: 28, coord: [15, 50] },
  { name: "北美", value: 22, coord: [-100, 40] },
  { name: "其他", value: 5, coord: [25, -8] }
];

if (!echarts.getMap(WORLD_MAP_NAME)) {
  echarts.registerMap(WORLD_MAP_NAME, worldGeoJson as never);
}

function clamp(value: number, min: number, max: number): number {
  return Math.max(min, Math.min(max, value));
}

function resolveTrendWindow(range: TimeRange): number {
  if (range === "7d") {
    return 7;
  }
  if (range === "30d") {
    return 30;
  }
  return 90;
}

function buildRecentDateLabels(length: number): string[] {
  if (length <= 0) {
    return [];
  }

  const labels: string[] = [];
  for (let idx = 0; idx < length; idx += 1) {
    const date = new Date();
    date.setDate(date.getDate() - (length - idx - 1));
    const month = String(date.getMonth() + 1).padStart(2, "0");
    const day = String(date.getDate()).padStart(2, "0");
    labels.push(`${month}-${day}`);
  }

  return labels;
}

export function DashboardPage() {
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [range, setRange] = useState<TimeRange>("30d");
  const [overview, setOverview] = useState<DashboardOverviewVo | null>(null);

  useDocumentTitle("首页看板 - Rust Admin");

  async function loadOverview() {
    setLoading(true);
    setError(null);
    try {
      const data = await getDashboardOverview();
      setOverview(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : "加载看板数据失败");
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    void loadOverview();
  }, []);

  const trendWindow = useMemo(() => resolveTrendWindow(range), [range]);
  const visitTrendData = useMemo(() => {
    const source = overview?.login_trend ?? [];
    return source.length <= trendWindow ? source : source.slice(-trendWindow);
  }, [overview?.login_trend, trendWindow]);
  const uniqueTrendData = useMemo(() => {
    const source = overview?.action_trend ?? [];
    return source.length <= trendWindow ? source : source.slice(-trendWindow);
  }, [overview?.action_trend, trendWindow]);
  const chartLabels = useMemo(() => buildRecentDateLabels(visitTrendData.length), [visitTrendData.length]);
  const visitTrendSum = useMemo(
    () => visitTrendData.reduce((sum, value) => sum + value, 0),
    [visitTrendData]
  );
  const actionTrendSum = useMemo(
    () => uniqueTrendData.reduce((sum, value) => sum + value, 0),
    [uniqueTrendData]
  );

  const resourceUsage = useMemo(() => {
    if (!overview) {
      return { cpu: 35, memory: 40, disk: 25 };
    }

    const cpuRaw = Math.max(overview.online_users, 1);
    const memoryRaw = Math.max(overview.today_logins, 1);
    const diskRaw = Math.max(overview.today_errors + overview.role_total, 1);
    const rawSum = cpuRaw + memoryRaw + diskRaw;
    const cpu = Math.round((cpuRaw / rawSum) * 100);
    const memory = Math.round((memoryRaw / rawSum) * 100);
    const disk = Math.max(1, 100 - cpu - memory);

    return {
      cpu: clamp(cpu, 10, 70),
      memory: clamp(memory, 10, 75),
      disk: clamp(disk, 5, 60)
    };
  }, [overview]);

  const regionDistribution = useMemo<RegionDistribution[]>(() => REGION_DISTRIBUTION_DESIGN, []);

  const serviceStatus = useMemo(
    () => [
      { name: "Web 服务器", detail: "Nginx 1.24.0", latency: "12ms", status: "运行中" },
      { name: "应用服务器", detail: "Rust 1.75.0", latency: "28ms", status: "运行中" },
      { name: "数据库", detail: "MySQL 8.0", latency: "5ms", status: "运行中" },
      { name: "缓存服务", detail: "Redis 7.2", latency: "3ms", status: "运行中" }
    ],
    []
  );

  const activities = useMemo(
    () => [
      { title: "新用户注册", detail: '用户 "John-Doe" 成功注册', time: "2 分钟前" },
      { title: "系统配置更新", detail: "管理员更新了邮件服务器配置", time: "15 分钟前" },
      { title: "用户登录", detail: '用户 "Alice" 成功登录系统', time: "1 小时前" },
      { title: "数据备份完成", detail: "系统自动备份执行完成", time: "2 小时前" },
      { title: "警告通知", detail: "数据库连接数接近上限", time: "3 小时前" }
    ],
    []
  );

  const trendChartOption = useMemo<EChartsOption>(
    () => ({
      animation: true,
      tooltip: {
        trigger: "axis",
        backgroundColor: "rgba(4, 24, 54, 0.95)",
        borderColor: "rgba(69, 141, 246, 0.35)",
        textStyle: {
          color: "#dbeeff"
        }
      },
      grid: {
        left: 36,
        right: 20,
        top: 24,
        bottom: 30
      },
      xAxis: {
        type: "category",
        boundaryGap: false,
        data: chartLabels,
        axisLine: {
          lineStyle: {
            color: "rgba(96, 146, 226, 0.4)"
          }
        },
        axisLabel: {
          color: "rgba(173, 202, 245, 0.78)",
          fontSize: 11
        }
      },
      yAxis: {
        type: "value",
        axisLine: { show: false },
        axisTick: { show: false },
        axisLabel: {
          color: "rgba(173, 202, 245, 0.78)",
          fontSize: 11
        },
        splitLine: {
          lineStyle: {
            color: "rgba(72, 126, 208, 0.22)",
            type: "dashed"
          }
        }
      },
      series: [
        {
          name: "访问量",
          type: "line",
          smooth: 0.35,
          data: visitTrendData,
          symbol: "circle",
          symbolSize: 6,
          lineStyle: { width: 3, color: "#2caeff" },
          itemStyle: { color: "#2caeff" },
          areaStyle: {
            color: "rgba(44, 174, 255, 0.22)"
          }
        },
        {
          name: "独立访客",
          type: "line",
          smooth: 0.35,
          data: uniqueTrendData,
          symbol: "circle",
          symbolSize: 5,
          lineStyle: { width: 2.2, color: "#7e6bff" },
          itemStyle: { color: "#7e6bff" }
        }
      ]
    }),
    [chartLabels, uniqueTrendData, visitTrendData]
  );

  const resourceChartOption = useMemo<EChartsOption>(
    () => ({
      tooltip: {
        trigger: "item",
        backgroundColor: "rgba(4, 24, 54, 0.95)",
        borderColor: "rgba(69, 141, 246, 0.35)",
        textStyle: {
          color: "#dbeeff"
        }
      },
      legend: {
        show: false
      },
      graphic: [
        {
          type: "text",
          left: "center",
          top: "42%",
          style: {
            text: "资源使用",
            textAlign: "center",
            fill: "rgba(199, 224, 255, 0.9)",
            fontSize: 13
          }
        }
      ],
      series: [
        {
          type: "pie",
          radius: ["62%", "82%"],
          center: ["50%", "50%"],
          avoidLabelOverlap: true,
          label: { show: false },
          labelLine: { show: false },
          itemStyle: {
            borderWidth: 2,
            borderColor: "#081d41"
          },
          data: [
            { name: "CPU", value: resourceUsage.cpu },
            { name: "内存", value: resourceUsage.memory },
            { name: "磁盘", value: resourceUsage.disk }
          ],
          color: ["#2a89ff", "#7b4bff", "#23c77f"]
        }
      ]
    }),
    [resourceUsage.cpu, resourceUsage.disk, resourceUsage.memory]
  );

  const worldMapOption = useMemo<EChartsOption>(() => {
    const hotspotData = regionDistribution.map((item) => ({
      name: item.name,
      value: [item.coord[0], item.coord[1], item.value]
    }));
    const hub = regionDistribution.reduce((max, item) => (item.value > max.value ? item : max), regionDistribution[0]);
    const flyLineData = regionDistribution
      .filter((item) => item.name !== hub.name)
      .map((item) => ({
        fromName: hub.name,
        toName: item.name,
        coords: [hub.coord, item.coord]
      }));

    return {
      animation: true,
      tooltip: {
        trigger: "item",
        backgroundColor: "rgba(4, 24, 54, 0.95)",
        borderColor: "rgba(69, 141, 246, 0.35)",
        textStyle: {
          color: "#dbeeff"
        },
        formatter: (params: unknown) => {
          if (!params || typeof params !== "object") {
            return "";
          }
          const item = params as {
            seriesType?: string;
            name?: string;
            data?: {
              name?: string;
              value?: number | [number, number, number];
              fromName?: string;
              toName?: string;
            };
          };
          if (item.seriesType === "lines") {
            return `${item.data?.fromName ?? ""} → ${item.data?.toName ?? ""}`;
          }
          const value = Array.isArray(item.data?.value) ? item.data.value[2] : item.data?.value ?? 0;
          return `${item.data?.name ?? item.name ?? ""}: ${value}%`;
        }
      },
      geo: {
        map: WORLD_MAP_NAME,
        roam: false,
        zoom: 1.1,
        left: 0,
        right: 0,
        top: 0,
        bottom: 0,
        itemStyle: {
          areaColor: "rgba(19, 62, 124, 0.65)",
          borderColor: "rgba(127, 186, 255, 0.45)",
          borderWidth: 0.8
        },
        emphasis: {
          itemStyle: {
            areaColor: "rgba(45, 110, 196, 0.72)"
          },
          label: {
            show: false
          }
        },
        select: {
          disabled: true
        },
        silent: true
      },
      series: [
        {
          type: "lines",
          coordinateSystem: "geo",
          data: flyLineData,
          zlevel: 2,
          effect: {
            show: true,
            period: 5,
            trailLength: 0.22,
            symbol: "arrow",
            symbolSize: 6,
            color: "#8fe5ff"
          },
          lineStyle: {
            color: "rgba(120, 187, 255, 0.7)",
            width: 1.2,
            curveness: 0.25,
            opacity: 0.68
          },
          silent: true
        },
        {
          type: "effectScatter",
          coordinateSystem: "geo",
          data: hotspotData,
          symbolSize: (value: number[]) => 9 + Number(value[2]) * 0.24,
          rippleEffect: {
            scale: 3.2,
            brushType: "stroke"
          },
          itemStyle: {
            color: "#66c6ff",
            shadowBlur: 18,
            shadowColor: "rgba(102, 198, 255, 0.95)",
            borderWidth: 1,
            borderColor: "rgba(220, 245, 255, 0.9)"
          },
          emphasis: {
            scale: true
          }
        }
      ]
    };
  }, [regionDistribution]);

  const rangeOptions = [
    { label: "最近 7 天", value: "7d" },
    { label: "最近 30 天", value: "30d" },
    { label: "最近 90 天", value: "90d" }
  ];

  return (
    <div className="dashboard-page">
      <section className="dashboard-welcome">
        <div>
          <h1>欢迎回来, Admin!</h1>
          <p>今天是 {new Date().toLocaleDateString("zh-CN")}，系统运行状态良好</p>
        </div>
        <div className="dashboard-welcome__actions">
          <Button icon={<ReloadOutlined />} onClick={() => void loadOverview()} loading={loading}>
            刷新
          </Button>
          <Select<TimeRange>
            value={range}
            onChange={(value) => setRange(value)}
            options={rangeOptions}
            suffixIcon={<CalendarOutlined />}
          />
        </div>
      </section>

      {error ? (
        <Alert
          type="warning"
          showIcon
          message="看板数据加载失败"
          description={error}
          className="dashboard-alert"
        />
      ) : null}

      {loading && !overview ? <Skeleton active paragraph={{ rows: 10 }} /> : null}

      {overview ? (
        <>
          <section className="dashboard-metrics-grid">
            <article className="dashboard-metric-card">
              <div className="dashboard-metric-card__label">总用户数</div>
              <div className="dashboard-metric-card__body">
                <div className="dashboard-metric-card__icon is-blue">
                  <UserOutlined />
                </div>
                <div className="dashboard-metric-card__content">
                  <div className="dashboard-metric-card__value-row">
                    <span className="dashboard-metric-card__value">{overview.admin_total}</span>
                    <Tag color="processing">+12.5%</Tag>
                  </div>
                  <div className="dashboard-metric-card__desc">较上月增长 140 人</div>
                </div>
              </div>
            </article>

            <article className="dashboard-metric-card">
              <div className="dashboard-metric-card__label">服务器状态</div>
              <div className="dashboard-metric-card__body">
                <div className="dashboard-metric-card__icon is-cyan">
                  <CloudServerOutlined />
                </div>
                <div className="dashboard-metric-card__content">
                  <div className="dashboard-metric-card__value-row">
                    <span className="dashboard-metric-card__value">8 / 8</span>
                    <Tag color="success">全部在线</Tag>
                  </div>
                  <div className="dashboard-metric-card__desc">在线率 100%</div>
                </div>
              </div>
            </article>

            <article className="dashboard-metric-card">
              <div className="dashboard-metric-card__label">CPU 使用率</div>
              <div className="dashboard-metric-card__body">
                <div className="dashboard-metric-card__icon is-indigo">
                  <ThunderboltOutlined />
                </div>
                <div className="dashboard-metric-card__content">
                  <div className="dashboard-metric-card__value-row">
                    <span className="dashboard-metric-card__value">{resourceUsage.cpu}%</span>
                    <Tag color="processing">+5.2%</Tag>
                  </div>
                  <div className="dashboard-metric-card__desc">较昨日下降 1.1%</div>
                </div>
              </div>
            </article>

            <article className="dashboard-metric-card">
              <div className="dashboard-metric-card__label">内存使用率</div>
              <div className="dashboard-metric-card__body">
                <div className="dashboard-metric-card__icon is-teal">
                  <HddOutlined />
                </div>
                <div className="dashboard-metric-card__content">
                  <div className="dashboard-metric-card__value-row">
                    <span className="dashboard-metric-card__value">{resourceUsage.memory}%</span>
                    <Tag color="error">+3.1%</Tag>
                  </div>
                  <div className="dashboard-metric-card__desc">较昨日上升 3.1%</div>
                </div>
              </div>
            </article>
          </section>

          <section className="dashboard-main-grid">
            <article className="dashboard-card">
              <div className="dashboard-card__header">
                <div>
                  <h3>访问趋势</h3>
                  <p>近 {rangeOptions.find((item) => item.value === range)?.label ?? "最近 30 天"} 访问统计</p>
                </div>
                <Button type="text" icon={<RiseOutlined />}>
                  查看详情
                </Button>
              </div>

              <div className="dashboard-trend-legends">
                <div>
                  <span className="dot is-blue" />
                  访问量 ({visitTrendSum})
                </div>
                <div>
                  <span className="dot is-purple" />
                  独立访客 ({actionTrendSum})
                </div>
              </div>

              <div className="dashboard-trend-chart">
                <ReactECharts
                  option={trendChartOption}
                  className="dashboard-echart"
                  notMerge
                  lazyUpdate
                />
              </div>
            </article>

            <article className="dashboard-card">
              <div className="dashboard-card__header">
                <div>
                  <h3>服务器状态</h3>
                  <p>核心组件运行监测</p>
                </div>
                <Button type="text">查看详情</Button>
              </div>

              <div className="dashboard-service-list">
                {serviceStatus.map((service) => (
                  <div className="dashboard-service-item" key={service.name}>
                    <div>
                      <div className="dashboard-service-item__name">{service.name}</div>
                      <div className="dashboard-service-item__detail">{service.detail}</div>
                    </div>
                    <div className="dashboard-service-item__status">
                      <Tag color="success">{service.status}</Tag>
                      <span>{service.latency}</span>
                    </div>
                  </div>
                ))}
              </div>
            </article>
          </section>

          <section className="dashboard-bottom-grid">
            <article className="dashboard-card">
              <div className="dashboard-card__header">
                <div>
                  <h3>系统资源</h3>
                  <p>当前系统资源使用情况</p>
                </div>
              </div>

              <div className="dashboard-resource-chart-wrap">
                <div className="dashboard-resource-chart">
                  <ReactECharts
                    option={resourceChartOption}
                    className="dashboard-echart"
                    notMerge
                    lazyUpdate
                  />
                </div>
              </div>

              <div className="dashboard-resource-legend">
                <div>
                  <span className="dot is-blue" />
                  CPU {resourceUsage.cpu}%
                </div>
                <div>
                  <span className="dot is-purple" />
                  内存 {resourceUsage.memory}%
                </div>
                <div>
                  <span className="dot is-green" />
                  磁盘 {resourceUsage.disk}%
                </div>
              </div>
            </article>

            <article className="dashboard-card">
              <div className="dashboard-card__header">
                <div>
                  <h3>全球访问分布</h3>
                  <p>用户访问地域分布情况</p>
                </div>
              </div>

              <div className="dashboard-world-map">
                <ReactECharts
                  option={worldMapOption}
                  className="dashboard-echart"
                  notMerge
                  lazyUpdate
                />
              </div>

              <div className="dashboard-region-stats">
                {regionDistribution.map((item) => (
                  <div className="dashboard-region-stat" key={item.name}>
                    <div className="dashboard-region-stat__name">{item.name}</div>
                    <div className="dashboard-region-stat__value">{item.value}%</div>
                  </div>
                ))}
              </div>
            </article>

            <article className="dashboard-card">
              <div className="dashboard-card__header">
                <div>
                  <h3>最近活动</h3>
                  <p>系统事件和用户行为</p>
                </div>
                <Button type="text">查看全部</Button>
              </div>

              <div className="dashboard-activity-list">
                {activities.map((activity) => (
                  <div className="dashboard-activity-item" key={activity.title}>
                    <div className="dashboard-activity-item__icon">
                      <DatabaseOutlined />
                    </div>
                    <div className="dashboard-activity-item__content">
                      <div className="dashboard-activity-item__title">{activity.title}</div>
                      <div className="dashboard-activity-item__detail">{activity.detail}</div>
                    </div>
                    <div className="dashboard-activity-item__time">{activity.time}</div>
                  </div>
                ))}
              </div>
            </article>
          </section>
        </>
      ) : null}
    </div>
  );
}
