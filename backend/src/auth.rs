// OAuth and JWT authentication utilities
// TODO: Implement full Google OAuth flow with openidconnect crate

pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
}
