# \RunningRacesApi

All URIs are relative to *https://www.strava.com/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_running_race_by_id**](RunningRacesApi.md#get_running_race_by_id) | **get** /running_races/{id} | Get Running Race
[**get_running_races**](RunningRacesApi.md#get_running_races) | **get** /running_races | List Running Races



## get_running_race_by_id

> crate::models::RunningRace get_running_race_by_id(id)
Get Running Race

Returns a running race for a given identifier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The identifier of the running race. | [required] |

### Return type

[**crate::models::RunningRace**](RunningRace.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_running_races

> Vec<crate::models::RunningRace> get_running_races(year)
List Running Races

Returns a list running races based on a set of search criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | Option<**i32**> | Filters the list by a given year. |  |

### Return type

[**Vec<crate::models::RunningRace>**](RunningRace.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

