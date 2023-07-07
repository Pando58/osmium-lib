use osmium::api::QueryParseError;

#[test]
fn test() {
    let mut app = osmium::init();

    let _ = osmium::api::query(&mut app).create().graph();

    println!("{:?}", app);
}

#[test]
fn test_query_str() {
    use osmium::api::query_str;
    use QueryParseError::*;

    let app = &mut osmium::init();

    assert_eq!(query_str(app, ""), Err(MissingArgument));
    assert_eq!(query_str(app, "asd"), Err(UnknownOperation));
    assert_eq!(query_str(app, "create"), Err(MissingArgument));
    assert_eq!(query_str(app, "create asd"), Err(UnknownOperation));
    assert_eq!(query_str(app, "create graph"), Ok(()));
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
    assert_eq!(query_str(app, "create node example 0"), Ok(()));

    println!("{:?}", app);
}
