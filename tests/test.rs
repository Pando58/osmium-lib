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

#[tokio::test]
async fn test_events() {
    use osmium::core::nodes::Nodes;
    use osmium::Event;

    let mut app = osmium::init();

    let mut rx = app.create_event_receiver();

    let t = tokio::spawn(async move {
        assert_eq!(
            rx.recv().await.unwrap(),
            Event::InputsUpdated { node_id: 0 }
        );
        assert_eq!(
            rx.recv().await.unwrap(),
            Event::NodesUpdated { graph_id: 0 }
        );
        assert_eq!(rx.recv().await.unwrap(), Event::GraphsUpdated);
        assert_eq!(
            rx.recv().await.unwrap(),
            Event::InputsUpdated { node_id: 1 }
        );
        assert_eq!(
            rx.recv().await.unwrap(),
            Event::NodesUpdated { graph_id: 0 }
        );
    });

    let _ = osmium::query(&mut app).create().graph();
    let _ = osmium::query(&mut app).create().node(Nodes::Example, 0);

    let _ = tokio::join!(t);
}
