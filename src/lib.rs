#[macro_use]
extern crate nix;
extern crate drm_sys;

#[macro_use]
extern crate error_chain;

mod ffi;
mod result;

use std::os::unix::io::AsRawFd;
use result::Result;

#[derive(Debug)]
/// A token unique to the process that determines who opened the device.
///
/// This token can be sent to another process that acts as the DRM Master and
/// then authenticated to give extra privileges.
pub struct AuthToken(u32);

#[derive(Debug)]
/// Capabilities that the process understands.
///
/// These can be used to tell the DRM device what capabilities the process can
/// use.
pub enum ClientCapability {
    Stereo3D = ffi::DRM_CLIENT_CAP_STEREO_3D as isize,
    UniversalPlanes = ffi::DRM_CLIENT_CAP_UNIVERSAL_PLANES as isize,
    Atomic = ffi::DRM_CLIENT_CAP_ATOMIC as isize
}

/// A trait for all DRM devices.
pub trait Device : AsRawFd {
    /// Generates and returns a magic token unique to the current process. This
    /// token can be used to authenticate with the DRM Master.
    fn get_auth_token(&self) -> Result<AuthToken> {
        let mut raw: ffi::drm_auth_t = Default::default();
        unsafe {
            try!(ffi::ioctl_get_magic(self.as_raw_fd(), &mut raw));
        }
        Ok(AuthToken(raw.magic))
    }

    /// Tells the DRM device whether we understand or do not understand a
    /// particular capability. Some features, such as atomic modesetting,
    /// require informing the device that the process can use such features
    /// before it will expose them.
    fn set_client_cap(&self, cap: ClientCapability, set: bool) -> Result<()> {
        let mut raw: ffi::drm_set_client_cap = Default::default();
        raw.capability = cap as u64;
        raw.value = set as u64;
        unsafe {
            try!(ffi::ioctl_set_client_cap(self.as_raw_fd(), &mut raw));
        }
        Ok(())
    }
}
