mod login;
pub use login::login;
mod register;
pub use register::register;
mod validate_otp;
pub use validate_otp::validate_otp;
mod validate_token;
pub use validate_token::validate_token;
mod refresh_token;
pub use refresh_token::refresh_token;
mod logout;
pub use logout::logout;
mod generate_magic_link;
pub use generate_magic_link::generate_magic_link;

