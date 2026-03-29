pub mod settings {
    pub const AB: bool = cfg!(feature = "ab");
    pub const QS: bool = cfg!(feature = "qs");
    pub const TT_AB: bool = cfg!(feature = "tt-ab");
    pub const TT_QS: bool = cfg!(feature = "tt-qs");
    pub const MVV_LVA: bool = cfg!(feature = "mvv-lva");
    pub const QS_CHECK_EVASION_LIMIT: usize = 2;
    pub const ORDER_TT_MV_FIRST: bool = cfg!(feature = "order-tt-mv-first");
    pub const TT_CUTTOFFS: bool = cfg!(feature = "tt-cuttoffs");
    pub const NULL_MOVE_PRUNING: bool = cfg!(feature = "nmp");
    pub const REVERSE_FUTILITY_PRUNING: bool = cfg!(feature = "rfp");
}
