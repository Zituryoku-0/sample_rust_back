use serde::Serialize;

use super::ResponseInfo;

#[derive(Serialize, Debug)]
pub struct Response<T> {
    pub responseinfo: ResponseInfo,
    pub data: T,
}
