use std::{error, fmt};

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// Errors from the server
pub struct SteamGridDbError {
    /// The error status code
    pub status: Option<u32>,
    /// The error messages
    pub errors: Option<Vec<String>>,
}
impl fmt::Display for SteamGridDbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "API reported status: {:?} with errors: {:?}",
            self.status, self.errors
        )
    }
}

impl error::Error for SteamGridDbError {}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// Steamgriddb response type
pub struct Response<T> {
    /// Was the request a success?
    pub success: bool,
    /// The response data
    pub data: Option<T>,
    /// The status code of the response
    pub status: Option<u32>,
    /// Any errors that occurred
    pub errors: Option<Vec<String>>,
}

pub type SteamGridDbResult<T> = std::result::Result<T, SteamGridDbError>;

/// Converts the reponse to a result, that is easier to work with
pub fn response_to_result<T>(inner: Response<Vec<T>>) -> SteamGridDbResult<Vec<T>> {
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

/// Converts the reponse to a result, that is easier to work with.
/// This will also return a single list of resutls instead of a list of lists 
/// (since there are only one element in the list anyways)
pub fn response_to_result_flat<T>(
    inner: Response<Vec<Response<Vec<T>>>>,
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
