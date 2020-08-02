use strava_rs::apis::configuration::Configuration;

/// This is a minimal program to make an API call to Strava. While the process
/// to create the client code was excruciatingly bad, the resulting code works
/// and using it isn't terrible given the recent updates in the OpenAPI 
/// Generator's Rust support.
#[tokio::main]
async fn main() {
    let cfg = Configuration {
        oauth_access_token: Some("<replace with your access token>".to_owned()),
        ..strava_rs::apis::configuration::Configuration::new()
    };
    let athlete_details = strava_rs::apis::athletes_api::get_logged_in_athlete(&cfg).await;
    println!("Athlete: {:?}", athlete_details);
}
