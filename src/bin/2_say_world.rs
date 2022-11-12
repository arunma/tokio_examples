use std::error::Error;
use std::future::Future;

async fn foo() -> usize {
    println!("World !");
    return 0;
}


fn foo2() -> impl Future<Output=()> {
    async {
        println!("hello1");
        let f = foo().await;
        println!("hello2")
    }
}

#[tokio::main]
async fn main() {
    foo2().await;
}
