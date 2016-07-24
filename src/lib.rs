extern crate rustc_serialize;
use rustc_serialize::json;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}


/*
 * EVENT OBJECT
 */
#[derive(RustcDecodable, RustcEncodable)]
struct ChannelEventObject {
    id: String,
    name: String,
    created: u32,
    creator: String,
}


/*
 * EVENTS
 */

#[derive(RustcDecodable, RustcEncodable)]
struct HelloEvent { 
}
#[derive(RustcDecodable, RustcEncodable)]
struct UserTypingEvent {
    channel: String,
    user: String,
}
/// Sent to all connections of a User when the read-cursor is advanced
/// _Useful if multiple instances of a user connected at once._
#[derive(RustcDecodable, RustcEncodable)]
struct ChannelMarkedEvent {
    channel: String,
    ts: u32,
}
#[derive(RustcDecodable, RustcEncodable)]
struct ChannelCreatedEvent {
    channel: ChannelEventObject,
}
/// Sent to all connections of a User when a channel is joined.
#[derive(RustcDecodable, RustcEncodable)]
struct ChannelJoinedEvent {
    channel: ChannelEventObject,
}
/// Sent to all connections of a User when a channel is left
#[derive(RustcDecodable, RustcEncodable)]
struct ChannelLeftEvent {
    channel: String,
}
#[derive(RustcDecodable, RustcEncodable)]
struct ChannelDeletedEvent {
    channel: String,
}

mod message_event {
    /// The basic message that has no sub-type
    #[derive(RustcDecodable, RustcEncodable)]
    struct Basic {
        channel: String,
        user: String,
        text: String,
        ts: u32,
    }

    /// Sent to all existing members of a channel when someone new joins
    type ChannelJoinEvent = Basic;
    /// Sent to all existing members of a channel when someone leaves
    type ChannelLeaveEvent = Basic;
}
