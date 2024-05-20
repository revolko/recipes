use dotenvy::dotenv;

fn main() {
    dotenv().expect(".env file not found");
    let test = dotenvy::var("TEST").expect("TEST is not set!");
    println!("{}", test);
}
