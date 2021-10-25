use flexi_logger::DeferredNow;
use flexi_logger::filter::{LogLineFilter, LogLineWriter};

pub struct LogFilter {
    pub exclude_pattern: String,
}

impl LogFilter {
    pub fn new() -> LogFilter {
        LogFilter {
            exclude_pattern: "sqlx".to_string()
        }
    }
}

impl LogLineFilter for LogFilter {
    fn write(
        &self,
        now: &mut DeferredNow,
        record: &log::Record,
        log_line_writer: &dyn LogLineWriter,
    ) -> std::io::Result<()> {
        // TODO apply pattern
        if !(record.module_path().is_some() && record.module_path().unwrap().to_string().contains(&self.exclude_pattern)) {
            log_line_writer.write(now, record)?;
        }
        Ok(())
    }
}