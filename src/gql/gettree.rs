pub struct Gettree;
pub mod gettree {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &'static str = "Gettree";
    pub const QUERY : & 'static str = "query Gettree($owner: String!, $repo: String!, $folderpath: String!){\n  repository(name:$repo,owner:$owner){\n    object(expression:$folderpath){\n      __typename\n      ... on Tree {\n        commitUrl\n        entries{\n          name\n        }\n      }\n    }\n  }\n}\n" ;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[doc = "An RFC 3986, RFC 3987, and RFC 6570 (level 4) compliant URI string.\n"]
    type URI = super::URI;
    #[derive(Deserialize)]
    #[doc = "Represents a Git tree entry.\n"]
    pub struct GettreeRepositoryObjectOnTreeEntries {
        #[doc = "Entry file name.\n"]
        pub name: String,
    }
    #[derive(Deserialize)]
    #[doc = "Represents a Git tree.\n"]
    pub struct GettreeRepositoryObjectOnTree {
        #[doc = "The HTTP URL for this Git object\n"]
        #[serde(rename = "commitUrl")]
        pub commit_url: URI,
        #[doc = "A list of tree entries.\n"]
        pub entries: Option<Vec<GettreeRepositoryObjectOnTreeEntries>>,
    }
    #[derive(Deserialize)]
    #[serde(tag = "__typename")]
    pub enum GettreeRepositoryObjectOn {
        Tree(GettreeRepositoryObjectOnTree),
        Blob,
        Commit,
        Tag,
    }
    #[derive(Deserialize)]
    pub struct GettreeRepositoryObject {
        #[serde(flatten)]
        pub on: GettreeRepositoryObjectOn,
    }
    #[derive(Deserialize)]
    #[doc = "A repository contains the content for a project.\n"]
    pub struct GettreeRepository {
        #[doc = "A Git object in the repository\n"]
        pub object: Option<GettreeRepositoryObject>,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub owner: String,
        pub repo: String,
        pub folderpath: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[doc = "Lookup a given repository by the owner and repository name.\n"]
        pub repository: Option<GettreeRepository>,
    }
}
impl graphql_client::GraphQLQuery for Gettree {
    type Variables = gettree::Variables;
    type ResponseData = gettree::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: gettree::QUERY,
            operation_name: gettree::OPERATION_NAME,
        }
    }
}
