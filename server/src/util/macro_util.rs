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
#[macro_export]
macro_rules! mutex_lock {
    ( $mutex:expr ) => {
    $mutex.lock().unwrap()
  };
}

#[macro_export]
macro_rules! cast {
    ( $packet:expr, $type:ty ) => {
       $packet.as_any().downcast_ref::<$type>().unwrap()
  };
}

#[macro_export]
macro_rules! socket_send {
    ( $tcp_stream:expr, $data:expr ) => {
    {
       let mut tcp_stream_guard = $tcp_stream.write().unwrap();
        debug!("Respond with: {:02X?}", $data);
        tcp_stream_guard.write_all($data).unwrap();
        tcp_stream_guard.flush().unwrap();
    }
  }
}

#[macro_export]
macro_rules! elapsed {
    ( $instant:expr ) => {
       info!("{} ms", $instant.elapsed().as_nanos() as f64 / 1000.0 / 1000.0);
  };
}