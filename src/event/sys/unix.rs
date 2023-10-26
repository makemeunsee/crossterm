#[cfg(feature = "event-stream")]
#[cfg(not(target_arch = "wasm32"))] // TODO: spin off parse instead.
pub(crate) mod waker;

#[cfg(feature = "events")]
pub(crate) mod parse;
