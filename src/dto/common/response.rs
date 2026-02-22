use serde::Serialize;

use super::ResponseInfo;

#[derive(Serialize, Debug)]
pub struct Response<T> {
    #[serde(rename = "responseInfo")]
    pub responseinfo: ResponseInfo,
    pub data: T,
}
