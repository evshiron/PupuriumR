
/*
* CoolQ SDK for VC++ 
* Api Version 9.6
* Written by Coxxs & Thanks for the help of orzFly

#pragma once

#define CQAPIVER 9
#define CQAPIVERTEXT "9"

#ifndef CQAPI
#define CQAPI(ReturnType) extern "C" __declspec(dllimport) ReturnType __stdcall
#endif

#define CQEVENT(ReturnType, Name, Size) __pragma(comment(linker, "/EXPORT:" #Name "=_" #Name "@" #Size))\
 extern "C" __declspec(dllexport) ReturnType __stdcall Name

typedef int32_t CQBOOL;

#define EVENT_IGNORE 0
#define EVENT_BLOCK 1

#define REQUEST_ALLOW 1
#define REQUEST_DENY 0

#define REQUEST_GROUPADD 1
#define REQUEST_GROUPINVITE 2

CQAPI(int32_t) CQ_sendFriendMsg(int32_t AuthCode, int64_t QQID, const char *msg);
CQAPI(int32_t) CQ_sendStrangerGroupMsg(int32_t AuthCode, int64_t groupNumber, int64_t QQID, const char *msg);
CQAPI(int32_t) CQ_sendStrangerOnlineMsg(int32_t AuthCode, int64_t QQID, const char *msg);
CQAPI(int32_t) CQ_sendPrivateMsg(int32_t AuthCode, int64_t QQID, const char *msg);
CQAPI(int32_t) CQ_sendGroupMsg(int32_t AuthCode, int64_t groupNumber, const char *msg);
CQAPI(int32_t) CQ_sendDiscussMsg(int32_t AuthCode, int64_t discussionNumber, const char *msg);
CQAPI(int32_t) CQ_sendLike(int32_t AuthCode, int64_t QQID);
CQAPI(int32_t) CQ_setGroupKick(int32_t AuthCode, int64_t groupNumber, int64_t QQID, CQBOOL 拒绝再加群);
CQAPI(int32_t) CQ_setGroupBan(int32_t AuthCode, int64_t groupNumber, int64_t QQID, int64_t 禁言时间);
CQAPI(int32_t) CQ_setGroupAdmin(int32_t AuthCode, int64_t groupNumber, int64_t QQID, CQBOOL 成为管理员);
CQAPI(int32_t) CQ_setGroupWholeBan(int32_t AuthCode, int64_t groupNumber, CQBOOL 开启禁言);
CQAPI(int32_t) CQ_setGroupAnonymousBan(int32_t AuthCode, int64_t groupNumber, const char *匿名, int64_t 禁言时间);
CQAPI(int32_t) CQ_setGroupAnonymous(int32_t AuthCode, int64_t groupNumber, CQBOOL 开启匿名);
CQAPI(int32_t) CQ_setGroupCard(int32_t AuthCode, int64_t groupNumber, int64_t QQID, const char *新名片_昵称);
CQAPI(int32_t) CQ_setGroupLeave(int32_t AuthCode, int64_t groupNumber, CQBOOL 是否解散);
CQAPI(int32_t) CQ_setGroupSpecialTitle(int32_t AuthCode, int64_t groupNumber, int64_t QQID, const char *头衔, int64_t 过期时间);
CQAPI(int32_t) CQ_setDiscussLeave(int32_t AuthCode, int64_t discussionNumber);
CQAPI(int32_t) CQ_setFriendAddRequest(int32_t AuthCode, const char *请求反馈标识, int32_t 反馈类型, const char *备注);
CQAPI(int32_t) CQ_setGroupAddRequestV2(int32_t AuthCode, const char *请求反馈标识, int32_t 请求类型, int32_t 反馈类型, const char *理由);
CQAPI(const char *) CQ_getGroupMemberInfoV2(int32_t AuthCode, int64_t groupNumber, int64_t QQID, CQBOOL 不使用缓存);
CQAPI(const char *) CQ_getStrangerInfo(int32_t AuthCode, int64_t QQID, CQBOOL 不使用缓存);
CQAPI(int32_t) CQ_addLog(int32_t AuthCode, int32_t 优先级, const char *类型, const char *内容);
CQAPI(const char *) CQ_getCookies(int32_t AuthCode);
CQAPI(int32_t) CQ_getCsrfToken(int32_t AuthCode);
CQAPI(int64_t) CQ_getLoginQQ(int32_t AuthCode);
CQAPI(const char *) CQ_getLoginNick(int32_t AuthCode);
CQAPI(const char *) CQ_getAppDirectory(int32_t AuthCode);
CQAPI(int32_t) CQ_setFunctionMark(int32_t AuthCode, const char *funcname);
CQAPI(int32_t) CQ_setFatal(int32_t AuthCode, const char *错误信息);

*/

#[link(name = "CQP")]
#[allow(non_snake_case)]
extern {

    fn CQ_sendFriendMsg(AuthCode: i32, QQID: i64, msg: *const u8) -> i32;
	fn CQ_sendStrangerGroupMsg(AuthCode: i32, groupNumber: i64, QQID: i64, msg: *const u8) -> i32;
	fn CQ_sendStrangerOnlineMsg(AuthCode: i32, QQID: i64, msg: *const u8) -> i32;
	fn CQ_sendPrivateMsg(AuthCode: i32, QQID: i64, msg: *const u8) -> i32;
	fn CQ_sendGroupMsg(AuthCode: i32, groupNumber: i64, msg: *const u8) -> i32;
	fn CQ_sendDiscussionMsg(AuthCode: i32, discussionNumber: i64, msg: *const u8) -> i32;
	fn CQ_sendLike(AuthCode: i32, QQID: i64) -> i32;
	fn CQ_setGroupKick(AuthCode: i32, groupNumber: i64, QQID: i64, refuseJoining: i32) -> i32;
	fn CQ_setGroupBan(AuthCode: i32, groupNumber: i64, QQID: i64, banTime: i64) -> i32;
	fn CQ_setGroupAdmin(AuthCode: i32, groupNumber: i64, QQID: i64, becomeAdmin: i32) -> i32;
	fn CQ_setGroupWholeBan(AuthCode: i32, groupNumber: i64, enableBan: i32) -> i32;
	fn CQ_setGroupAnonymousBan(AuthCode: i32, groupNumber: i64, anonymousName: *const u8, banTime: i64) -> i32;
	fn CQ_setGroupAnonymous(AuthCode: i32, groupNumber: i64, enableAnonymous: i32) -> i32;
	fn CQ_setGroupCard(AuthCode: i32, groupNumber: i64, QQID: i64, nickname: *const u8) -> i32;
	fn CQ_setGroupLeave(AuthCode: i32, groupNumber: i64, QQID: i64, disposeGroup: i32) -> i32;
	fn CQ_setGroupSpecialTitle(AuthCode: i32, groupNumber: i64, QQID: i64, title: *const u8, expireTime: i64) -> i32;
	fn CQ_setDiscussLeave(AuthCode: i32, discussionNumber: i64) -> i32;
	/*
CQAPI(int32_t) CQ_setFriendAddRequest(int32_t AuthCode, const char *请求反馈标识, int32_t 反馈类型, const char *备注);
CQAPI(int32_t) CQ_setGroupAddRequestV2(int32_t AuthCode, const char *请求反馈标识, int32_t 请求类型, int32_t 反馈类型, const char *理由);
	*/
	fn CQ_getGroupMemberInfoV2(AuthCode: i32, groupNumber: i64, QQID: i64, useCache: i32) -> *const u8;
	fn CQ_getStrangerInfo(AuthCode: i32, QQID: i64, useCache: i32) -> *const u8;
	fn CQ_addLog(AuthCode: i32, priority: i32, t: *const u8, msg: *const u8) -> i32;
	fn CQ_getCookies(AuthCode: i32) -> *const u8;
	fn CQ_getCsrfToken(AuthCode: i32) -> i32;
	fn CQ_getLoginQQ(AuthCode: i32) -> i64;
	fn CQ_getLoginNick(AuthCode: i32) -> *const u8;
	fn CQ_getAppDirectory(AuthCode: i32) -> *const u8;
	fn CQ_setFunctionMark(AuthCode: i32, functionName: *const u8) -> i32;
	fn CQ_setFatal(AuthCode: i32, errMsg: *const u8) -> i32;

}

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

		CQ_sendFriendMsg(AUTH_CODE, QQID, "Reply!".as_ptr());

	}

	return 0;

}
