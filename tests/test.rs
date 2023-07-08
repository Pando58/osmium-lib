#[test]
fn test() {
    let mut app = osmium::init();

    let _ = osmium::query(&mut app).create().graph();

    println!("{:?}", app);
}

#[test]
fn test_query_str() {
    use osmium::query_str;
    use osmium::OperationResponse::*;
    use osmium::QueryParseError::*;

    let app = &mut osmium::init();

    assert_eq!(query_str(app, ""), Err(MissingArgument));
    assert_eq!(query_str(app, "asd"), Err(UnknownOperation));
    assert_eq!(query_str(app, "create"), Err(MissingArgument));
    assert_eq!(query_str(app, "create asd"), Err(UnknownOperation));
    assert_eq!(query_str(app, "create graph"), Ok(Id(0)));
    assert_eq!(query_str(app, "create node"), Err(MissingArgument));
    assert_eq!(query_str(app, "create node asd"), Err(InvalidArgument));
    assert_eq!(query_str(app, "create node example"), Err(MissingArgument));
    assert_eq!(
        query_str(app, "create node example asd"),
        Err(InvalidArgument)
    );
    assert_eq!(
        query_str(app, "create node example 1"),
        Err(OperationError(osmium::OperationError::NonExistentItem))
    );
    assert_eq!(query_str(app, "create node example 0"), Ok(Id(1)));

    println!("{:?}", app);
}
