
extern crate encoding;

extern crate cqpsdk;

use std::ffi::{ CString, CStr };

use encoding::{ Encoding, EncoderTrap, DecoderTrap };
use encoding::all::{ UTF_8, GBK };

use cqpsdk::cqpapi;

// Some macros for convenience.
// Should have been integrated in evshiron/cqpsdk-rust.
// But just leave here for easy reading.

macro_rules! gbk {

	( $x: expr ) => (CString::new(GBK.encode($x, EncoderTrap::Ignore).unwrap()).unwrap().into_raw());
	
}

macro_rules! utf8 {
	
	( $x: expr ) => (&GBK.decode(CStr::from_ptr($x).to_bytes(), DecoderTrap::Ignore).unwrap()[..]);
	
}

static mut AUTH_CODE: i32 = 0;

// https://github.com/rust-lang/rust/issues/17806

#[export_name="\x01_AppInfo"]
pub extern "stdcall" fn app_info() -> *const i8 {
	
	return gbk!("9,com.github.res.pupurium_r");

}

#[export_name="\x01_Initialize"]
pub extern "stdcall" fn initialize(AuthCode: i32) -> i32 {
	
	unsafe {

		AUTH_CODE = AuthCode;

	}

	return cqpapi::EVENT_IGNORE;

}

#[export_name="\x01_PrivateMessageHandler"]
pub extern "stdcall" fn private_message_handler(subType: i32, sendTime: i32, qqNum: i64, msg: *const i8, font: i32) -> i32 {
	
	return cqpapi::EVENT_IGNORE;

}

#[export_name="\x01_GroupMessageHandler"]
pub extern "stdcall" fn group_message_handler(subType: i32, sendTime: i32, groupNum: i64, qqNum: i64, anonymousName: *const i8, msg: *const i8, font: i32) -> i32 {
	
	return cqpapi::EVENT_IGNORE;

}
