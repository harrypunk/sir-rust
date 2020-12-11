use graphql_client::{GraphQLQuery, Response};
use reqwest::blocking::Client;
use sir_rust::gql::gettree::gettree::{
    GettreeRepositoryObjectOn, GettreeRepositoryObjectOnTreeEntries, ResponseData, Variables,
};
use sir_rust::gql::gettree::Gettree;
use std::env;

pub fn main() {
    let args: env::Vars = env::vars();
    let token = args
        .filter(|(k, _)| k == "GHTOKEN1")
        .map(|(_, v)| v)
        .next()
        .expect("no token env");
    let url = "https://api.github.com/graphql";
    let repo = "COVID-19";
    let owner = "CSSEGISandData";
    let folderpath = "master:csse_covid_19_data/csse_covid_19_daily_reports_us";

    let query_body = Gettree::build_query(Variables {
        owner: owner.to_string(),
        repo: repo.to_string(),
        folderpath: folderpath.to_string(),
    });
    let client = Client::builder()
        .user_agent("sir_rust/0.1.0")
        .build()
        .expect("client build failed");
    let req_builder = client.post(url).bearer_auth(token).json(&query_body);
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
        GettreeRepositoryObjectOn::Tree(x) => {
            let list: &Vec<GettreeRepositoryObjectOnTreeEntries> =
                &x.entries.as_ref().expect("entries failed");
            for i in list {
                println!("{}", i.name);
            }
        }
        _ => println!("infos"),
    }
}
