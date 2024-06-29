pub(super) mod create_entries;
pub(super) mod update_entries;
pub(super) mod delete_entries;
pub(super) mod get_entries;

#[macro_export]
macro_rules! entry_wrapper {
    ($pool:ident, $claim:ident, $block:block) => {
        if !$claim.sub.is_admin {
            return HttpResponse::Unauthorized().finish();
        }
    
        $block
    
        HttpResponse::Ok().finish()
    };
}