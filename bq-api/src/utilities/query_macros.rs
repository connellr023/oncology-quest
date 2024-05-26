#[macro_export]
macro_rules! query_many {
    ($transaction:expr, $domain_id:expr, $($query:expr),+ $(,)?) => {
        $(
            sqlx::query($query)
                .bind($domain_id)
                .execute($transaction)
                .await?;
        )+
    };
}