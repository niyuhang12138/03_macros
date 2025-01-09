use macros::AutoDebug;

#[allow(dead_code)]
#[derive(AutoDebug)]
struct RespBulkString {
    inner: String,
    #[debug(skip)]
    nothing: (),
}

fn main() {}
