use cynic;
pub mod get_tree;

pub mod github_dsl {
    cynic::query_dsl!("src/gql/github.schema.graphql");
}
