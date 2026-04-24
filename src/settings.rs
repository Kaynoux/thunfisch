pub const AB: bool = cfg!(feature = "ab");
pub const QS: bool = cfg!(feature = "qs");
pub const TT_AB: bool = cfg!(feature = "tt-ab");
pub const TT_QS: bool = cfg!(feature = "tt-qs");
pub const MVV_LVA: bool = cfg!(feature = "mvv-lva");
pub const ORDER_TT_MV_FIRST: bool = cfg!(feature = "order-tt-mv-first");
pub const TT_CUTTOFFS: bool = cfg!(feature = "tt-cuttoffs");
pub const NMP: bool = cfg!(feature = "nmp");
pub const RFP: bool = cfg!(feature = "rfp");
pub const PVS: bool = cfg!(feature = "pvs");
pub const KILLERS: bool = cfg!(feature = "killers");
pub const HISTORIES: bool = cfg!(feature = "histories");

// Can be tweaked, hava an effect on elo
pub const QS_CHECK_EVASION_LIMIT: usize = 2;
pub const MAX_QS_DEPTH: usize = 12;
// Maximum search depth. In practice likely never reached, but has an effect on memory usage of the program
pub const MAX_AB_DEPTH: usize = 128;
// How far a static eval needs to be over beta to initiate an RFP cutoff
pub const RFP_MARGIN: usize = 50;

#[inline]
pub fn repr() -> String {
    format!(
        "Activated Features: AB={AB:?} QS={QS:?} TT-AB={TT_AB:?} TT-QS={TT_QS:?} MVV-LVA={MVV_LVA:?} QS_CHECK_EVASION_LIMIT={QS_CHECK_EVASION_LIMIT:?} ORDER_TT_MV_FIRST={ORDER_TT_MV_FIRST:?} TT_CUTTOFFS={TT_CUTTOFFS:?} NMP={NMP:?} RFP={RFP:?} PVS={PVS:?}, KILLERS={KILLERS:?}"
    )
}
