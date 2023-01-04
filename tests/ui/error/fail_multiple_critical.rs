#[obce::error]
enum Error {
    One(u32),
    #[obce(critical)]
    Two(CriticalError),
    #[obce(critical)]
    Three(CriticalError)
}

fn main() {}
