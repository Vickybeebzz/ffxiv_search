use std::env;
use serde_json::{Value};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "Pagination")]
    pub pagination: Pagination,
    #[serde(rename = "Results")]
    pub results: Vec<Result>,
    #[serde(rename = "SpeedMs")]
    pub speed_ms: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    #[serde(rename = "Page")]
    pub page: i64,
    #[serde(rename = "PageNext")]
    pub page_next: ::serde_json::Value,
    #[serde(rename = "PagePrev")]
    pub page_prev: ::serde_json::Value,
    #[serde(rename = "PageTotal")]
    pub page_total: i64,
    #[serde(rename = "Results")]
    pub results: i64,
    #[serde(rename = "ResultsPerPage")]
    pub results_per_page: i64,
    #[serde(rename = "ResultsTotal")]
    pub results_total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    #[serde(rename = "ID")]
    pub id: i64,
    #[serde(rename = "Icon")]
    pub icon: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Url")]
    pub url: String,
    #[serde(rename = "UrlType")]
    pub url_type: String,
    #[serde(rename = "_")]
    pub field: String,
    #[serde(rename = "_Score")]
    pub score: i64,
}



fn main(){

    //Gets arguments from command line.
    let args: Vec<String> = env::args().collect();
    let itemname = &args[1];

    //Gets the request url.
    let request_url = format!("https://xivapi.com/search?string={}",itemname);

    //Does the request to the API
    let  body = reqwest::blocking::get(&request_url).unwrap()
    .text().unwrap();

    //Parses the request into a Struct
    let list: Root = serde_json::from_str(&body).unwrap();
    let resultlist: Vec<Result> = list.results;

    //Gets the item url from the best match (first result)
    let request_url = format!("https://xivapi.com{}",resultlist[0].url);

    //Does second API request
    let body2 = reqwest::blocking::get(&request_url).unwrap()
    .text().unwrap();

    //Parses request into a Value
    let v: Value = serde_json::from_str(&body2).unwrap();

    //Printing results
    println!("Name: {}", v["Name"]);
    println!("Description {}", v["Description"]);
    println!("Item ID {}", v["ID"]);

    //Just cleans up and gives a usable icon url.
    let image = format!("https://xivapi.com{}",v["Icon"]);
    let image= image.replace("\"", "");

    //Printing urls
    println!("Image: {} URL: {}", &image, &request_url);

    
   


    ()
}
