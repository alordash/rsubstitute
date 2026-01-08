use syn::ItemImpl;

pub struct SendSyncImpls {
    pub send_impl: ItemImpl,
    pub sync_impl: ItemImpl
}