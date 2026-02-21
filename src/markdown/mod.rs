pub mod codeblocks;
pub mod toc;

// Re-export commonly used functions
pub use codeblocks::{
    apply_codeblock_updates, check_codeblocks_up_to_date, find_codeblock_markers,
};
pub use toc::{apply_toc_updates, check_toc_up_to_date, extract_headings, find_toc_regions};
