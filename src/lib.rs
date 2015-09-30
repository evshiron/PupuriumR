extern crate cqpsdk;
use cqpsdk::cqpapi;

static mut AUTH_CODE: i32 = 0;

// https://github.com/rust-lang/rust/issues/17806

#[export_name="\x01_AppInfo"]
pub extern "stdcall" fn app_info() -> *const u8 {
	
	return "9,com.github.res.pupurium_r".as_ptr();

}

#[export_name="\x01_Initialize"]
pub extern "stdcall" fn initialize(AuthCode: i32) -> i32 {
	
	//println!("Initialize.");

	unsafe {

		AUTH_CODE = AuthCode;

	}

	return 0;

}

#[export_name="\x01_PrivateMessageHandler"]
pub extern "stdcall" fn private_message_handler(subType: i32, sendTime: i32, QQID: i64, msg: *const u8, font: i32) -> i32 {
	
	unsafe {

		cqpapi::CQ_sendPrivateMsg(AUTH_CODE, QQID, "Reply!".as_ptr());

	}

	return 0;

}
