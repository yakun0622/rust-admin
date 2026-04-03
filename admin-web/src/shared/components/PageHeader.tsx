import { Typography } from "antd";

type PageHeaderProps = {
  title: string;
  description?: string;
};

export function PageHeader({ title, description }: PageHeaderProps) {
  return (
    <div className="page-header" style={{ marginBottom: 16 }}>
      <Typography.Title level={4} className="page-header__title" style={{ margin: 0 }}>
        {title}
      </Typography.Title>
      {description ? (
        <Typography.Paragraph type="secondary" className="page-header__desc" style={{ marginBottom: 0 }}>
          {description}
        </Typography.Paragraph>
      ) : null}
    </div>
  );
}
