use std::ffi::CStr;
use ffi::*;

pub trait VPXCodec {
    fn get_context<'a>(&'a mut self) -> &'a mut vpx_codec_ctx;

    /// Return a human-readable representation of the last error occurred
    fn error_to_str(&mut self) -> String {
        unsafe {
            let c_str = vpx_codec_error(self.get_context());

            CStr::from_ptr(c_str).to_string_lossy().into_owned()
        }
    }
}
