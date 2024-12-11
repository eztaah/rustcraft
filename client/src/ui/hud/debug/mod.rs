pub mod blocks;
pub mod chunks;
pub mod coords;
pub mod fps;
mod loaded_stats;
pub mod setup;
pub mod targeted_block;

use bevy::prelude::Resource;
pub use blocks::*;
pub use chunks::*;
pub use coords::*;
pub use fps::*;
pub use loaded_stats::*;
pub use setup::*;

#[derive(Resource, Default)]
pub struct DebugOptions {
    is_chunk_debug_mode_enabled: bool,
}

impl DebugOptions {
    pub fn toggle_chunk_debug_mode(&mut self) {
        self.is_chunk_debug_mode_enabled = !self.is_chunk_debug_mode_enabled;
    }
}