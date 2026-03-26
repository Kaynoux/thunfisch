pub mod settings {

    pub const AB: bool = true;
    pub const QS: bool = true;
    pub const TT_AB: bool = true;
    pub const TT_QS: bool = true;
    pub const MVV_LVA: bool = true;
    pub const QS_CHECK_EVASION_LIMIT: usize = 2; //  value to limit full check evasion generation in Quiescence Search in plies
    pub const ORDER_TT_MV_FIRST: bool = true;
    pub const TT_CUTTOFFS: bool = true;
    pub const NULL_MOVE_PRUNING: bool = true;
}
