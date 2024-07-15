use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Language{
    pub name: String,
    pub answer: Vec<String>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionList {
    pub question_list: Vec<String>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DB {
    pub language_list: Vec<Language>,
    pub question_list: Vec<String>
}