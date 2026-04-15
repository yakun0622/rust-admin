pub mod auth_converter;
pub mod dashboard_converter;
pub mod sys_config_converter;
pub mod sys_dept_converter;
pub mod sys_dict_converter;
pub mod sys_menu_converter;
pub mod sys_post_converter;
pub mod sys_role_converter;
pub mod sys_user_converter;

pub trait Converter<Source, Target> {
    fn convert(source: Source) -> Target;
}
