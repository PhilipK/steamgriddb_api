use serde::{Deserialize, Serialize};
pub struct SteamGridDbError {
    pub status: Option<u32>,
    pub errors: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    pub success: bool,
    pub data: Option<Vec<T>>,
    pub status: Option<u32>,
    pub errors: Option<Vec<String>>,
}

pub type SteamGridDbResult<T> = std::result::Result<T, SteamGridDbError>;

pub fn response_to_result<T>(inner: Response<T>) -> SteamGridDbResult<Vec<T>> {
    if !inner.success {
        std::result::Result::Err(SteamGridDbError {
            errors: inner.errors,
            status: inner.status,
        })
    } else {
        match inner.data {
            Some(data) => std::result::Result::Ok(data),
            None => std::result::Result::Err(SteamGridDbError {
                errors: Some(vec!["Succes reported but no grids found".to_string()]),
                status: None,
            }),
        }
    }
}

pub fn response_to_result_flat<T>(
    inner: Response<Response<T>>,
) -> SteamGridDbResult<Vec<SteamGridDbResult<T>>>
where
    T: Clone,
{
    if !inner.success {
        std::result::Result::Err(SteamGridDbError {
            errors: inner.errors,
            status: None,
        })
    } else {
        match inner.data {
            Some(data) => {
                let inner = data.iter().map(|i| {
                    if !i.success {
                        std::result::Result::Err(SteamGridDbError {
                            errors: i.errors.clone(),
                            status: i.status,
                        })
                    } else {
                        match &i.data {
                            Some(data) => {
                                let first = data.iter().next();
                                match first {
                                    Some(first) => Ok(first.clone()),
                                    None => std::result::Result::Err(SteamGridDbError {
                                        status: None,
                                        errors: Some(vec![
                                            "Succes reported but no grids found".to_string()
                                        ]),
                                    }),
                                }
                            }
                            None => std::result::Result::Err(SteamGridDbError {
                                status: None,
                                errors: i.errors.clone(),
                            }),
                        }
                    }
                });
                let inner_res = inner.collect();
                Ok(inner_res)
            }
            None => std::result::Result::Err(SteamGridDbError {
                status: None,
                errors: inner.errors,
            }),
        }
    }
}
