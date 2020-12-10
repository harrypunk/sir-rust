/*use cynic::QueryFragment;

#[derive(cynic::QueryFragment)]
#[cynic(
    schema_path = "src/gql/github.schema.graphql",
    query_module = "github_dsl",
    graphql_type = "Film"
)]
struct Tree {
    commitUrl: Option<String>,
}

#[derive(cynic::QueryFragment)]
#[cynic(
    schema_path = "src/gql/gettree.graphql",
    query_module = "github_dsl",
    graphql_type = "Tree"
)]
struct EntriesQuery {
    owner: String,
    repo: String,
    folderpath: String,
}

pub fn req_entries() {
    let operation = cynic::Operation::query(EntriesQuery::fragment(&()));
}*/
