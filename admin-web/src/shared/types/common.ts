export type PageResult<T> = {
  list: T[];
  pageNum: number;
  pageSize: number;
  total: number;
};
