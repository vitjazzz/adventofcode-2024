mod day1;

#[tokio::main]
async fn main() {
    day1::execute().await.unwrap();
}
