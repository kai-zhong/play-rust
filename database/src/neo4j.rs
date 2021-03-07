#[cfg(test)]
mod testing {
    use neo4rs::{Graph, Node};


    async fn setup() {
        // clear database.
        let result = get_graph().await;
        result.unwrap().run(neo4rs::query("MATCH (n) DETACH DELETE n")).await;
    }

    async fn get_graph() -> neo4rs::Result<Graph> {
        let uri = "127.0.0.1:7687";
        let user = "neo4j";
        let pass = "123456";

        Graph::new(&uri, user, pass).await
    }

    #[tokio::test]
    async fn connect() {

        let result = get_graph().await;

        assert!(result.is_ok());
    }


    #[tokio::test]
    async fn create_node() {
        setup().await;

        let graph = get_graph().await.unwrap();
        let result = graph.execute(
            neo4rs::query("CREATE (n:Example {text: $text}) RETURN n")
                .param("text", "hello")
        ).await;

        assert!(result.is_ok());

        let mut result = result.unwrap();
        while let Ok(Some(row)) = result.next().await {
            let node: Node = row.get("n").unwrap();
            let labels = node.labels();
            let text: Option<String> = node.get("text");

            assert_eq!(Some(String::from("hello")), text);
            assert_eq!(vec!["Example"], labels);
        };
    }
}