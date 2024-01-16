use super::super::to_io_result;
use libc::{timespec, FD_ISSET};
use std::io;
use std::os::fd::RawFd;
use std::time::Duration;
use std::{mem, ptr};

// macOS does not support polling /dev/tty using kqueue, so we have to
// resort to pselect/select. See https://nathancraddock.com/blog/macos-dev-tty-polling/.
pub(crate) fn poll_read(fd: RawFd, timeout: Duration) -> io::Result<()> {
    let mut readfds = unsafe { std::mem::zeroed() };
    let timeout = to_timespec(timeout);
    unsafe { libc::FD_SET(fd, &mut readfds) };
    to_io_result(unsafe {
        libc::pselect(
            fd + 1,
            &mut readfds,
            ptr::null_mut(),
            ptr::null_mut(),
            &timeout,
            ptr::null(),
        )
    })?;
    if unsafe { FD_ISSET(fd, &readfds) } {
        Ok(())
    } else {
        todo!("timeout")
    }
}

const fn to_timespec(duration: Duration) -> timespec {
    let mut ts: timespec = unsafe { mem::zeroed() };
    ts.tv_sec = duration.as_secs() as libc::time_t;
    #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
    {
        ts.tv_nsec = duration.subsec_nanos() as i64;
    }
    #[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))]
    {
        ts.tv_nsec = duration.subsec_nanos() as libc::c_long;
    }
    ts
}
