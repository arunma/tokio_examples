#[tokio::main]
async fn main() {
    let vec = vec![1,2,3,4,5];
    // Throws compilation error if the vector isn't moved using `async move`
    // All tokio tasks have 'static lifetime. Therefore, spawned task cannot borrow from references outside the task
    tokio::spawn(async {
        println!("{:?}",vec);
    });
}