//
// Copyright (C) 2019 Signal Messenger, LLC.
// All rights reserved.
//
// SPDX-License-Identifier: GPL-3.0-only
//

//! WebRTC FFI Create / Set Session Description Interface.

use crate::core::util::RustObject;
use libc::size_t;
use std::ffi::c_void;
use std::os::raw::c_char;

use crate::webrtc::sdp_observer::RffiConnectionParametersV4;

/// Incomplete type for SessionDescriptionInterface, used by
/// CreateSessionDescriptionObserver callbacks.
#[repr(C)]
pub struct RffiSessionDescriptionInterface {
    _private: [u8; 0],
}

/// Incomplete type for C++ webrtc::rffi::CreateSessionDescriptionObserverRffi
#[repr(C)]
pub struct RffiCreateSessionDescriptionObserver {
    _private: [u8; 0],
}

/// Incomplete type for C++ CreateSessionDescriptionObserverRffi
#[repr(C)]
pub struct RffiSetSessionDescriptionObserver {
    _private: [u8; 0],
}

extern "C" {
    pub fn Rust_createSetSessionDescriptionObserver(
        ssd_observer: RustObject,
        ssd_observer_cb: *const c_void,
    ) -> *const RffiSetSessionDescriptionObserver;

    pub fn Rust_createCreateSessionDescriptionObserver(
        csd_observer: RustObject,
        csd_observer_cb: *const c_void,
    ) -> *const RffiCreateSessionDescriptionObserver;

    pub fn Rust_toSdp(offer: *const RffiSessionDescriptionInterface) -> *const c_char;

    pub fn Rust_answerFromSdp(sdp: *const c_char) -> *const RffiSessionDescriptionInterface;

    pub fn Rust_offerFromSdp(sdp: *const c_char) -> *const RffiSessionDescriptionInterface;

    pub fn Rust_replaceRtpDataChannelsWithSctp(sdi: *const RffiSessionDescriptionInterface)
        -> bool;

    pub fn Rust_disableDtlsAndSetSrtpKey(
        sdi: *const RffiSessionDescriptionInterface,
        crypto_suite: crate::webrtc::sdp_observer::SrtpCryptoSuite,
        key_ptr: *const u8,
        key_len: size_t,
        salt_ptr: *const u8,
        salt_len: size_t,
    ) -> bool;

    pub fn Rust_sessionDescriptionToV4(
        sdi: *const RffiSessionDescriptionInterface,
    ) -> *mut RffiConnectionParametersV4;

    pub fn Rust_releaseV4(sdi: *mut RffiConnectionParametersV4);

    pub fn Rust_sessionDescriptionFromV4(
        offer: bool,
        v4: *const RffiConnectionParametersV4,
    ) -> *const RffiSessionDescriptionInterface;
}
