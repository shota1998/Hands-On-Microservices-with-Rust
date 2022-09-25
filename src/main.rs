
fn main() {
    multiple();
}

fn multiple() {
    let (tx_sink, rx_stream) = mpsc::channel::<u8>(8);

}