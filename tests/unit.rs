use crate::graphql::schema::Schema;
use async_graphql::Request;

#[tokio::test]
async fn test_graphql_query() {
    let schema = Schema::new();
    let request = Request::new("{ __typename }".to_string());
    let response = schema.execute(request).await;
    assert!(response.is_ok());
}
