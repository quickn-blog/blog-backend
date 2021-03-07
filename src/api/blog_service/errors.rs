use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum BlogError {
    Nothing,
    AuthError,
    DatabaseError,
    NetworkError,
    PermissionError,
}
