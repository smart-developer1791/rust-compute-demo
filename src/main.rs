// Import necessary modules from the Axum web framework.
// Axum is used here to define routes, handle HTTP requests, and serve responses.
use axum::{
    extract::Query,    // Used to extract query parameters from HTTP requests.
    response::Html,    // Used to return HTML responses from handler functions.
    routing::get,      // Defines GET routes for the router.
    Router,            // The Router struct is used to create a collection of routes.
};

// Import random number generation utilities from the `rand` crate.
// Used for generating a vector of random numbers in the compute handler.
use rand::Rng;

// Import parallel iteration utilities from the `rayon` crate.
// Rayon allows parallel processing of the vector to improve performance.
use rayon::prelude::*;

// Import standard library modules.
// HashMap: Used to store query parameters.
// SocketAddr: Represents the IP address and port for the server.
// Instant: Used to measure elapsed time for computation.
use std::{collections::HashMap, net::SocketAddr, time::Instant};

// Define the handler function for the root "/" route.
// This function returns an HTML page as the response.
// It is marked `async` because Axum handler functions are asynchronous.
async fn index() -> Html<&'static str> {
    // Return HTML content wrapped in the `Html` type.
    // This tells Axum to send it as an HTML response.
    Html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8" />
<meta name="viewport" content="width=device-width, initial-scale=1.0" /> 
<title>Rust Compute Demo</title>
<script src="https://cdn.tailwindcss.com"></script>
</head>
<body class="bg-gray-100 flex flex-col items-center justify-center min-h-screen p-4">
<h1 class="text-3xl font-bold mb-6 text-center">Rust Compute Demo</h1>

<!-- Three buttons to trigger computation with different sizes -->
<div class="flex gap-4 mb-4">
    <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" onclick="compute(10000000)">Compute 10M</button>
    <button class="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded" onclick="compute(50000000)">Compute 50M</button>
    <button class="bg-purple-500 hover:bg-purple-700 text-white font-bold py-2 px-4 rounded" onclick="compute(100000000)">Compute 100M</button>
</div>

<!-- Progress bar container -->
<div class="w-full max-w-lg">
    <div class="relative w-full h-4 bg-gray-300 rounded mb-2">
        <div id="progress" class="absolute top-0 left-0 h-4 bg-blue-500 rounded w-0"></div>
    </div>
</div>

<!-- Area to display computation results -->
<div id="result" class="text-lg font-mono whitespace-pre-wrap text-center mt-4"></div>

<script>
// JavaScript function that fetches computation results from the server
async function compute(size) {
    // Clear previous result text
    document.getElementById('result').textContent = '';
    
    // Get reference to the progress bar and reset it
    const progressBar = document.getElementById('progress');
    progressBar.style.width = '0%';
    progressBar.classList.remove('bg-green-500');
    progressBar.classList.add('bg-blue-500');

    // Show initial status text
    document.getElementById('result').textContent = 'Computing ' + size.toLocaleString() + ' numbers...';

    // Simulate progress updates while the server is computing
    let width = 0;
    const interval = setInterval(() => {
        // Increase the width randomly up to 90% to simulate progress
        width = Math.min(width + Math.random()*10, 90);
        progressBar.style.width = width + '%';
    }, 50);

    // Send GET request to /compute with the selected size as a query parameter
    const res = await fetch('/compute?size=' + size);
    const text = await res.text(); // Get the result as text

    // Stop progress simulation and finalize the progress bar
    clearInterval(interval);
    progressBar.style.width = '100%';
    progressBar.classList.remove('bg-blue-500');
    progressBar.classList.add('bg-green-500');

    // Display the final computation result in the result div
    document.getElementById('result').textContent = text;
}
</script>
</body>
</html>
"#)
}

// Define the handler function for the "/compute" route.
// It takes query parameters extracted from the request.
async fn compute_handler(params: Query<HashMap<String, String>>) -> String {
    // Extract the "size" query parameter or default to 10,000,000
    let size = params
        .get("size")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(10_000_000);

    // Generate a vector of random numbers in the range 0..10,000
    let data: Vec<u32> = (0..size)
        .map(|_| rand::thread_rng().gen_range(0..10_000))
        .collect();

    // Start timing the computation
    let start = Instant::now();

    // Parallel computation:
    // 1. Filter even numbers
    // 2. Square each even number
    // 3. Sum the squared values
    let sum: u64 = data
        .par_iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x as u64 * x as u64)
        .sum();

    // Measure elapsed time
    let elapsed = start.elapsed();

    // Return a formatted string with the results and elapsed time
    format!("Processed {size} numbers\nResult: {sum}\nTime: {:.2?}", elapsed)
}

// Entry point of the application using Tokio runtime
#[tokio::main]
async fn main() {
    // Create a new Axum router and attach the routes
    let app = Router::new()
        .route("/", get(index))           // Root route serves the HTML page
        .route("/compute", get(compute_handler)); // Compute route handles number computation

    // Determine port from environment variable or default to 8080
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap();

    // Bind the server to all network interfaces on the chosen port
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Server running on http://{addr}");

    // Start the Axum server using Tokio's TCP listener
    // This call will block and handle all incoming HTTP requests
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
