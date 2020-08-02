/*
 * Strava API v3
 *
 * The [Swagger Playground](https://developers.strava.com/playground) is the easiest way to familiarize yourself with the Strava API by submitting HTTP requests and observing the responses before you write any client code. It will show what a response will look like with different endpoints depending on the authorization scope you receive from your athletes. To use the Playground, go to https://www.strava.com/settings/api and change your “Authorization Callback Domain” to developers.strava.com. Please note, we only support Swagger 2.0. There is a known issue where you can only select one scope at a time. For more information, please check the section “client code” at https://developers.strava.com/docs.
 *
 * The version of the OpenAPI document: 3.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdatableActivity {
    /// Whether this activity is a commute
    #[serde(rename = "commute", skip_serializing_if = "Option::is_none")]
    pub commute: Option<bool>,
    /// Whether this activity was recorded on a training machine
    #[serde(rename = "trainer", skip_serializing_if = "Option::is_none")]
    pub trainer: Option<bool>,
    /// The description of the activity
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the activity
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::ActivityType>,
    /// Identifier for the gear associated with the activity. ‘none’ clears gear from activity
    #[serde(rename = "gear_id", skip_serializing_if = "Option::is_none")]
    pub gear_id: Option<String>,
}

impl UpdatableActivity {
    pub fn new() -> UpdatableActivity {
        UpdatableActivity {
            commute: None,
            trainer: None,
            description: None,
            name: None,
            _type: None,
            gear_id: None,
        }
    }
}

