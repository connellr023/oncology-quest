#[macro_export]
macro_rules! query_many {
    ($transaction:expr, $arg:expr, $($query:expr), + $(,)?) => {
        $(
            sqlx::query!(
                $query,
                $arg
            )
            .execute($transaction)
            .await?;
        )+
    };
}