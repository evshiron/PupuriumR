
extern crate winapi;
extern crate user32;
extern crate encoding;

extern crate cqpsdk;

use std::ffi::CString;
use std::ffi::CStr;

use encoding::{Encoding, EncoderTrap, DecoderTrap};
use encoding::all::{UTF_8, GBK};

use cqpsdk::cqpapi;

macro_rules! gbk {

	( $x: expr ) => (CString::new(GBK.encode($x, EncoderTrap::Ignore).unwrap()).unwrap().as_ptr());
	
}

macro_rules! utf8 {
	
	( $x: expr ) => (&GBK.decode(CStr::from_ptr($x).to_bytes(), DecoderTrap::Ignore).unwrap()[..]);
	
}

static mut AUTH_CODE: i32 = 0;

// https://github.com/rust-lang/rust/issues/17806

#[export_name="\x01_AppInfo"]
pub extern "stdcall" fn app_info() -> *const i8 {
	
	return "9,com.github.res.pupurium_r".as_ptr();

}

#[export_name="\x01_Initialize"]
pub extern "stdcall" fn initialize(AuthCode: i32) -> i32 {
	
	//println!("Initialize.");

	unsafe {

		user32::MessageBoxA(std::ptr::null_mut(), gbk!("PupuriumR初始化完毕。"), gbk!("PupuriumR初始化完毕。"), 0);

		AUTH_CODE = AuthCode;

	}

	return 0;

}

#[export_name="\x01_PrivateMessageHandler"]
pub extern "stdcall" fn private_message_handler(subType: i32, sendTime: i32, qqNumber: i64, msg: *const i8, font: i32) -> i32 {
	
	unsafe {

		let msg = utf8!(msg);

		cqpapi::CQ_addLog(AUTH_CODE, cqpapi::CQLOG_INFO, gbk!(msg), gbk!(msg));

		cqpapi::CQ_sendPrivateMsg(AUTH_CODE, qqNumber, gbk!(&format!("你刚才说：{}", msg)[..]));

		match msg {

			"Alive?" => {

				cqpapi::CQ_sendPrivateMsg(AUTH_CODE, qqNumber, gbk!("Alive."));

			},
			_ => {

				return cqpapi::EVENT_IGNORE;

			}

		}

	}

	return cqpapi::EVENT_IGNORE;

}

#[export_name="\x01_GroupMessageHandler"]
pub extern "stdcall" fn group_message_handler(subType: i32, sendTime: i32, groupNumber: i64, qqNumber: i64, anonymousName: *const i8, msg: *const i8, font: i32) -> i32 {
	
	return cqpapi::EVENT_IGNORE;

}
