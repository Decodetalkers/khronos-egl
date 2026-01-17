pub use r_egl;
use r_egl as egl;
use wayland_client::{Proxy, protocol::wl_display::WlDisplay};

pub use egl::api::EGL1_5;

pub use egl::API as EGL_INSTALCE;

/// The trait help reduce unsafe
pub trait WayEglTrait {
	fn get_display_wl(&self, display: &WlDisplay) -> Option<egl::Display>;
}

impl<T: egl::api::EGL1_5> WayEglTrait for egl::Instance<T> {
	fn get_display_wl(&self, display: &WlDisplay) -> Option<egl::Display> {
		unsafe { self.get_display(display.id().as_ptr() as *mut std::ffi::c_void) }
	}
}
