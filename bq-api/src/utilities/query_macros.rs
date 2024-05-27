#[macro_export]
macro_rules! domain_query_many {
    ($transaction:expr, $domain_id:expr, $($query:expr), + $(,)?) => {
        $(
            sqlx::query!(
                $query,
                $domain_id
            )
            .execute($transaction)
            .await?;
        )+
    };
}