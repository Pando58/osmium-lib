#[test]
fn test() {
    let mut app = osmium::init();

    let _ = osmium::api::query(&mut app).create().graph();

    println!("{:?}", app);
}
