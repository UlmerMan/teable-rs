use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FieldOptions {
    Expression(ExpressionOptions),
    Link(LinkOptions),
    Formatting(FormattingOptions),
    Checkbox(CheckboxOptions),
    Text(TextOptions),
    Rating(RatingOptions),
    User(UserOptions),
    Select(SelectOptions),
    Number(NumberOptions),
    Button(ButtonOptions),
    Empty(EmptyOptions),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExpressionOptions {
    pub expression: String,
    #[serde(rename = "timeZone")]
    pub time_zone: Option<String>,
    pub formatting: Option<Formatting>,
    #[serde(rename = "showAs")]
    pub show_as: Option<ShowAs>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LinkOptions {
    #[serde(rename = "foreignTableId")]
    pub foreign_table_id: String,
    pub relationship: Relationship,
    #[serde(rename = "baseId")]
    pub base_id: Option<String>,
    #[serde(rename = "lookupFieldId")]
    pub lookup_field_id: Option<String>,
    #[serde(rename = "isOneWay")]
    pub is_one_way: Option<bool>,
    #[serde(rename = "fkHostTableName")]
    pub fk_host_table_name: Option<String>,
    #[serde(rename = "selfKeyName")]
    pub self_key_name: Option<String>,
    #[serde(rename = "foreignKeyName")]
    pub foreign_key_name: Option<String>,
    #[serde(rename = "symmetricFieldId")]
    pub symmetric_field_id: Option<String>,
    #[serde(rename = "filterByViewId")]
    pub filter_by_view_id: Option<String>,
    #[serde(rename = "visibleFieldIds")]
    pub visible_field_ids: Option<Vec<String>>,
    pub filter: Option<Filter>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormattingOptions {
    pub formatting: Formatting,
    #[serde(rename = "defaultValue")]
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CheckboxOptions {
    #[serde(rename = "defaultValue")]
    pub default_value: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TextOptions {
    #[serde(rename = "showAs", skip_serializing_if = "Option::is_none")]
    pub show_as: Option<ShowAs>,
    #[serde(rename = "defaultValue", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RatingOptions {
    pub icon: RatingIcon,
    pub color: RatingColor,
    pub max: u8,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserOptions {
    #[serde(rename = "isMultiple")]
    pub is_multiple: Option<bool>,
    #[serde(rename = "shouldNotify")]
    pub should_notify: Option<bool>,
    #[serde(rename = "defaultValue")]
    pub default_value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SelectOptions {
    pub choices: Vec<Choice>,
    #[serde(rename = "defaultValue")]
    pub default_value: Option<serde_json::Value>,
    #[serde(rename = "preventAutoNewOptions")]
    pub prevent_auto_new_options: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NumberOptions {
    pub formatting: NumberFormatting,
    #[serde(rename = "showAs")]
    pub show_as: Option<ShowAs>,
    #[serde(rename = "defaultValue")]
    pub default_value: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ButtonOptions {
    pub label: String,
    pub color: ButtonColor,
    #[serde(rename = "maxCount")]
    pub max_count: Option<u32>,
    #[serde(rename = "resetCount")]
    pub reset_count: Option<bool>,
    pub workflow: Option<Workflow>,
    pub confirm: Option<Confirmation>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct EmptyOptions {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Choice {
    pub id: Option<String>,
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Workflow {
    pub id: String,
    pub name: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Confirmation {
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "confirmText")]
    pub confirm_text: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Relationship {
    OneOne,
    ManyMany,
    OneMany,
    ManyOne,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ShowAs {
    Url,
    Email,
    Phone,
    Bar,
    Ring,
    Line,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Formatting {
    pub date: DateFormat,
    pub time: TimeFormat,
    #[serde(rename = "timeZone")]
    pub time_zone: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NumberFormatting(pub serde_json::Value);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Filter(pub serde_json::Value);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DateFormat {
    #[serde(rename = "M/D/YYYY")]
    MonthDayYear,
    #[serde(rename = "D/M/YYYY")]
    DayMonthYear,
    #[serde(rename = "YYYY/MM/DD")]
    YearMonthDaySlash,
    #[serde(rename = "YYYY-MM-DD")]
    YearMonthDay,
    #[serde(rename = "YYYY-MM")]
    YearMonth,
    #[serde(rename = "MM-DD")]
    MonthDay,
    #[serde(rename = "YYYY")]
    Year,
    #[serde(rename = "MM")]
    Month,
    #[serde(rename = "DD")]
    Day,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TimeFormat {
    #[serde(rename = "HH:mm")]
    HoursMinutes,
    #[serde(rename = "hh:mm A")]
    HoursMinutesAmPm,
    #[serde(rename = "None")]
    None,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RatingIcon {
    Star,
    Moon,
    Sun,
    Zap,
    Flame,
    Heart,
    Apple,
    ThumbUp,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RatingColor {
    YellowBright,
    RedBright,
    TealBright,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ButtonColor(pub String);
