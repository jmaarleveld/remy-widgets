mod core;


use cfg_if::cfg_if;

cfg_if!{
    if #[cfg(feature = "backend-crossterm")] {
        mod crossterm;
    } else if #[cfg(feature = "backend-termion")] {

    } else if #[cfg(feature = "backend-termwiz")] {
        
    } else if #[cfg(feature = "backend-test")] {
        
    } else {
        compile_error!("At least one backend must be enabled");
    }
}

pub use core::{
    GenericEvent as Event,
    GenericKeyEventKind as KeyEventKind,
    Key
};
