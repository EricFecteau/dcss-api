use std::collections::VecDeque;

use crate::CrawlData;

#[derive(Debug)]
pub(crate) struct Log {
    pub(crate) log: VecDeque<String>,
}

impl Log {
    pub(crate) fn init() -> Self {
        Self {
            log: VecDeque::new(),
        }
    }
}

impl CrawlData {
    pub fn get_log_message(&mut self) -> Option<String> {
        self.log.log.pop_front()
    }

    pub fn logs_to_process(&mut self) -> bool {
        !self.log.log.is_empty()
    }
}
