pub use consumer_error::*;
pub use event::*;
pub use incoming_message::*;
pub use memphis_consumer::*;
pub use memphis_consumer_options::*;

use crate::helper::memphis_util::get_internal_name;

mod consumer_error;
mod event;
mod incoming_message;
mod memphis_consumer;
mod memphis_consumer_options;

fn get_effective_consumer_name(options: &MemphisConsumerOptions) -> String {
    if options.consumer_group.is_empty() {
        get_internal_name(&options.consumer_name)
    } else {
        get_internal_name(&options.consumer_group)
    }
}
