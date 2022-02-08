///! Authentication helper utilities

/// Returns the value to set in the `AUTHORIZATION` header for a request.
pub fn auth_token(token: &str) -> String {
    // "Bearer ".len() == 7
    let mut bearer_token = String::with_capacity(7 + token.len());
    bearer_token.push_str("Bearer ");
    bearer_token.push_str(token);

    bearer_token
}
