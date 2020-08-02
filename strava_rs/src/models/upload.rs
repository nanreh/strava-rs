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
pub struct Upload {
    /// The unique identifier of the upload
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The unique identifier of the upload in string format
    #[serde(rename = "id_str", skip_serializing_if = "Option::is_none")]
    pub id_str: Option<String>,
    /// The external identifier of the upload
    #[serde(rename = "external_id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// The error associated with this upload
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// The status of this upload
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The identifier of the activity this upload resulted into
    #[serde(rename = "activity_id", skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<i64>,
}

impl Upload {
    pub fn new() -> Upload {
        Upload {
            id: None,
            id_str: None,
            external_id: None,
            error: None,
            status: None,
            activity_id: None,
        }
    }
}


