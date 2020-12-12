pub struct Gettree;
pub mod query {
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
    type URI = String;
    #[derive(Debug, Deserialize)]
    #[doc = "Represents a Git tree entry.\n"]
    pub struct GettreeRepositoryObjectOnTreeEntries {
        #[doc = "Entry file name.\n"]
        pub name: String,
    }
    #[derive(Debug, Deserialize)]
    #[doc = "Represents a Git tree.\n"]
    pub struct GettreeRepositoryObjectOnTree {
        #[doc = "The HTTP URL for this Git object\n"]
        #[serde(rename = "commitUrl")]
        pub commit_url: URI,
        #[doc = "A list of tree entries.\n"]
        pub entries: Option<Vec<GettreeRepositoryObjectOnTreeEntries>>,
    }
    #[derive(Debug, Deserialize)]
    #[serde(tag = "__typename")]
    pub enum GettreeRepositoryObjectOn {
        Tree(GettreeRepositoryObjectOnTree),
        Tag,
        Blob,
        Commit,
    }
    #[derive(Debug, Deserialize)]
    pub struct GettreeRepositoryObject {
        #[serde(flatten)]
        pub on: GettreeRepositoryObjectOn,
    }
    #[derive(Debug, Deserialize)]
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
    #[derive(Debug, Deserialize)]
    pub struct ResponseData {
        #[doc = "Lookup a given repository by the owner and repository name.\n"]
        pub repository: Option<GettreeRepository>,
    }
}
impl graphql_client::GraphQLQuery for Gettree {
    type Variables = query::Variables;
    type ResponseData = query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: query::QUERY,
            operation_name: query::OPERATION_NAME,
        }
    }
}

pub mod request {
    use super::query::{GettreeRepositoryObjectOn, ResponseData, Variables};
    use graphql_client::{GraphQLQuery, Response};
    use reqwest::blocking::Client;
    pub fn req_entries(url: String, token: String, repo_info: Variables) -> Vec<String> {
        let query_body = super::Gettree::build_query(repo_info);
        let client = Client::builder()
            .user_agent("sir_rust/0.1.0")
            .build()
            .expect("client build failed");
        let req_builder = client.post(&url).bearer_auth(token).json(&query_body);
        let res = req_builder.send();
        let response = res.expect("Resposne failed");
        let res_body: Response<ResponseData> = response.json().expect("text failed");
        let res_data: ResponseData = res_body.data.expect("data from body failed");
        let obj = res_data
            .repository
            .as_ref()
            .map(|repo| repo.object.as_ref())
            .flatten()
            .expect("repo object failed");
        match &obj.on {
            GettreeRepositoryObjectOn::Tree(x) => x
                .entries
                .as_ref()
                .iter()
                .flat_map(|xs| xs.iter())
                .map(|o| (&o.name).clone())
                .fold(vec![], |mut acc, n| {
                    acc.push(n.to_string());
                    acc
                }),
            _ => vec![],
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::gql::gettree::query::Variables;
        use regex::Regex;
        use std::env;
        #[test]
        fn filenames() {
            let url = "https://api.github.com/graphql";
            let repo = "COVID-19".to_string();
            let owner = "CSSEGISandData".to_string();
            let folderpath = "master:csse_covid_19_data/csse_covid_19_daily_reports_us".to_string();
            let args: env::Vars = env::vars();
            let token = args
                .filter(|(k, _)| k == "GHTOKEN1")
                .map(|(_, v)| v)
                .next()
                .expect("no token env");
            let filenames: Vec<String> = super::req_entries(
                url.to_string(),
                token,
                Variables {
                    owner,
                    repo,
                    folderpath,
                },
            );
            let re = Regex::new(r"^\d{2}-\d{2}-\d{4}.csv$").expect("regex failure");
            filenames
                .iter()
                .skip(20)
                .take(10)
                .for_each(|x| assert!(re.is_match(x)))
        }
    }
}
