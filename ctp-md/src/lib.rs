extern crate ctp_common;

use std::ffi::{ CStr, CString };
use std::mem::transmute;
use std::os::raw::{ c_void, c_char, c_int };
use std::sync::mpsc;

#[allow(non_camel_case_types)]
type c_bool = std::os::raw::c_uchar;

pub use ctp_common::*;

#[link(name = "thostmduserapi")]
extern "C" {
    fn _ZN15CThostFtdcMdApi15CreateFtdcMdApiEPKcbb(pszFlowPath: *const c_char, bIsUsingUdp: c_bool, bIsMulticast: c_bool) -> *mut c_void;
    fn _ZN14CFtdcMdApiImpl7ReleaseEv(api: *mut c_void);
    fn _ZN14CFtdcMdApiImpl4InitEv(api: *mut c_void);
    fn _ZN14CFtdcMdApiImpl4JoinEv(api: *mut c_void) -> c_int;
    fn _ZN14CFtdcMdApiImpl13GetTradingDayEv(api: *mut c_void) -> *const c_char;
    fn _ZN14CFtdcMdApiImpl13RegisterFrontEPc(api: *mut c_void, pszFrontAddress: *const c_char);
    fn _ZN14CFtdcMdApiImpl18RegisterNameServerEPc(api: *mut c_void, pszNsAddress: *const c_char);
    fn _ZN14CFtdcMdApiImpl20RegisterFensUserInfoEP27CThostFtdcFensUserInfoField(api: *mut c_void, pFensUserInfo: *const CThostFtdcFensUserInfoField);
    fn _ZN14CFtdcMdApiImpl11RegisterSpiEP15CThostFtdcMdSpi(api: *mut c_void, pSpi: *mut c_void);
    fn _ZN14CFtdcMdApiImpl19SubscribeMarketDataEPPci(api: *mut c_void, ppInstrumentID: *const *const c_char, nCount: c_int) -> c_int;
    fn _ZN14CFtdcMdApiImpl21UnSubscribeMarketDataEPPci(api: *mut c_void, ppInstrumentID: *const *const c_char, nCount: c_int) -> c_int;
    fn _ZN14CFtdcMdApiImpl20SubscribeForQuoteRspEPPci(api: *mut c_void, ppInstrumentID: *const *const c_char, nCount: c_int) -> c_int;
    fn _ZN14CFtdcMdApiImpl22UnSubscribeForQuoteRspEPPci(api: *mut c_void, ppInstrumentID: *const *const c_char, nCount: c_int) -> c_int;
    fn _ZN14CFtdcMdApiImpl12ReqUserLoginEP27CThostFtdcReqUserLoginFieldi(api: *mut c_void, pReqUserLoginField: *const CThostFtdcReqUserLoginField, nRequestID: c_int) -> c_int;
    fn _ZN14CFtdcMdApiImpl13ReqUserLogoutEP25CThostFtdcUserLogoutFieldi(api: *mut c_void, pUserLogoutField: *const CThostFtdcUserLogoutField, nRequestID: c_int) -> c_int;
}

pub trait GenericMdApi {
    fn new(flow_path: CString, use_udp: bool, use_multicast: bool) -> Self;
    fn init(&mut self);
    fn join(&mut self) -> ApiResult;
    fn get_trading_day<'a>(&mut self) -> &'a CStr;
    fn register_front(&mut self, front_socket_address: CString);
    fn register_name_server(&mut self, name_server: CString);
    fn register_fens_user_info(&mut self, fens_user_info: &CThostFtdcFensUserInfoField);
    fn register_spi(&mut self, md_spi: Box<MdSpi>);
    fn subscribe_market_data(&mut self, instrument_ids: &[CString]) -> ApiResult;
    fn unsubscribe_market_data(&mut self, instrument_ids: &[CString]) -> ApiResult;
    fn subscribe_for_quote_rsp(&mut self, instrument_ids: &[CString]) -> ApiResult;
    fn unsubscribe_for_quote_rsp(&mut self, instrument_ids: &[CString]) -> ApiResult;
    fn req_user_login(&mut self, req_user_login: &CThostFtdcReqUserLoginField, request_id: TThostFtdcRequestIDType) -> ApiResult;
    fn req_user_logout(&mut self, req_user_logout: &CThostFtdcUserLogoutField, request_id: TThostFtdcRequestIDType) -> ApiResult;
}

pub struct MdApi {
    md_api_ptr: *mut c_void,
    registered_spi: Option<*mut CThostFtdcMdSpi>,
}

unsafe impl Send for MdApi {}

impl GenericMdApi for MdApi {
    fn new(flow_path: CString, use_udp: bool, use_multicast: bool) -> Self {
        let flow_path_ptr = flow_path.into_raw();
        let api = unsafe { _ZN15CThostFtdcMdApi15CreateFtdcMdApiEPKcbb(flow_path_ptr, use_udp as c_bool, use_multicast as c_bool) };
        let flow_path = unsafe { CString::from_raw(flow_path_ptr) };
        drop(flow_path);
        MdApi{ md_api_ptr: api, registered_spi: None }
    }

    fn init(&mut self) {
        unsafe { _ZN14CFtdcMdApiImpl4InitEv(self.md_api_ptr) };
    }

    fn join(&mut self) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN14CFtdcMdApiImpl4JoinEv(self.md_api_ptr) })
    }

    fn get_trading_day<'a>(&mut self) -> &'a CStr {
        let trading_day_cstr = unsafe { _ZN14CFtdcMdApiImpl13GetTradingDayEv(self.md_api_ptr) };
        unsafe { CStr::from_ptr(trading_day_cstr) }
    }

    fn register_front(&mut self, front_socket_address: CString) {
        let front_socket_address_ptr = front_socket_address.into_raw();
        unsafe { _ZN14CFtdcMdApiImpl13RegisterFrontEPc(self.md_api_ptr, front_socket_address_ptr) };
        let front_socket_address = unsafe { CString::from_raw(front_socket_address_ptr) };
        drop(front_socket_address);
    }

    fn register_name_server(&mut self, name_server: CString) {
        let name_server_ptr = name_server.into_raw();
        unsafe { _ZN14CFtdcMdApiImpl18RegisterNameServerEPc(self.md_api_ptr, name_server_ptr) };
        let name_server = unsafe { CString::from_raw(name_server_ptr) };
        drop(name_server);
    }

    fn register_fens_user_info(&mut self, fens_user_info: &CThostFtdcFensUserInfoField) {
        unsafe { _ZN14CFtdcMdApiImpl20RegisterFensUserInfoEP27CThostFtdcFensUserInfoField(self.md_api_ptr, fens_user_info) };
    }

    fn register_spi(&mut self, md_spi: Box<MdSpi>) {
        let last_registered_spi_ptr = self.registered_spi.take();
        let md_spi_ptr = Box::into_raw(md_spi);
        let spi_ptr = Box::into_raw(Box::new(new_spi(md_spi_ptr)));
        unsafe { _ZN14CFtdcMdApiImpl11RegisterSpiEP15CThostFtdcMdSpi(self.md_api_ptr, spi_ptr as *mut c_void) };
        self.registered_spi = Some(spi_ptr);
        if let Some(last_registered_spi_ptr) = last_registered_spi_ptr {
            unsafe {
                let last_registered_spi = Box::from_raw(last_registered_spi_ptr);
                drop(last_registered_spi.md_spi_ptr);
                drop(last_registered_spi);
            }
        };
    }

    fn subscribe_market_data(&mut self, instrument_ids: &[CString]) -> ApiResult {
        let v = cstring_slice_to_char_star_vec(instrument_ids);
        from_api_return_to_api_result(unsafe { _ZN14CFtdcMdApiImpl19SubscribeMarketDataEPPci(self.md_api_ptr, v.as_ptr(), v.len() as c_int) })
    }

    fn unsubscribe_market_data(&mut self, instrument_ids: &[CString]) -> ApiResult {
        let v = cstring_slice_to_char_star_vec(instrument_ids);
        from_api_return_to_api_result(unsafe { _ZN14CFtdcMdApiImpl21UnSubscribeMarketDataEPPci(self.md_api_ptr, v.as_ptr(), v.len() as c_int) })
    }

    fn subscribe_for_quote_rsp(&mut self, instrument_ids: &[CString]) -> ApiResult {
        let v = cstring_slice_to_char_star_vec(instrument_ids);
        from_api_return_to_api_result(unsafe { _ZN14CFtdcMdApiImpl20SubscribeForQuoteRspEPPci(self.md_api_ptr, v.as_ptr(), v.len() as c_int) })
    }

    fn unsubscribe_for_quote_rsp(&mut self, instrument_ids: &[CString]) -> ApiResult {
        let v = cstring_slice_to_char_star_vec(instrument_ids);
        from_api_return_to_api_result(unsafe { _ZN14CFtdcMdApiImpl22UnSubscribeForQuoteRspEPPci(self.md_api_ptr, v.as_ptr(), v.len() as c_int) })
    }

    fn req_user_login(&mut self, req_user_login: &CThostFtdcReqUserLoginField, request_id: TThostFtdcRequestIDType) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN14CFtdcMdApiImpl12ReqUserLoginEP27CThostFtdcReqUserLoginFieldi(self.md_api_ptr, req_user_login, request_id) })
    }

    fn req_user_logout(&mut self, req_user_logout: &CThostFtdcUserLogoutField, request_id: TThostFtdcRequestIDType) -> ApiResult {
        from_api_return_to_api_result(unsafe { _ZN14CFtdcMdApiImpl13ReqUserLogoutEP25CThostFtdcUserLogoutFieldi(self.md_api_ptr, req_user_logout, request_id) })
    }
}

impl Drop for MdApi {
    fn drop(&mut self) {
        let last_registered_spi_ptr = self.registered_spi.take();
        if let Some(last_registered_spi_ptr) =  last_registered_spi_ptr {
            unsafe {
                _ZN14CFtdcMdApiImpl11RegisterSpiEP15CThostFtdcMdSpi(self.md_api_ptr, ::std::ptr::null_mut::<c_void>());
                let last_registered_spi = Box::from_raw(last_registered_spi_ptr);
                drop(last_registered_spi.md_spi_ptr);
                drop(last_registered_spi);
            }
        };
        unsafe { _ZN14CFtdcMdApiImpl7ReleaseEv(self.md_api_ptr) };
    }
}

fn cstring_slice_to_char_star_vec(cstring_vec: &[CString]) -> Vec<*const c_char> {
    cstring_vec.iter().map(|cstring| cstring.as_ptr()).collect()
}

pub trait MdSpi : Send {
    fn on_front_connected(&mut self) {
        println!("on_front_connected");
    }

    fn on_front_disconnected(&mut self, reason: DisconnectionReason) {
        println!("on_front_disconnected: {:?}", reason);
    }

    #[allow(unused_variables)]
    fn on_rsp_user_login(&mut self, rsp_user_login: Option<&CThostFtdcRspUserLoginField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_user_login: {:?}, {}, {:?}, {:?}", rsp_user_login, from_rsp_result_to_string(&result), request_id, is_last);
    }

    #[allow(unused_variables)]
    fn on_rsp_user_logout(&mut self, rsp_user_logout: Option<&CThostFtdcUserLogoutField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_user_logout: {:?}, {}, {:?}, {:?}", rsp_user_logout, from_rsp_result_to_string(&result), request_id, is_last);
    }

    #[allow(unused_variables)]
    fn on_rsp_error(&mut self, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_error: {}, {:?}, {:?}", from_rsp_result_to_string(&result), request_id, is_last);
    }

    #[allow(unused_variables)]
    fn on_rsp_sub_market_data(&mut self, specific_instrument: Option<&CThostFtdcSpecificInstrumentField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_sub_market_data: {:?}, {}, {:?}, {:?}", specific_instrument, from_rsp_result_to_string(&result), request_id, is_last);
    }

    #[allow(unused_variables)]
    fn on_rsp_un_sub_market_data(&mut self, specific_instrument: Option<&CThostFtdcSpecificInstrumentField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_un_sub_market_data: {:?}, {}, {:?}, {:?}", specific_instrument, from_rsp_result_to_string(&result), request_id, is_last);
    }

    #[allow(unused_variables)]
    fn on_rsp_sub_for_quote_rsp(&mut self, specific_instrument: Option<&CThostFtdcSpecificInstrumentField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_sub_for_quote_rsp: {:?}, {}, {:?}, {:?}", specific_instrument, from_rsp_result_to_string(&result), request_id, is_last);
    }

    #[allow(unused_variables)]
    fn on_rsp_un_sub_for_quote_rsp(&mut self, specific_instrument: Option<&CThostFtdcSpecificInstrumentField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        println!("on_rsp_un_sub_for_quote_rsp: {:?}, {}, {:?}, {:?}", specific_instrument, from_rsp_result_to_string(&result), request_id, is_last);
    }

    #[allow(unused_variables)]
    fn on_rtn_depth_market_data(&mut self, depth_market_data: Option<&CThostFtdcDepthMarketDataField>) {
        println!("on_rtn_depth_market_data: {:?}", depth_market_data);
    }

    #[allow(unused_variables)]
    fn on_rtn_for_quote_rsp(&mut self, for_quote_rsp: Option<&CThostFtdcForQuoteRspField>) {
        println!("on_rtn_for_quote_rsp: {:?}", for_quote_rsp);
    }
}

#[derive(Clone, Debug)]
pub struct MdSpiOnFrontConnected {
}

#[derive(Clone, Debug)]
pub struct MdSpiOnFrontDisconnected {
    pub reason: DisconnectionReason,
}

#[derive(Clone, Debug)]
pub struct MdSpiOnRspUserLogin {
    pub user_login: Option<CThostFtdcRspUserLoginField>,
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct MdSpiOnRspUserLogout {
    pub user_logout: Option<CThostFtdcUserLogoutField>,
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct MdSpiOnRspError {
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct MdSpiOnRspSubMarketData {
    pub specific_instrument: Option<CThostFtdcSpecificInstrumentField>,
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct MdSpiOnRspUnSubMarketData {
    pub specific_instrument: Option<CThostFtdcSpecificInstrumentField>,
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct MdSpiOnRspSubForQuoteRsp {
    pub specific_instrument: Option<CThostFtdcSpecificInstrumentField>,
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct MdSpiOnRspUnSubForQuoteRsp {
    pub specific_instrument: Option<CThostFtdcSpecificInstrumentField>,
    pub result: RspResult,
    pub request_id: TThostFtdcRequestIDType,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct MdSpiOnRtnDepthMarketData {
    pub depth_market_data: CThostFtdcDepthMarketDataField,
}

#[derive(Clone, Debug)]
pub struct MdSpiOnRtnForQuoteRsp {
    pub for_quote_rsp: CThostFtdcForQuoteRspField,
}

#[derive(Clone, Debug)]
pub enum MdSpiOutput {
    FrontConnected(MdSpiOnFrontConnected),
    RspUserLogin(MdSpiOnRspUserLogin),
    FrontDisconnected(MdSpiOnFrontDisconnected),
    RspUserLogout(MdSpiOnRspUserLogout),
    RspError(MdSpiOnRspError),
    SubMarketData(MdSpiOnRspSubMarketData),
    UnSubMarketData(MdSpiOnRspUnSubMarketData),
    SubForQuoteRsp(MdSpiOnRspSubForQuoteRsp),
    UnSubForQuoteRsp(MdSpiOnRspUnSubForQuoteRsp),
    DepthMarketData(MdSpiOnRtnDepthMarketData),
    ForQuoteRsp(MdSpiOnRtnForQuoteRsp),
}

#[derive(Clone, Debug)]
pub struct SenderMdSpi<T: From<MdSpiOutput> + Send + 'static> {
    sender: mpsc::Sender<T>,
}

impl<T> SenderMdSpi<T> where T: From<MdSpiOutput> + Send + 'static {
    pub fn new(sender: mpsc::Sender<T>) -> Self {
        SenderMdSpi {
            sender: sender,
        }
    }
}

impl<T> MdSpi for SenderMdSpi<T> where T: From<MdSpiOutput> + Send + 'static {
    fn on_front_connected(&mut self) {
        self.sender.send(T::from(MdSpiOutput::FrontConnected(MdSpiOnFrontConnected{ }))).expect("spi callback send front_connected failed");
    }

    fn on_front_disconnected(&mut self, reason: DisconnectionReason) {
        self.sender.send(T::from(MdSpiOutput::FrontDisconnected(MdSpiOnFrontDisconnected{ reason: reason }))).expect("spi callback send front_disconnected failed");
    }

    fn on_rsp_user_login(&mut self, rsp_user_login: Option<&CThostFtdcRspUserLoginField>, result: RspResult, request_id: i32, is_last: bool) {
        self.sender.send(T::from(MdSpiOutput::RspUserLogin(MdSpiOnRspUserLogin{ user_login: rsp_user_login.cloned(), result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_user_login failed");
    }

    fn on_rsp_user_logout(&mut self, rsp_user_logout: Option<&CThostFtdcUserLogoutField>, result: RspResult, request_id: i32, is_last: bool) {
        self.sender.send(T::from(MdSpiOutput::RspUserLogout(MdSpiOnRspUserLogout{ user_logout: rsp_user_logout.cloned(), result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_user_logout failed");
    }

    fn on_rsp_error(&mut self, result: RspResult, request_id: i32, is_last: bool) {
        self.sender.send(T::from(MdSpiOutput::RspError(MdSpiOnRspError{ result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_error failed");
    }

    fn on_rsp_sub_market_data(&mut self, specific_instrument: Option<&CThostFtdcSpecificInstrumentField>, result: RspResult, request_id: i32, is_last: bool) {
        self.sender.send(T::from(MdSpiOutput::SubMarketData(MdSpiOnRspSubMarketData{ specific_instrument: specific_instrument.cloned(), result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_sub_market_data failed");
    }

    fn on_rsp_un_sub_market_data(&mut self, specific_instrument: Option<&CThostFtdcSpecificInstrumentField>, result: RspResult, request_id: i32, is_last: bool) {
        self.sender.send(T::from(MdSpiOutput::UnSubMarketData(MdSpiOnRspUnSubMarketData{ specific_instrument: specific_instrument.cloned(), result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_sub_market_data failed");
    }

    fn on_rsp_sub_for_quote_rsp(&mut self, specific_instrument: Option<&CThostFtdcSpecificInstrumentField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send(T::from(MdSpiOutput::SubForQuoteRsp(MdSpiOnRspSubForQuoteRsp{ specific_instrument: specific_instrument.cloned(), result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_sub_sub_for_quote_rsp failed");
    }

    fn on_rsp_un_sub_for_quote_rsp(&mut self, specific_instrument: Option<&CThostFtdcSpecificInstrumentField>, result: RspResult, request_id: TThostFtdcRequestIDType, is_last: bool) {
        self.sender.send(T::from(MdSpiOutput::UnSubForQuoteRsp(MdSpiOnRspUnSubForQuoteRsp{ specific_instrument: specific_instrument.cloned(), result: result, request_id: request_id, is_last: is_last }))).expect("spi callback send rsp_sub_sub_for_quote_rsp failed");
    }

    fn on_rtn_depth_market_data(&mut self, depth_market_data: Option<&CThostFtdcDepthMarketDataField>) {
        self.sender.send(T::from(MdSpiOutput::DepthMarketData(MdSpiOnRtnDepthMarketData{ depth_market_data: depth_market_data.expect("depth_market_data is none").clone() }))).expect("spi callback send depth_market_data failed");
    }

    fn on_rtn_for_quote_rsp(&mut self, for_quote_rsp: Option<&CThostFtdcForQuoteRspField>) {
        self.sender.send(T::from(MdSpiOutput::ForQuoteRsp(MdSpiOnRtnForQuoteRsp{ for_quote_rsp: for_quote_rsp.expect("for_quote_rsp is none").clone() }))).expect("spi callback send depth_market_data failed");
    }
}

#[allow(non_snake_case)]
extern "C" fn spi_on_front_connected(spi: *mut CThostFtdcMdSpi) {
    unsafe { transmute::<*mut MdSpi, &mut MdSpi>(transmute::<*mut CThostFtdcMdSpi, &mut CThostFtdcMdSpi>(spi).md_spi_ptr).on_front_connected() };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_front_disconnected(spi: *mut CThostFtdcMdSpi, nReason: c_int) {
    let reason = std::convert::From::from(nReason);
    unsafe { transmute::<*mut MdSpi, &mut MdSpi>(transmute::<*mut CThostFtdcMdSpi, &mut CThostFtdcMdSpi>(spi).md_spi_ptr).on_front_disconnected(reason) };
}

#[allow(non_snake_case, unused_variables)]
extern "C" fn spi_on_heart_beat_warning(spi: *mut CThostFtdcMdSpi, nTimeLapse: c_int) {
    // CTP API specification shows this will never be called
    unreachable!();
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_user_login(spi: *mut CThostFtdcMdSpi, pRspUserLogin: *const CThostFtdcRspUserLoginField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut MdSpi, &mut MdSpi>(transmute::<*mut CThostFtdcMdSpi, &mut CThostFtdcMdSpi>(spi).md_spi_ptr).on_rsp_user_login(pRspUserLogin.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_user_logout(spi: *mut CThostFtdcMdSpi, pUserLogout: *const CThostFtdcUserLogoutField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut MdSpi, &mut MdSpi>(transmute::<*mut CThostFtdcMdSpi, &mut CThostFtdcMdSpi>(spi).md_spi_ptr).on_rsp_user_logout(pUserLogout.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_error(spi: *mut CThostFtdcMdSpi, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut MdSpi, &mut MdSpi>(transmute::<*mut CThostFtdcMdSpi, &mut CThostFtdcMdSpi>(spi).md_spi_ptr).on_rsp_error(rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_sub_market_data(spi: *mut CThostFtdcMdSpi, pSpecificInstrument: *const CThostFtdcSpecificInstrumentField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut MdSpi, &mut MdSpi>(transmute::<*mut CThostFtdcMdSpi, &mut CThostFtdcMdSpi>(spi).md_spi_ptr).on_rsp_sub_market_data(pSpecificInstrument.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_un_sub_market_data(spi: *mut CThostFtdcMdSpi, pSpecificInstrument: *const CThostFtdcSpecificInstrumentField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut MdSpi, &mut MdSpi>(transmute::<*mut CThostFtdcMdSpi, &mut CThostFtdcMdSpi>(spi).md_spi_ptr).on_rsp_un_sub_market_data(pSpecificInstrument.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_sub_for_quote_rsp(spi: *mut CThostFtdcMdSpi, pSpecificInstrument: *const CThostFtdcSpecificInstrumentField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut MdSpi, &mut MdSpi>(transmute::<*mut CThostFtdcMdSpi, &mut CThostFtdcMdSpi>(spi).md_spi_ptr).on_rsp_sub_for_quote_rsp(pSpecificInstrument.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rsp_un_sub_for_quote_rsp(spi: *mut CThostFtdcMdSpi, pSpecificInstrument: *const CThostFtdcSpecificInstrumentField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool) {
    let rsp_info = from_rsp_info_to_rsp_result(pRspInfo);
    unsafe { transmute::<*mut MdSpi, &mut MdSpi>(transmute::<*mut CThostFtdcMdSpi, &mut CThostFtdcMdSpi>(spi).md_spi_ptr).on_rsp_un_sub_for_quote_rsp(pSpecificInstrument.as_ref(), rsp_info, nRequestID, bIsLast != 0) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rtn_depth_market_data(spi: *mut CThostFtdcMdSpi, pDepthMarketData: *const CThostFtdcDepthMarketDataField ) {
    unsafe { transmute::<*mut MdSpi, &mut MdSpi>(transmute::<*mut CThostFtdcMdSpi, &mut CThostFtdcMdSpi>(spi).md_spi_ptr).on_rtn_depth_market_data(pDepthMarketData.as_ref()) };
}

#[allow(non_snake_case)]
extern "C" fn spi_on_rtn_for_quote_rsp(spi: *mut CThostFtdcMdSpi, pForQuoteRsp: *const CThostFtdcForQuoteRspField ) {
    unsafe { transmute::<*mut MdSpi, &mut MdSpi>(transmute::<*mut CThostFtdcMdSpi, &mut CThostFtdcMdSpi>(spi).md_spi_ptr).on_rtn_for_quote_rsp(pForQuoteRsp.as_ref()) };
}

#[repr(C)]
#[derive(Debug)]
struct SpiVTable {
    #[allow(non_snake_case)]
    on_front_connected: extern "C" fn(spi: *mut CThostFtdcMdSpi),
    #[allow(non_snake_case)]
    on_front_disconnected: extern "C" fn(spi: *mut CThostFtdcMdSpi, nReason: c_int),
    #[allow(non_snake_case)]
    on_heart_beat_warning: extern "C" fn(spi: *mut CThostFtdcMdSpi, nTimeLapse: c_int),
    #[allow(non_snake_case)]
    on_rsp_user_login: extern "C" fn(spi: *mut CThostFtdcMdSpi, pRspUserLogin: *const CThostFtdcRspUserLoginField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_user_logout: extern "C" fn(spi: *mut CThostFtdcMdSpi, pUserLogout: *const CThostFtdcUserLogoutField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_error: extern "C" fn(spi: *mut CThostFtdcMdSpi, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_sub_market_data: extern "C" fn(spi: *mut CThostFtdcMdSpi, pSpecificInstrument: *const CThostFtdcSpecificInstrumentField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_un_sub_market_data: extern "C" fn(spi: *mut CThostFtdcMdSpi, pSpecificInstrument: *const CThostFtdcSpecificInstrumentField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_sub_for_quote_rsp: extern "C" fn(spi: *mut CThostFtdcMdSpi, pSpecificInstrument: *const CThostFtdcSpecificInstrumentField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rsp_un_sub_for_quote_rsp: extern "C" fn(spi: *mut CThostFtdcMdSpi, pSpecificInstrument: *const CThostFtdcSpecificInstrumentField, pRspInfo: *const CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: c_bool),
    #[allow(non_snake_case)]
    on_rtn_depth_market_data: extern "C" fn(spi: *mut CThostFtdcMdSpi, pDepthMarketData: *const CThostFtdcDepthMarketDataField ),
    #[allow(non_snake_case)]
    on_rtn_for_quote_rsp: extern "C" fn(spi: *mut CThostFtdcMdSpi, pForQuoteRsp: *const CThostFtdcForQuoteRspField ),
}

static SPI_VTABLE: SpiVTable = SpiVTable{
    on_front_connected: spi_on_front_connected,
    on_front_disconnected: spi_on_front_disconnected,
    on_heart_beat_warning: spi_on_heart_beat_warning,
    on_rsp_user_login: spi_on_rsp_user_login,
    on_rsp_user_logout: spi_on_rsp_user_logout,
    on_rsp_error: spi_on_rsp_error,
    on_rsp_sub_market_data: spi_on_rsp_sub_market_data,
    on_rsp_un_sub_market_data: spi_on_rsp_un_sub_market_data,
    on_rsp_sub_for_quote_rsp: spi_on_rsp_sub_for_quote_rsp,
    on_rsp_un_sub_for_quote_rsp: spi_on_rsp_un_sub_for_quote_rsp,
    on_rtn_depth_market_data: spi_on_rtn_depth_market_data,
    on_rtn_for_quote_rsp: spi_on_rtn_for_quote_rsp,
};

#[repr(C)]
pub struct CThostFtdcMdSpi {
    vtable: *const SpiVTable,
    pub md_spi_ptr: *mut MdSpi
}

fn new_spi(md_spi: *mut MdSpi) -> CThostFtdcMdSpi {
    CThostFtdcMdSpi{ vtable: &SPI_VTABLE, md_spi_ptr: md_spi }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::mem::size_of;

    #[test]
    fn spi_output_size() {
        let expected_size = 416;
        let actual_size = size_of::<MdSpiOutput>();
        assert_eq!(expected_size, actual_size, "MdSpiOutput expected size {}, actual size {}", expected_size, actual_size);
    }

    #[test]
    fn create_release() {
        let flow_path = CString::new("").unwrap();
        let md_api = MdApi::new(flow_path, false, false);
        drop(md_api);
        assert!(true);
    }

    #[test]
    fn get_trading_day() {
        let flow_path = CString::new("").unwrap();
        let mut md_api = MdApi::new(flow_path, false, false);
        let trading_day = md_api.get_trading_day();
        assert_eq!(b"19700101".len(), trading_day.to_bytes().len());
        let year = ::std::str::from_utf8(&trading_day.to_bytes()[0..4]).unwrap().parse::<i32>().unwrap();
        assert!(year > 1970 && year < 2038, "year of trading day {} is not an integer in [1971, 2037]", year);
        let month = ::std::str::from_utf8(&trading_day.to_bytes()[4..6]).unwrap().parse::<i32>().unwrap();
        assert!(month > 0 && month < 13, "month of trading day {} is not an integer in [1, 12]", month);
        let day = ::std::str::from_utf8(&trading_day.to_bytes()[6..8]).unwrap().parse::<i32>().unwrap();
        assert!(day > 0 && day < 32, "day of trading day {} is not an integer in [1, 31]", day);
    }
}
