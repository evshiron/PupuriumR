
extern crate winapi;
extern crate user32;
extern crate encoding;

#[macro_use]
extern crate cqpsdk;

use std::ffi::CString;
use std::ffi::CStr;

use encoding::{Encoding, EncoderTrap, DecoderTrap};
use encoding::all::{UTF_8, GBK};

use cqpsdk::cqpapi;

struct Pupurium {

	IsInitialized: bool,
	AuthCode: i32

}

static mut Pupurium: Pupurium = Pupurium {

	IsInitialized: false,
	AuthCode: 0

};

fn welcomeResistance(groupNum: i64, qqNum: i64) {
	
	unsafe {

		cqpapi::CQ_sendGroupMsg(Pupurium.AuthCode, groupNum, gbk!(&format!(r#"欢迎新人 [CQ:at,qq={}]！
建议玩家使用英文界面方便交流（不要吐槽英文界面哪里方便交流...）
右上角 目录 -> 设备 -> 语言 -> English 即可
请务必完成新手四项任务：
  1.修改群名片（群名片格式：游戏等级-游戏id-活动区域）
  2.上传带 游戏ID 的游戏内截图（上传到群内新人报道相册）
  3.完成游戏自带 Training （游戏主界面右上角 OPS -> Training 下的项目）
  4.阅读 ingress 新手指南: http://pan.baidu.com/s/1pJ4yUuB
  5.汉子请爆照, 妹子请爆两张！
如果对这个游戏有任何疑问，ASK。"#, qqNum.to_string())[..]));

	}

}

// https://github.com/rust-lang/rust/issues/17806

#[export_name="\x01_AppInfo"]
pub extern "stdcall" fn app_info() -> *const i8 {
	
	return gbk!("9,com.github.res.pupurium_r");

}

#[export_name="\x01_Initialize"]
pub extern "stdcall" fn initialize(AuthCode: i32) -> i32 {
	
	//println!("Initialize.");

	unsafe {

		//user32::MessageBoxA(std::ptr::null_mut(), gbk!("PupuriumR初始化完毕。"), gbk!("PupuriumR初始化完毕。"), 0);

		Pupurium.AuthCode = AuthCode;

		Pupurium.IsInitialized = true;

	}

	return 0;

}

#[export_name="\x01_PrivateMessageHandler"]
pub extern "stdcall" fn private_message_handler(subType: i32, sendTime: i32, qqNum: i64, msg: *const i8, font: i32) -> i32 {
	
	unsafe {

		let msg = utf8!(msg);

		//cqpapi::CQ_addLog(Pupurium.AuthCode, cqpapi::CQLOG_INFO, gbk!(msg), gbk!(msg));

		match msg {

			"Alive?" => {

				cqpapi::CQ_sendPrivateMsg(Pupurium.AuthCode, qqNum, gbk!("Alive."));

			},
			_ if(msg.starts_with("WelcomeResistance")) => {

				let arguments: Vec<&str> = msg.split_whitespace().collect();

				if(arguments.len() < 3) {

					cqpapi::CQ_sendPrivateMsg(Pupurium.AuthCode, qqNum, gbk!("INVALID_ARGUMENT"));

					return cqpapi::EVENT_IGNORE;

				}

				match arguments[1] {

					"Wuhan" => {

						match arguments[2].parse::<i64>() {

							Ok(v) => welcomeResistance(147798016, v),
							Err(e) => {

								cqpapi::CQ_sendPrivateMsg(Pupurium.AuthCode, qqNum, gbk!("INVALID_ARGUMENT"));

								return cqpapi::EVENT_IGNORE

							}

						}

					},
					_ => {

						cqpapi::CQ_sendPrivateMsg(Pupurium.AuthCode, qqNum, gbk!("INVALID_ARGUMENT"));

						return cqpapi::EVENT_IGNORE;

					}

				}

			},
			_ => {

				cqpapi::CQ_sendPrivateMsg(Pupurium.AuthCode, qqNum, gbk!(&format!("你刚才说：{}", msg)[..]));

				return cqpapi::EVENT_IGNORE;

			}

		}

	}

	return cqpapi::EVENT_IGNORE;

}

#[export_name="\x01_GroupMessageHandler"]
pub extern "stdcall" fn group_message_handler(subType: i32, sendTime: i32, groupNum: i64, qqNum: i64, anonymousName: *const i8, msg: *const i8, font: i32) -> i32 {
	
	return cqpapi::EVENT_IGNORE;

}

#[export_name="\x01_GroupMemberLeaveHandler"]
pub extern "stdcall" fn group_member_leave_handler(subType: i32, sendTime: i32, groupNum: i64, opQQNum: i64, qqNum: i64) -> i32 {

	unsafe {

		if(groupNum == 147798016) {

			cqpapi::CQ_sendGroupMsg(Pupurium.AuthCode, groupNum, gbk!(&format!("又有人因为忍受不了这群的污退群了……快来个人去关爱一下他，Q号是{}（", qqNum.to_string())[..]));

		}

	}

	return cqpapi::EVENT_IGNORE;

}

#[export_name="\x01_GroupMemberJoinHandler"]
pub extern "stdcall" fn group_member_join_handler(subType: i32, sendTime: i32, groupNum: i64, opQQNum: i64, qqNum: i64) -> i32 {
	
	unsafe {

		if(groupNum == 147798016) {

			welcomeResistance(groupNum, qqNum);

		}

	}

	return cqpapi::EVENT_IGNORE;

}
