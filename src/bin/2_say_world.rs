async fn say_world(){
    print!("World !")
}
#[tokio::main]
async fn main() {
    let op = say_world();

    print!("hello ");

    op.await

}