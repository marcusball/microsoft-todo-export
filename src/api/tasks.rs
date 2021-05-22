
use crate::api::{Date, DateTimeOffset, DateTimeTimeZone};

/// Represents a single Todo List. 
/// 
/// See: https://docs.microsoft.com/en-us/graph/api/resources/todotasklist?view=graph-rest-1.0#properties
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TodoTaskList {
    /// The name of the task list.
    pub display_name: String,

    /// The identifier of the task list, unique in the user's mailbox. 
    /// Read-only. Inherited from entity
    pub id: String,

    /// True if the user is owner of the given task list.
    pub is_owner: bool,

    /// True if the task list is shared with other users
    pub is_shared: bool,

    /// Property indicating the list name if the given list is a well-known list. 
    pub wellknown_list_name: WellknownListName,
}

/// The possible values of a `TodoTaskList` `wellknown_list_name`. 
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum WellknownListName {
    None,
    DefaultList,
    FlaggedEmails,
    UnknownFutureValue,
}

/// Represents a single Task within a todo list. 
/// 
/// See: https://docs.microsoft.com/en-us/graph/api/resources/todotask?view=graph-rest-1.0
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TodoTask {
    /// The task body that typically contains information about the task.
    pub body: ItemBody,

    /// The date and time when the task was last modified. By default, it is in UTC. 
    /// You can provide a custom time zone in the request header. 
    /// The property value uses ISO 8601 format and is always in UTC time. 
    /// For example, midnight UTC on Jan 1, 2020 would look like this: '2020-01-01T00:00:00Z'.
    pub body_last_modified_date_time: Option<String>,

    /// The date in the specified time zone that the task was finished.
    pub completed_date_time: Option<DateTimeTimeZone>,

    /// The date and time when the task was created. By default, it is in UTC.
    pub created_date_time: DateTimeOffset,

    /// The date in the specified time zone that the task is to be finished.
    pub due_date_time: Option<DateTimeTimeZone>,

    /// Unique identifier for the task. By default, this value changes when the item is moved from one list to another.
    pub id: String,

    /// The importance of the task.
    pub importance: Importance,

    /// Set to true if an alert is set to remind the user of the task.
    pub is_reminder_on: bool,

    /// The date and time when the task was last modified. By default, it is in UTC. 
    pub last_modified_date_time: DateTimeOffset,

    /// The recurrence pattern for the task.
    pub recurrence: Option<PatternedRecurrence>,

    /// The date and time for a reminder alert of the task to occur.
    pub reminder_date_time: Option<DateTimeTimeZone>,

    /// Indicates the state or progress of the task. 
    pub status: TaskStatus,

    /// A brief description of the task.
    pub title: String,

    // TODO: extensions
    // TODO: linkedResources
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TaskStatus {
    NotStarted,
    InProgress,
    Completed,
    WaitingOnOthers,
    Deferred,
}

/// Represents properties of the body of an item, such as a message, event or group post.
/// 
/// See: https://docs.microsoft.com/en-us/graph/api/resources/itembody?view=graph-rest-1.0
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemBody {
    /// The content of the item.
    pub content: String,

    /// The type of the content.
    pub content_type: BodyType,
}

/// The possible values of `content_type` for an `ItemBody`.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum BodyType {
    Text,
    Html,
}

/// The possible `importance` values for a `TodoTask`. 
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Importance {
    Low,
    Normal,
    High
}

/// The recurrence pattern and range for a `TodoTask`. 
/// 
/// See: https://docs.microsoft.com/en-us/graph/api/resources/patternedrecurrence?view=graph-rest-1.0
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PatternedRecurrence {
    /// The frequency of an event.
    pub pattern: RecurrencePattern,

    /// The duration of an event.
    pub range: RecurrenceRange,
}

/// Describes the frequency by which a recurrning `TodoTask` repeats. 
/// 
/// See: https://docs.microsoft.com/en-us/graph/api/resources/recurrencepattern?view=graph-rest-1.0
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum RecurrencePattern {
    /// Event repeats based on the number of days specified by *interval* between occurrences.
    Daily { 
        /// The number of units between occurrences, where units can be in days, weeks, months, or years, depending on the type. Required.
        interval: i32 
    },

    /// Event repeats on the same day or days of the week, based on the number of weeks between each set of occurrences.
    Weekly { 
        /// The number of units between occurrences, where units can be in days, weeks, months, or years, depending on the type. Required.
        interval: i32, 

        /// A collection of the days of the week on which the event occurs. 
        days_of_week: Vec<DayOfWeek>, 

        /// The first day of the week. Default is `Sunday`.
        first_day_of_week: DayOfWeek 
    },

    /// Event repeats on the specified day of the month (e.g. the 15th), based on the number of months between occurrences.
    AbsoluteMonthly { 
        /// The number of units between occurrences, where units can be in days, weeks, months, or years, depending on the type. Required.
        interval: i32, 

        /// The day of the month on which the event occurs.
        day_of_month: i32 
    },
    
    /// Event repeats on the specified day or days of the week, in the same relative position in the month, based on the number of months between occurrences.
    RelativeMonthly { 
        /// The number of units between occurrences, where units can be in days, weeks, months, or years, depending on the type. Required.
        interval: i32, 

        /// A collection of the days of the week on which the event occurs. 
        /// If `days_of_week` specifies more than one day, the event falls on the first day that satisfies the pattern. 
        days_of_week: Vec<DayOfWeek> 
    },

    /// Event repeats on the specified day and month, based on the number of years between occurrences.
    AbsoluteYearly { 
        /// The number of units between occurrences, where units can be in days, weeks, months, or years, depending on the type. Required.
        interval: i32, 

        /// The day of the month on which the event occurs.
        day_of_month: i32, 

        /// The month in which the event occurs. This is a number from 1 to 12.
        month: i32 
    },

    /// Event repeats on the specified day or days of the week, in the same relative position in a specific month of the year, based on the number of years between occurrences.
    RelativeYearly { 
        /// The number of units between occurrences, where units can be in days, weeks, months, or years, depending on the type. Required.
        interval: i32, 

        /// A collection of the days of the week on which the event occurs. 
        /// If `days_of_week` specifies more than one day, the event falls on the first day that satisfies the pattern. 
        days_of_week: Vec<DayOfWeek>, 

        /// The month in which the event occurs. This is a number from 1 to 12.
        month: i32 
    }
}

#[derive(Deserialize, Debug, Clone)]
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

/// Describes a date range over which a recurring `TodoTask` repeats.
/// 
/// See: https://docs.microsoft.com/en-us/graph/api/resources/recurrencerange?view=graph-rest-1.0
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum RecurrenceRange {
    /// Event repeats on all the days that fit the corresponding recurrence pattern between the `start_date` and `end_date` inclusive.
    EndDate { 
        /// The date to start applying the recurrence pattern. 
        /// The first occurrence of the meeting may be this date or later, 
        /// depending on the recurrence pattern of the event. 
        /// Must be the same value as the start property of the recurring event. Required.
        start_date: Date, 

        /// The date to stop applying the recurrence pattern. 
        /// Depending on the recurrence pattern of the event, the last occurrence of the meeting may not be this date. 
        /// Required if type is endDate.
        end_date: Date, 

        /// Time zone for the startDate and endDate properties. Optional. 
        /// If not specified, the time zone of the event is used.
        recurrence_time_zone: Option<String> 
    },

    /// Event repeats on all the days that fit the corresponding recurrence pattern beginning on the `start_date`.
    NoEnd { 
        /// The date to start applying the recurrence pattern. 
        /// The first occurrence of the meeting may be this date or later, 
        /// depending on the recurrence pattern of the event. 
        /// Must be the same value as the start property of the recurring event. Required.
        start_date: Date, 

        /// Time zone for the startDate and endDate properties. Optional. 
        /// If not specified, the time zone of the event is used.
        recurrence_time_zone: Option<String> 
    },

    /// Event repeats for the `number_of_occurrences` based on the recurrence pattern beginning on the `start_date`.
    Numbered { 
        /// The date to start applying the recurrence pattern. 
        /// The first occurrence of the meeting may be this date or later, 
        /// depending on the recurrence pattern of the event. 
        /// Must be the same value as the start property of the recurring event. Required.
        start_date: Date, 

        /// The number of times to repeat the event. 
        /// Required and must be positive if type is numbered.
        number_of_occurrences: i32, 

        /// Time zone for the startDate and endDate properties. Optional. 
        /// If not specified, the time zone of the event is used.
        recurrence_time_zone: Option<String> 
    },
}