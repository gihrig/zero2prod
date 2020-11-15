use chapter03_0::run;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    run().await
}
