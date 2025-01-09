use macros::AutoDeref;

#[allow(dead_code)]
#[derive(AutoDeref)]
#[deref(mutable = true, field = "inner")]
struct RespBulkString {
    inner: String,
}

fn main() {}
