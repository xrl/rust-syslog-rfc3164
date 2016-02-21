#[derive(Copy,Clone,Debug,PartialEq)]
#[allow(non_camel_case_types)]
pub enum SyslogSeverity {
    SEV_EMERG = 0,
    SEV_ALERT = 1,
    SEV_CRIT = 2,
    SEV_ERR = 3,
    SEV_WARNING = 4,
    SEV_NOTICE = 5,
    SEV_INFO = 6,
    SEV_DEBUG = 7,
}

impl SyslogSeverity {
    pub fn from_int(i: i32) -> Option<SyslogSeverity> {
        match i {
            0 => Some(SyslogSeverity::SEV_EMERG),
            1 => Some(SyslogSeverity::SEV_ALERT),
            2 => Some(SyslogSeverity::SEV_CRIT),
            3 => Some(SyslogSeverity::SEV_ERR),
            4 => Some(SyslogSeverity::SEV_WARNING),
            5 => Some(SyslogSeverity::SEV_NOTICE),
            6 => Some(SyslogSeverity::SEV_INFO),
            7 => Some(SyslogSeverity::SEV_DEBUG),
            _ => None,
        }
    }
}
