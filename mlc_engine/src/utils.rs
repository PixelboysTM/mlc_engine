#[macro_export]
macro_rules! send {
    ($info:expr, $msg:expr) => {
        let _ = $info.send($msg);
    };
}
