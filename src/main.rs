use std::fmt::Display;
use errors::{ClientError, ServerError};
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use async_graphql::{http::GraphiQLSource, Request as GraphQlRequest, ServerError as GraphQlError, Response as GraphQlResponse};
use http::{Method, StatusCode};
use schemas::APP_SCHEMA;

mod errors;
mod schemas;

async fn handler(event: Request) -> Result<Response<Body>, Error> {
    if event.method() == Method::GET {
        return Response::builder()
            .status(200)
            .header("Content-Type", "text/html")
            .body(Body::Text(GraphiQLSource::build().endpoint("/lambda-url/rust-lambda/").finish().to_string()))
            .map_err(ServerError::from)
            .map_err(Error::from)
    }

    let query = match event.method() {
        &Method::POST => graphql_handle_post(event),
        _ => Err(ClientError::MethodNotAllowed),
    };

    let query = match query {
        Err(e) => {
            return generate_error_response(StatusCode::BAD_REQUEST, handle_graphql_error(e));
        }
        Ok(query) => query,
    };

    let response_body =
        serde_json::to_string(&APP_SCHEMA.execute(query).await).map_err(ServerError::from)?;
        
    Response::builder()
        .status(200)
        .body(Body::Text(response_body))
        .map_err(ServerError::from)
        .map_err(Error::from)
}

fn handle_graphql_error(message: impl Display) -> String {
    let message = format!("{}", message);
    let response = GraphQlResponse::from_errors(vec![GraphQlError::new(message, None)]);
    
    serde_json::to_string(&response).unwrap()
}

fn generate_error_response(status: StatusCode, body: String) -> Result<Response<Body>, Error> {
    Ok(Response::builder().status(status).body(Body::Text(body))?)
}

fn graphql_handle_post(request: Request) -> Result<GraphQlRequest, ClientError> {
    match request.into_body() {
        Body::Empty => Err(ClientError::EmptyBody),
        Body::Text(text) => {
            serde_json::from_str::<GraphQlRequest>(&text).map_err(ClientError::from)
        }
        Body::Binary(binary) => {
            serde_json::from_slice::<GraphQlRequest>(&binary).map_err(ClientError::from)
        }
    }
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(handler)).await?;

    Ok(())
}
