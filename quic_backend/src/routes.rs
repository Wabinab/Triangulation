use crate::*;

pub(crate) async fn handle_bi(send: SendStream, recv: RecvStream, path: String) {
    let _ = match path.as_str() {
        "/" => home_controller::bi_echo(send, recv).await,

        _ => home_controller::bi_do_nothing().await
    };
}

pub(crate) async fn handle_datagram(session: Session, recv: Bytes, path: String) {
    let _ = match path.as_str() {
        "/" => home_controller::dg_echo(session, recv).await,

        _ => home_controller::dg_do_nothing().await
    };
}