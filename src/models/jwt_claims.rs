use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct JwtClaims
{
    pub(crate) enterfield:String,
    pub(crate) roles:String,
    pub(crate) exp:usize,
}
