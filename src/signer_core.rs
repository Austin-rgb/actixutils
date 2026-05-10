use anyhow::Result;
use std::sync::Arc;

pub trait Sign<T>: Send + Sync + 'static {
    fn sign(&self, claims: &T) -> Result<String>;
}

pub trait Validate<T>: Send + Sync + 'static {
    fn validate(&self, token: &str) -> Result<T>;
}

pub trait ValidateProvider<T>: Send + Sync + 'static {
    fn provide(&self) -> Arc<dyn Validate<T>>;
}
