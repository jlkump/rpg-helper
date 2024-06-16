fn main() {
    // Load the .env file
    dotenv::dotenv().ok();

    let api_url = std::env::var("API_URL").expect("API_URL is not set in .env");
}