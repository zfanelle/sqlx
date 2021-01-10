use crate::{ConnectOptions, Runtime};

pub trait Connect<Rt>
where
    Rt: Runtime,
{
    type Options: ConnectOptions<Rt, Connection = Self>;

    #[cfg(feature = "async")]
    fn connect(url: &str) -> futures_util::future::BoxFuture<'_, crate::Result<Self>>
    where
        Self: Sized,
        Rt: crate::Async,
        for<'s> <Rt as Runtime>::TcpStream: crate::io::Stream<'s, Rt>;
}

// TODO: impl Connect for Pool { ... }
// TODO: impl Connect for PgConnection { ... }