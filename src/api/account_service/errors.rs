use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum AccountError {
    Nothing,
    PassNotMatched,
    UserNotExists,
    DatabaseError,
    UsernameAlreadyExists,
    EmailAlreadyExists,
}
