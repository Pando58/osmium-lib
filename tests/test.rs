#[test]
fn test() {
    let app = osmium::init();

    let _ = osmium::api::query(&app).create().graph();

    println!("{:?}", app);
}
