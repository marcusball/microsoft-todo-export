
use crate::api::{Date, DateTimeOffset, DateTimeTimeZone};


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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TodoTask {
    pub body: ItemBody,
    pub body_last_modified_date_time: Option<String>,
    pub completed_date_time: Option<DateTimeTimeZone>,
    pub created_date_time: DateTimeOffset,
    pub due_date_time: Option<DateTimeTimeZone>,
    pub id: String,
    pub importance: Importance,
    pub is_reminder_on: bool,
    pub last_modified_date_time: DateTimeOffset,
    pub recurrence: Option<PatternedRecurrence>,
    pub reminder_date_time: Option<DateTimeTimeZone>,
    pub status: TaskStatus,
    pub title: String,

    // TODO: extensions
    // TODO: linkedResources
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TaskStatus {
    NotStarted,
    InProgress,
    Completed,
    WaitingOnOthers,
    Deferred,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemBody {
    pub content: String,
    pub content_type: BodyType,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum BodyType {
    Text,
    Html,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Importance {
    Low,
    Normal,
    High
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PatternedRecurrence {
    pub pattern: RecurrencePattern,
    pub range: RecurrenceRange,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum RecurrencePattern {
    Daily { interval: i32 },
    Weekly { interval: i32, days_of_week: Vec<DayOfWeek>, first_day_of_week: DayOfWeek },
    AbsoluteMonthly { interval: i32, day_of_month: i32 },
    RelativeMonthly { interval: i32, days_of_week: Vec<DayOfWeek> },
    AbsoluteYearly { interval: i32, day_of_month: i32, month: i32 },
    RelativeYearly { interval: i32, days_of_week: Vec<DayOfWeek>, month: i32 }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum RecurrenceRange {
    EndDate { start_date: Date, end_date: Date, recurrence_time_zone: Option<String> },
    NoEnd { start_date: Date, recurrence_time_zone: Option<String> },
    Numbered { start_date: Date, number_of_occurrences: i32, recurrence_time_zone: Option<String> },
}