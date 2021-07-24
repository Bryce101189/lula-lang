use crate::token::Position;

pub fn display_general_error<S>(subject: &str, message: S, position: Position)
where
    S: Into<String>,
{
    eprintln!("{} error, {}:\n    {}.", subject, position, message.into());
}
