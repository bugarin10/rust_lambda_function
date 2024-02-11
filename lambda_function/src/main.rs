use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

/// This is a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Deserialize)]
struct Request {
    vector: String, // Changed field to accept a String representing a vector
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
struct Response {
    vector_sorted: String, // Changed field name to vector_sorted
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract the vector from the request
    let vector_str = &event.payload.vector;

    // Parse the string representation of the vector into a Vec<i32>
    let vector: Vec<i32> = serde_json::from_str(vector_str)
        .map_err(|e| {
            eprintln!("Error parsing vector: {}", e);
            Error::from("Error parsing vector")
        })?;

    // Sort the vector using insertion sort
    let mut sorted_vector = vector.clone();
    let length = sorted_vector.len();

    for j in 1..length {
        let key = sorted_vector[j];
        let mut i = j as i32 - 1;
        while i >= 0 && sorted_vector[i as usize] > key {
            sorted_vector[(i + 1) as usize] = sorted_vector[i as usize];
            i = i - 1;
        }
        sorted_vector[(i + 1) as usize] = key;
    }

    // Serialize the sorted vector back into a string
    let sorted_vector_str = serde_json::to_string(&sorted_vector)
        .map_err(|e| {
            eprintln!("Error serializing sorted vector: {}", e);
            Error::from("Error serializing sorted vector")
        })?;

    // Prepare the response
    // let resp = Response {
    //     vector_sorted: sorted_vector_str,
    // };

    let message = format!("sorted_vector:{sorted_vector_str}");

    let resp = Response::builder()
    .status(200)
    .header("content-type", "text/html")
    .body(message.into())
    .map_err(Box::new)?;
    Ok(resp);

    // Return the response
    // Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
