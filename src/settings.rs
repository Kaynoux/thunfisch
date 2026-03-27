pub mod settings {
    use crate::search::transposition_table::ReplacementStrategy;

    pub const AB: bool = true;
    pub const QS: bool = true;
    pub const TT_AB: bool = true;
    pub const TT_QS: bool = true;
    pub const MVV_LVA: bool = true;
    pub const QS_CHECK_EVASION_LIMIT: usize = 5; //  value to limit full check evasion generation in Quiescence Search in plies
    pub const ORDER_TT_MV_FIRST: bool = true;
    pub const DEPTH_PENALTY_PER_AGE: u8 = 2;
    pub const REPLACEMENT_STRATEGY: ReplacementStrategy = ReplacementStrategy::Aging;
    pub const TT_CUTTOFFS: bool = true;
}
