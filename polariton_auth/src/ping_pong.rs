use polariton::packet::Ping;

#[inline]
pub fn ping_pong(mut ping: Ping) -> Ping {
    if ping.tick2.is_some() {
        ping.tick2 = None;
    } else {
        ping.tick2 = Some(ping.tick1);
    }
    ping
}
