#[macro_export]
macro_rules! read_lock {
    ( $rw_lock:expr ) => {
    $rw_lock.read().unwrap()
  };
}
#[macro_export]
macro_rules! write_lock {
    ( $rw_lock:expr ) => {
    $rw_lock.write().unwrap()
  };
}
pub(crate) use read_lock;
pub(crate) use write_lock;

#[macro_export]
macro_rules! read_session {
    ( $session_map:expr, $session_id:expr ) => {
        crate::read_lock!($session_map.get(&$session_id).unwrap())
  };
}
#[macro_export]
macro_rules! write_session {
    ( $session_map:expr, $session_id:expr ) => {
        crate::write_lock!($session_map.get(&$session_id).unwrap())
  };
}
#[macro_export]
macro_rules! cast {
    ( $packet:expr, $type:ty ) => {
       $packet.as_any().downcast_ref::<$type>().unwrap();
  };
}

pub(crate) use read_session;
pub(crate) use write_session;