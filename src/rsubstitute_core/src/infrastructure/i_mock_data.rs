use crate::infrastructure::*;

pub trait IMockData {
    fn get_received_nothing_else_error_msgs<const N: usize>(
        &self,
        fn_idents: [&'static str; N],
    ) -> Vec<Vec<String>>;

    fn verify_received_nothing_else<const N: usize>(&self, fn_idents: [&'static str; N]) {
        let all_error_msgs: Vec<_> = self.get_received_nothing_else_error_msgs(fn_idents);
        if all_error_msgs.is_empty() {
            return;
        }
        let error_msgs: Vec<_> = all_error_msgs.into_iter().flatten().collect();
        error_printing::panic_received_unexpected_calls_error(error_msgs);
    }
}
