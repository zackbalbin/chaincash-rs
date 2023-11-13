use ergo_lib::ergoscript_compiler::compiler::compile;
use ergo_lib::ergotree_ir::ergo_tree::ErgoTree;
use once_cell::sync::Lazy;

pub(crate) static RESERVE_CONTRACT: &str =
    include_str!("../../../contracts/chaincash/contracts/onchain/reserve.es");
pub(crate) static NOTE_CONTRACT: &str =
    include_str!("../../../contracts/chaincash/contracts/onchain/note.es");

pub(crate) static RESERVE_ERGO_TREE: Lazy<ErgoTree> =
    Lazy::new(|| compile(RESERVE_CONTRACT, Default::default()).unwrap());

pub(crate) static NOTE_ERGO_TREE: Lazy<ErgoTree> =
    Lazy::new(|| compile(NOTE_CONTRACT, Default::default()).unwrap());
