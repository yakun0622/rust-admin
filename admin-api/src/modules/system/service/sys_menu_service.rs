use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use async_trait::async_trait;
use shaku::{Component, Interface};

use crate::core::{
    converter::sys_menu_converter::{from_create_dto, from_update_dto, to_sys_menu_vo},
    dto::sys_menu_dto::{SysMenuCreateReqDto, SysMenuUpdateReqDto},
    errors::AppError,
    model::sys_menu::SysMenuModel,
    vo::sys_menu_vo::{SysMenuListVo, SysMenuVo},
};
use crate::modules::system::repository::ISysMenuRepository;

#[async_trait]
pub trait ISysMenuService: Interface {
    async fn list(&self, keyword: Option<&str>) -> Result<SysMenuListVo, AppError>;
    async fn list_tree(&self, keyword: Option<&str>) -> Result<SysMenuListVo, AppError>;
    async fn create(&self, dto: SysMenuCreateReqDto) -> Result<SysMenuVo, AppError>;
    async fn update_by_id(&self, id: u64, dto: SysMenuUpdateReqDto) -> Result<SysMenuVo, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}

#[derive(Component, Clone)]
#[shaku(interface = ISysMenuService)]
pub struct SysMenuService {
    #[shaku(inject)]
    repo: Arc<dyn ISysMenuRepository>,
}

impl SysMenuService {
    pub async fn list(&self, keyword: Option<&str>) -> Result<SysMenuListVo, AppError> {
        let items = self
            .repo
            .list(keyword.and_then(normalize_optional_str))
            .await?
            .into_iter()
            .map(to_sys_menu_vo)
            .collect::<Vec<_>>();

        Ok(SysMenuListVo {
            total: items.len(),
            items,
        })
    }

    pub async fn list_tree(&self, keyword: Option<&str>) -> Result<SysMenuListVo, AppError> {
        let flat_items = self
            .repo
            .list(keyword.and_then(normalize_optional_str))
            .await?
            .into_iter()
            .map(to_sys_menu_vo)
            .collect::<Vec<_>>();
        let total = flat_items.len();
        let items = build_menu_tree(flat_items);

        Ok(SysMenuListVo { total, items })
    }

    pub async fn create(&self, dto: SysMenuCreateReqDto) -> Result<SysMenuVo, AppError> {
        let model = from_create_dto(dto)?;
        let id = self.repo.insert(&model).await?;
        let created = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| AppError::internal(format!("创建成功但读取菜单失败: id={id}")))?;
        Ok(to_sys_menu_vo(created))
    }

    pub async fn update_by_id(
        &self,
        id: u64,
        dto: SysMenuUpdateReqDto,
    ) -> Result<SysMenuVo, AppError> {
        let current = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found(format!("menu 资源中不存在 id={id} 的记录")))?;

        let parent_id = dto.parent_id.unwrap_or(current.parent_id);
        if parent_id == id {
            return Err(AppError::bad_request("上级菜单不能为自身"));
        }

        let model = from_update_dto(
            SysMenuModel {
                parent_id,
                ..current
            },
            dto,
        )?;
        let affected = self.repo.update_by_id(id, &model).await?;
        if !affected {
            return Err(AppError::not_found(format!(
                "menu 资源中不存在 id={id} 的记录"
            )));
        }

        let updated = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| AppError::internal(format!("更新成功但读取菜单失败: id={id}")))?;
        Ok(to_sys_menu_vo(updated))
    }

    pub async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        let deleted = self.repo.delete_by_id(id).await?;
        if !deleted {
            return Err(AppError::not_found(format!(
                "menu 资源中不存在 id={id} 的记录"
            )));
        }
        Ok(true)
    }
}

#[async_trait]
impl ISysMenuService for SysMenuService {
    async fn list(&self, keyword: Option<&str>) -> Result<SysMenuListVo, AppError> {
        self.list(keyword).await
    }

    async fn list_tree(&self, keyword: Option<&str>) -> Result<SysMenuListVo, AppError> {
        self.list_tree(keyword).await
    }

    async fn create(&self, dto: SysMenuCreateReqDto) -> Result<SysMenuVo, AppError> {
        self.create(dto).await
    }

    async fn update_by_id(&self, id: u64, dto: SysMenuUpdateReqDto) -> Result<SysMenuVo, AppError> {
        self.update_by_id(id, dto).await
    }

    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        self.delete_by_id(id).await
    }
}

fn normalize_optional_str(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

fn build_menu_tree(items: Vec<SysMenuVo>) -> Vec<SysMenuVo> {
    if items.is_empty() {
        return Vec::new();
    }

    let mut index_by_id: HashMap<u64, usize> = HashMap::new();
    for (index, item) in items.iter().enumerate() {
        index_by_id.insert(item.id, index);
    }

    let mut children_map: HashMap<u64, Vec<usize>> = HashMap::new();
    let mut root_indices = Vec::new();
    for (index, item) in items.iter().enumerate() {
        if item.parent_id == 0
            || !index_by_id.contains_key(&item.parent_id)
            || item.parent_id == item.id
        {
            root_indices.push(index);
        } else {
            children_map.entry(item.parent_id).or_default().push(index);
        }
    }

    let mut visited = HashSet::new();
    root_indices
        .into_iter()
        .map(|index| build_menu_tree_node(index, &items, &children_map, &mut visited))
        .collect()
}

fn build_menu_tree_node(
    index: usize,
    items: &[SysMenuVo],
    children_map: &HashMap<u64, Vec<usize>>,
    visited: &mut HashSet<u64>,
) -> SysMenuVo {
    let mut node = items[index].clone();
    if !visited.insert(node.id) {
        return node;
    }

    if let Some(children_indices) = children_map.get(&node.id) {
        node.children = children_indices
            .iter()
            .map(|child_index| build_menu_tree_node(*child_index, items, children_map, visited))
            .collect();
    }

    visited.remove(&node.id);
    node
}
