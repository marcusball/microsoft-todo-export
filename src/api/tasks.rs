
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TodoTaskListCollection {
    pub value: Vec<TodoTaskList>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TodoTaskList {
    pub display_name: String,

    pub id: String,

    pub is_owner: bool,

    pub is_shared: bool,

    pub wellknown_list_name: WellknownListName,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum WellknownListName {
    None,
    DefaultList,
    FlaggedEmails,
    UnknownFutureValue,
}