pub mod auth;
pub mod dashboard;

pub trait Converter<Source, Target> {
    fn convert(source: Source) -> Target;
}
