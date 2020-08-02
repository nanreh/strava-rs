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
pub struct DetailedSegmentEffortAllOf {
    /// The name of the segment on which this effort was performed
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "activity", skip_serializing_if = "Option::is_none")]
    pub activity: Option<crate::models::MetaActivity>,
    #[serde(rename = "athlete", skip_serializing_if = "Option::is_none")]
    pub athlete: Option<crate::models::MetaAthlete>,
    /// The effort's moving time
    #[serde(rename = "moving_time", skip_serializing_if = "Option::is_none")]
    pub moving_time: Option<i32>,
    /// The start index of this effort in its activity's stream
    #[serde(rename = "start_index", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
    /// The end index of this effort in its activity's stream
    #[serde(rename = "end_index", skip_serializing_if = "Option::is_none")]
    pub end_index: Option<i32>,
    /// The effort's average cadence
    #[serde(rename = "average_cadence", skip_serializing_if = "Option::is_none")]
    pub average_cadence: Option<f32>,
    /// The average wattage of this effort
    #[serde(rename = "average_watts", skip_serializing_if = "Option::is_none")]
    pub average_watts: Option<f32>,
    /// For riding efforts, whether the wattage was reported by a dedicated recording device
    #[serde(rename = "device_watts", skip_serializing_if = "Option::is_none")]
    pub device_watts: Option<bool>,
    /// The heart heart rate of the athlete during this effort
    #[serde(rename = "average_heartrate", skip_serializing_if = "Option::is_none")]
    pub average_heartrate: Option<f32>,
    /// The maximum heart rate of the athlete during this effort
    #[serde(rename = "max_heartrate", skip_serializing_if = "Option::is_none")]
    pub max_heartrate: Option<f32>,
    #[serde(rename = "segment", skip_serializing_if = "Option::is_none")]
    pub segment: Option<crate::models::SummarySegment>,
    /// The rank of the effort on the global leaderboard if it belongs in the top 10 at the time of upload
    #[serde(rename = "kom_rank", skip_serializing_if = "Option::is_none")]
    pub kom_rank: Option<i32>,
    /// The rank of the effort on the athlete's leaderboard if it belongs in the top 3 at the time of upload
    #[serde(rename = "pr_rank", skip_serializing_if = "Option::is_none")]
    pub pr_rank: Option<i32>,
    /// Whether this effort should be hidden when viewed within an activity
    #[serde(rename = "hidden", skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
}

impl DetailedSegmentEffortAllOf {
    pub fn new() -> DetailedSegmentEffortAllOf {
        DetailedSegmentEffortAllOf {
            name: None,
            activity: None,
            athlete: None,
            moving_time: None,
            start_index: None,
            end_index: None,
            average_cadence: None,
            average_watts: None,
            device_watts: None,
            average_heartrate: None,
            max_heartrate: None,
            segment: None,
            kom_rank: None,
            pr_rank: None,
            hidden: None,
        }
    }
}


