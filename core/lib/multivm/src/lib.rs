// #![deny(unreachable_pub)]
#![deny(unused_crate_dependencies)]
#![warn(unused_extern_crates)]
#![warn(unused_imports)]

pub use zk_evm_1_3_1;
pub use zk_evm_1_3_3;
pub use zk_evm_1_4_0;
pub use zksync_types::vm_version::VmVersion;

pub use self::versions::{
    vm_1_3_2, vm_latest, vm_m5, vm_m6, vm_refunds_enhancement, vm_virtual_blocks,
};
pub use crate::{
    glue::{
        history_mode::HistoryMode,
        tracers::{MultiVMTracer, MultiVmTracerPointer},
    },
    vm_instance::VmInstance,
};

mod glue;
pub mod interface;
pub mod tracers;
pub mod versions;
mod vm_instance;
