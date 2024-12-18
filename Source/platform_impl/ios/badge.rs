use objc::{
	msg_send,
	runtime::{Class, Object},
	sel,
	sel_impl,
};

pub fn set_badge_count(count:i32) {
	unsafe {
		let ui_application =
			Class::get("UIApplication").expect("Failed to get UIApplication class");

		let app:*mut Object = msg_send![ui_application, sharedApplication];

		let _:() = msg_send![app, setApplicationIconBadgeNumber:count];
	}
}
