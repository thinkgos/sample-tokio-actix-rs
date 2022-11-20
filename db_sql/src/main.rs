fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = async {
        println!("hello world");

        Ok(())
    };

    return tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime")
        .block_on(body);
}
