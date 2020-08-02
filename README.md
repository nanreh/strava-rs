
# Rust client for the Strava API

This repo contains:
* A working Rust client for the Strava API [strava_rs](./strava_rs).
* An [example project](./example_project) that uses the Rust client to make a simple Strava API call.
* [A patch file](./strava-api-spec.patch) to fix a few issues with Strava's Swagger JSON spec.
* A few support files.
* Some salty commentary.

## Salty Commentary
Consuming the Strava API sucks. Strava provides a JSON Swagger description of their API and a few basic instructions. There are issues with the spec they provide, however, and the code generation tools for Swagger are a minefield.

I've found the entire process to be pretty busted and just plain unfortunate because I love Strava. I wish I could say the same about my experience trying to use the Strava API in 2020.

Below are the steps I went through to get a working Strava client in Rust. If you're trying to generate a Strava client in Rust (or any other language) you may find the information useful.

Here we go.

## Clone this repo
```
git clone git@github.com:nanreh/strava-rs.git
```

## Download Strava's (broken) Swagger spec
```
cd strava-rs
curl https://developers.strava.com/swagger/swagger.json -o strava-api-spec.json
```

Unfortunately, this API spec has several problems which are consequential for generating working Rust code (and code in other languages I'm sure). The next step applies a patch file to address these issues.

## Patch it
```
patch < strava-api-spec.patch
```

## Download openapi-generator-cli-5.0.0-beta.jar

The Strava developers page sends developers to Swagger Codegen but its Rust support is antiquated and the code it generates is awful. That's not just my opinion: the Rust compiler hates it too.

OpenAPI Generator is a fork of Swagger Codegen. It is still a terrible way to consume an HTTP API, but it's far ahead of Swagger Codegen at the moment.

**It's important to use version 5.0.0-beta (or newer) of OpenAPI Generator** since it includes many recent updates to the Rust codegen, including the option to generate code that relies on the reqwest crate (much simpler) plus updated dependencies.

```
wget https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/5.0.0-beta/openapi-generator-cli-5.0.0-beta.jar -O openapi-generator-cli.jar
```

Joy: it's a Java program. So you need to make sure you have java installed. Because it's 2020 and everybody loves Java about as much as Covid-19.

If you want to know why OpenAPI Generator forked from Swagger Codegen, there's a write up available [here](https://github.com/OpenAPITools/openapi-generator/blob/master/docs/qna.md). Maybe you're like me and one afternoon you thought "I'm going to build something cool with the Strava API" and now you're reading about open source drama on a project you never knew existed and never cared about. You're welcome.

## Generate the Rust client code.

We use a config file with values for the Rust code generator to specifically ask for a client that uses reqwest and that supports async operation.
```
java -jar ../openapi-generator-cli.jar generate -i ../swagger.json -g rust --config=rust-gen-config.json --generate-alias-as-model -o strava_rs
```

## Build
```
cd strava_rs
cargo build
```

This compiles pretty cleanly as of this writing (2020/07/01) with just a single warning for an unused variable:
```
warning: unused variable: `file`
  --> src/apis/uploads_api.rs:39:78
   |
39 |     pub async fn create_upload(configuration: &configuration::Configuration, file: Option<std::path::PathBuf>, name: Option<&str>, description: Option<&str>, trainer: Option<&str>, commute: Option<&str>, data_type: Option<&str>, external_id: Option<&str>) -> Result<crate::models::Upload, Error<CreateUploadError>> {
   |                                                                              ^^^^ help: if this is intentional, prefix it with an underscore: `_file`
   |
   = note: `#[warn(unused_variables)]` on by default
```

## Example

So how do you use this monstrosity to make an HTTP call to Strava's API? Great question. Check [the example project](./example_project) in this repo.