use crate::error_printing;

pub trait IMockData {
    fn get_received_nothing_else_error_msgs(&self) -> Vec<Vec<String>>;

    fn verify_received_nothing_else(&self) {
        let all_error_msgs: Vec<_> = self.get_received_nothing_else_error_msgs();
        let error_msgs: Vec<_> = all_error_msgs.into_iter().flatten().collect();
        if error_msgs.is_empty() {
            return;
        }
        error_printing::panic_received_unexpected_calls_error(error_msgs);
    }
}
