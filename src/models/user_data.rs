
#[derive(::serde::Serialize, ::serde::Deserialize, Debug)]
pub struct UserData {
    pub user_name: String,
    pub tasks: Vec<crate::task_data::TaskData>,
}
