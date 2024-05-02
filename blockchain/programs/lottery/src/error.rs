use anchor_lang::error_code;

#[error_code]
pub enum LotteryError {
    #[msg("Winner already exist!")]
    WinnerAlreadyExist,
    #[msg("Can not choose winner when there is no tickets!")]
    NoTickets,
    #[msg("Winner has not been chosen!")]
    WinnerNotChosen,
    #[msg("Invalid winner!")]
    InvalidWinner,
    #[msg("The prize has already been claimed!")]
    AlreadyClaim,
}
