use crate::*;

pub(crate) async fn bi_do_nothing() -> Result<()> {
    Ok(())
}
pub(crate) async fn dg_do_nothing() -> Result<()> {
    Ok(())
}


pub(crate) async fn bi_echo(send: SendStream, recv: RecvStream) -> Result<()> {
    let (mut send, mut recv) = (send, recv);

    let msg = recv.read_to_end(1024).await?;
    send.write_all(&msg).await?;

    Ok(())
}
pub(crate) async fn dg_echo(session: Session, recv: Bytes) -> Result<()> {
    session.send_datagram(recv.clone()).await?;

    Ok(())
}

