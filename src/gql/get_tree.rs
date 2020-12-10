use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/github.schema.graphql",
    query_path = "src/gql/gettree.graphql",
)]
pub struct TreeQuery;
