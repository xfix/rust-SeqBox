macro_rules! worker_stop {
    (
        graceful => $tx_error:path, $shutdown_flag:path $([$( $x:path ),*]),*
    ) => {{
        use std::sync::atomic::Ordering;
        use misc_utils::ignore;
        ignore($tx_error.send(None));
        $( $( ignore($x.send(None)); )* )*
        $shutdown_flag.store(true, Ordering::Relaxed);
        break;
    }};
    (
        graceful_ret => $tx_error:path, $shutdown_flag:path $([$( $x:path ),*]),*
    ) => {{
        use std::sync::atomic::Ordering;
        use misc_utils::ignore;
        ignore($tx_error.send(None));
        $( $( ignore($x.send(None)); )* )*
        $shutdown_flag.store(true, Ordering::Relaxed);
        return;
    }};
    (
        graceful_if ($cond:expr) => $tx_error:path, $shutdown_flag:path $([$( $x:path ),*]),*
    ) => {{
        if $cond {
            worker_stop!(graceful => $tx_error, $shutdown_flag $([$( $x ),*]),*)
        }
    }};
    (
        graceful_if_ret ($cond:expr) => $tx_error:path, $shutdown_flag:path $([$( $x:path ),*]),*
    ) => {{
        if $cond {
            worker_stop!(graceful_ret => $tx_error, $shutdown_flag $([$( $x ),*]),*)
        }
    }};
    (
        graceful_if_shutdown => $tx_error:path, $shutdown_flag:path $([$( $x:path ),*]),*
    ) => {{
        use std::sync::atomic::Ordering;
        worker_stop!(graceful_if ($shutdown_flag.load(Ordering::Relaxed)) =>
                     $tx_error, $shutdown_flag $([$( $x ),*]),*)
    }};
    (
        graceful_if_shutdown_ret => $tx_error:path, $shutdown_flag:path $([$( $x:path ),*]),*
    ) => {{
        use std::sync::atomic::Ordering;
        worker_stop!(graceful_if_ret ($shutdown_flag.load(Ordering::Relaxed)) =>
                     $tx_error, $shutdown_flag $([$( $x ),*]),*)
    }};
    (
        with_error $error:expr => $tx_error:path, $shutdown_flag:path $([$( $x:path ),*]),*
    ) => {{
        use std::sync::atomic::Ordering;
        use misc_utils::ignore;
        ignore($tx_error.send(Some($error)));
        $( $( ignore($x.send(None)); )* )*
        $shutdown_flag.store(true, Ordering::Relaxed);
        break;
    }};
    (
        with_error_if_fail ($expr:expr) => $tx_error:path, $shutdown_flag:path $([$( $x:path ),*]),*
    ) => {{
        match $expr {
            Ok(res) => res,
            Err(e)  => worker_stop!(with_error e => $tx_error, $shutdown_flag)
        }
    }};
    (
        with_error_ret $error:expr => $tx_error:path, $shutdown_flag:path $([$( $x:path ),*]),*
    ) => {{
        use std::sync::atomic::Ordering;
        use misc_utils::ignore;
        ignore($tx_error.send(Some($error)));
        $( $( ignore($x.send(None)); )* )*
        $shutdown_flag.store(true, Ordering::Relaxed);
        return;
    }}
}

macro_rules! send {
    (
        no_back_off $item:expr => $sender:ident, $tx_error:path, $shutdown_flag:path $([$( $x:path ),*]),*
    ) => {{
        match $sender.send($item) {
            Ok(()) => {},
            Err(_) => worker_stop!(graceful => $tx_error, $shutdown_flag $([$( $x ),*]),*)
        }
    }};
    (
        no_back_off_ret $item:expr => $sender:ident, $tx_error:path, $shutdown_flag:path $([$( $x:path ),*]),*
    ) => {{
        match $sender.send($item) {
            Ok(()) => {},
            Err(_) => worker_stop!(graceful_ret => $tx_error, $shutdown_flag $([$( $x ),*]),*)
        }
    }};
    (
        back_off_millis $time:expr, $item:expr => $sender:ident, $tx_error:path, $shutdown_flag:path $([$( $x:path ),*]),*
    ) => {{
        use std::time::Duration;
        use std::sync::mpsc::TrySendError;
        use std::sync::atomic::Ordering;
        let mut secondary_buffer = Some($item);
        let mut force_shutdown = false;
        loop {
            let item = match secondary_buffer {
                None    => { break; },
                Some(b) => b,
            };
            match $sender.try_send(item) {
                Ok(()) => { break; },
                Err(TrySendError::Full(b)) => {
                    thread::sleep(Duration::from_millis($time));
                    if $shutdown_flag.load(Ordering::Relaxed) {
                        break;
                    }
                    secondary_buffer = Some(b);
                },
                Err(TrySendError::Disconnected(_)) => {
                    force_shutdown = true; break;
                }
            }
        }
        if force_shutdown {
            worker_stop!(graceful =>
                         $tx_error, $shutdown_flag $([$( $x ),*]),*)
        } else {
            worker_stop!(graceful_if_shutdown =>
                         $tx_error, $shutdown_flag $([$( $x ),*]),*);
        }
    }};
    (
        back_off $item:expr => $sender:ident, $tx_error:path, $shutdown_flag:path $([$( $x:path ),*]),*
    ) => {{
        send!(back_off_millis 100, $item => $sender, $tx_error, $shutdown_flag $([$( $x ),*]),*)
    }}
}

macro_rules! recv {
    (
        no_timeout => $receiver:ident, $tx_error:path, $shutdown_flag:path $([$( $x:path ),*]),*
    ) => {{
        match $receiver.recv() {
            Ok(item) => item,
            Err(_)   => worker_stop!(graceful => $tx_error, $shutdown_flag $([$( $x ),*]),*)
        }
    }};
    (
        no_timeout_shutdown_if_none => $receiver:ident, $tx_error:path, $shutdown_flag:path $([$( $x:path ),*]),*
    ) => {{
        match recv!(no_timeout => $receiver,
                    $tx_error, $shutdown_flag $([$( $x ),*]),*) {
            Some(x) => x,
            None    => worker_stop!(graceful => $tx_error, $shutdown_flag $([$( $x ),*]),*)
        }
    }};
    (
        timeout_millis $timeout:expr => $receiver:ident, $tx_error:path, $shutdown_flag:path $([$( $x:path ),*]),*
    ) => {{
        use std::time::Duration;
        use std::sync::mpsc::RecvTimeoutError;
        match $receiver.recv_timeout(Duration::from_millis($timeout)) {
            Ok(item)                            => item,
            Err(RecvTimeoutError::Timeout)      => {
                worker_stop!(graceful_if_shutdown => $tx_error, $shutdown_flag $([$( $x ),*]),*);
                continue;
            },
            Err(RecvTimeoutError::Disconnected) => {
                worker_stop!(graceful =>
                             $tx_error, $shutdown_flag $([$( $x ),*]),*);
            }
        }
    }};
    (
        timeout => $receiver:ident, $tx_error:path, $shutdown_flag:path $([$( $x:path ),*]),*
    ) => {{
        recv!(timeout_millis 1000 => $receiver, $tx_error, $shutdown_flag $([$( $x ),*]),*)
    }};
}