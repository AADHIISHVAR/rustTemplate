use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct JwtClaims
{
    pub(crate) id:u32,
    pub(crate) email_or_username:String,
    pub(crate) roles:String,
    pub(crate) exp:usize,
    
}
