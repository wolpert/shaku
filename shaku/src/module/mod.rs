//! This module handles building and resolving services.

mod module_build_context;
mod module_builder;
mod module_traits;

pub use self::module_build_context::ModuleBuildContext;
pub use self::module_builder::ModuleBuilder;
pub use self::module_traits::{Module, ModuleInterface};

#[cfg(not(feature = "thread_safe"))]
type AnyType = dyn anymap2::any::Any;
#[cfg(feature = "thread_safe")]
type AnyType = dyn anymap2::any::Any + Send + Sync;

#[cfg(not(feature = "thread_safe"))]
type ParamAnyType = dyn anymap2::any::Any;
#[cfg(feature = "thread_safe")]
type ParamAnyType = dyn anymap2::any::Any + Send;

type ComponentMap = anymap2::Map<AnyType>;
type ParameterMap = anymap2::Map<ParamAnyType>;
