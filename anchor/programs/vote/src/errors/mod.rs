use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Start date can not be greater than end date")]
    InvalidDates,
    #[msg("Poll does not exist or not found")]
    PollDoesNotExist,
    #[msg("Candidate can not register twice")]
    CandidateAlreadyRegistered,
    #[msg("Candidate is not in the poll")]
    CandidateNotRegistered,
    #[msg("Voter can not vote twice")]
    VoterAlreadyVoted,
    #[msg("Poll not active")]
    PollNotActive,
}
