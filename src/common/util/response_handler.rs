use crate::model::response::api_response::ApiResponse;

pub fn success<T>(resp: ApiResponse<T>) -> bool {
    if resp.resultCode == "200" && resp.statusCode == "200" {
        return true;
    }
    else{
        return false;
    }
}


