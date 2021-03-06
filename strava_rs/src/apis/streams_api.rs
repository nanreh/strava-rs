/*
 * Strava API v3
 *
 * The [Swagger Playground](https://developers.strava.com/playground) is the easiest way to familiarize yourself with the Strava API by submitting HTTP requests and observing the responses before you write any client code. It will show what a response will look like with different endpoints depending on the authorization scope you receive from your athletes. To use the Playground, go to https://www.strava.com/settings/api and change your “Authorization Callback Domain” to developers.strava.com. Please note, we only support Swagger 2.0. There is a known issue where you can only select one scope at a time. For more information, please check the section “client code” at https://developers.strava.com/docs.
 *
 * The version of the OpenAPI document: 3.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused_imports)]
use std::rc::Rc;

use std::option::Option;

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `get_activity_streams`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetActivityStreamsError {
    DefaultResponse(crate::models::Fault),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_route_streams`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRouteStreamsError {
    DefaultResponse(crate::models::Fault),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_segment_effort_streams`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSegmentEffortStreamsError {
    DefaultResponse(crate::models::Fault),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_segment_streams`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSegmentStreamsError {
    DefaultResponse(crate::models::Fault),
    UnknownValue(serde_json::Value),
}


    pub async fn get_activity_streams(configuration: &configuration::Configuration, id: i64, keys: Vec<String>, key_by_type: bool) -> Result<crate::models::StreamSet, Error<GetActivityStreamsError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/activities/{id}/streams", configuration.base_path, id=id);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("keys", &keys.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
        req_builder = req_builder.query(&[("key_by_type", &key_by_type.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<GetActivityStreamsError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn get_route_streams(configuration: &configuration::Configuration, id: i64) -> Result<crate::models::StreamSet, Error<GetRouteStreamsError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/routes/{id}/streams", configuration.base_path, id=id);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<GetRouteStreamsError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn get_segment_effort_streams(configuration: &configuration::Configuration, id: i64, keys: Vec<String>, key_by_type: bool) -> Result<crate::models::StreamSet, Error<GetSegmentEffortStreamsError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/segment_efforts/{id}/streams", configuration.base_path, id=id);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("keys", &keys.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
        req_builder = req_builder.query(&[("key_by_type", &key_by_type.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<GetSegmentEffortStreamsError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn get_segment_streams(configuration: &configuration::Configuration, id: i64, keys: Vec<String>, key_by_type: bool) -> Result<crate::models::StreamSet, Error<GetSegmentStreamsError>> {
        let client = &configuration.client;

        let uri_str = format!("{}/segments/{id}/streams", configuration.base_path, id=id);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("keys", &keys.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
        req_builder = req_builder.query(&[("key_by_type", &key_by_type.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<GetSegmentStreamsError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

