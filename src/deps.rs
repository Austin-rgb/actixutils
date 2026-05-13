use std::sync::Arc;

use crate::{HS256Signer, RS256Signer, RS256Validator, Sign, Validate};
use ferrumec::di::{AsyncFromEnv, EnvContext, EnvError};
use serde::{Deserialize, Serialize};

impl<T> AsyncFromEnv for OrphanWrapper<Arc<dyn Validate<T>>>
where
    T: for<'a> Deserialize<'a> + 'static,
{
    async fn from_env(ctx: &EnvContext) -> Result<Self, EnvError> {
        let signer_type = ctx.get("validate.type")?;
        match signer_type {
            "hs256" => Ok(OrphanWrapper(Arc::new(HS256Signer::new(
                ctx.get("validate.aud")?.to_owned(),
                ctx.get("validate.secret")?.to_owned(),
            )) as Arc<dyn Validate<T>>)),
            "rs256" => Ok(OrphanWrapper(Arc::new(RS256Validator::new(
                ctx.get("validate.public_key")?.to_string(),
                ctx.get("validate.aud")?.to_string(),
            )) as Arc<dyn Validate<T>>)),
            _ => Err(EnvError::new(format!(
                "Unsupported validate.type value: {signer_type}"
            ))),
        }
    }
}

impl<T> AsyncFromEnv for OrphanWrapper<Arc<dyn Sign<T>>>
where
    T: Serialize + 'static,
{
    async fn from_env(ctx: &EnvContext) -> Result<Self, EnvError> {
        let signer_type = ctx.get("sign.type")?;
        let aud = ctx.get("sign.aud")?.to_string();
        match signer_type {
            "hs256" => Ok(OrphanWrapper(Arc::new(HS256Signer::new(
                aud,
                ctx.get("validate.secret")?.to_owned(),
            )) as Arc<dyn Sign<T>>)),
            "rs256" => Ok(OrphanWrapper(Arc::new(RS256Signer::new(
                ctx.get("sign.private_key")?.to_string(),
                aud,
            )) as Arc<dyn Sign<T>>)),
            _ => Err(EnvError::new(format!(
                "Unsupported sign.type value: {signer_type}"
            ))),
        }
    }
}

pub struct OrphanWrapper<T>(pub T);
