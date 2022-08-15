use std::env;
use xianniu_auto_pause::auto_pause;
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let phone = args.get(1).expect("phone is required");
    let password = args.get(2).expect("password is required");
    auto_pause(phone,password).await
}
