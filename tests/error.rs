use obce::{
    substrate::CriticalError,
    to_critical_error,
};

#[test]
fn error_macro_with_critical_works() {
    #[obce::error]
    pub enum Error<T> {
        NonCritical(T),

        #[obce(critical)]
        Critical(CriticalError),
    }

    let error: Result<(), _> = Err(Error::<u32>::Critical(CriticalError::BadOrigin));
    assert_eq!(to_critical_error!(error), Err(CriticalError::BadOrigin));

    let error: Result<(), _> = Err(Error::NonCritical(123));
    assert_eq!(to_critical_error!(error), Ok(Err(Error::NonCritical(123))));
}
