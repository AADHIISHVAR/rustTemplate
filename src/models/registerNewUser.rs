use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]

pub struct RegisterUser
{
    #[serde(default)]
    pub id: i32,

    pub name: String,
    pub password: String,
    pub email: String,
    pub username: String
}

