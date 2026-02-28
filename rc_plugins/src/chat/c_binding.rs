//! The foreign function interface implementation for writing ChatPlugins in different shared objects/libraries.
//! This is sort of cursed, I'm sorry in advance.
use std::ffi::{CString, c_char, CStr};

const SET_SEND_MESSAGE_CALLBACK_SYMBOL_NAME: &[u8] = b"oj_rc_set_chat_send_message_callback";
const SET_SEND_MESSAGE_CALLBACK_SYMBOL_NAME_STR: &str = "oj_rc_set_chat_send_message_callback";

const ON_MESSAGE_SYMBOL_NAME: &[u8] = b"oj_rc_on_chat_message";
const ON_MESSAGE_SYMBOL_NAME_STR: &str = "oj_rc_on_chat_message";

pub struct ChatCPlugin {
    dll: libloading::Library,
    pretty_name: String,
    provider: std::sync::Mutex<Option<std::sync::Arc<Box<dyn super::ChatProvider>>>>,
}

impl ChatCPlugin {
    pub fn new(file: impl AsRef<std::path::Path>) -> Result<Self, libloading::Error> {
        let dll = unsafe { libloading::Library::new(file.as_ref()) }?;
        Ok(Self {
            dll,
            pretty_name: file.as_ref().display().to_string(),
            provider: std::sync::Mutex::new(None),
        })
    }
}

struct ContextC {
    provider: std::sync::Weak<Box<dyn super::ChatProvider>>,
}

impl super::ChatPlugin for ChatCPlugin {
    fn set_provider(&self, provider: std::sync::Arc<Box<dyn super::ChatProvider>>) {
        let func: libloading::Symbol<unsafe extern "C" fn(*const ContextC, unsafe extern "C" fn(*const ContextC, *const c_char, *const c_char, *const c_char))> = match unsafe { self.dll.get(SET_SEND_MESSAGE_CALLBACK_SYMBOL_NAME) } {
            Ok(x) => x,
            Err(e) => {
                log::error!("Failed to find symbol {} in library {}: {}", SET_SEND_MESSAGE_CALLBACK_SYMBOL_NAME_STR, self.pretty_name, e);
                return;
            }
        };
        *self.provider.lock().unwrap() = Some(provider.clone());
        let weak_provider = std::sync::Arc::downgrade(&provider);
        let ctx = Box::new(ContextC {
            provider: weak_provider,
        });
        extern "C" fn callback(ctx: *const ContextC, msg: *const c_char, chann: *const c_char, usern: *const c_char) {
            let ctx = if let Some(ctx) = unsafe { ctx.as_ref() } { ctx } else { return };
            let msg = unsafe { CStr::from_ptr(msg) };
            let chann = unsafe { CStr::from_ptr(chann) };
            let usern = unsafe { CStr::from_ptr(usern) };
            if let Some(provider) = ctx.provider.upgrade() {
                match (msg.to_str(), chann.to_str(), usern.to_str()) {
                    (Ok(message), Ok(channel), Ok(username)) => {
                        provider.send_message(message, channel, username);
                    },
                    _ => {
                        log::warn!("{} called with invalid string parameter", SET_SEND_MESSAGE_CALLBACK_SYMBOL_NAME_STR);
                    }
                }
            } else {
                log::warn!("Chat provider callback invoked after it has been dropped");
            }
        }
        unsafe {
            func(&*ctx, callback)
        }
    }

    fn on_message(&self, message: &str, channel: &str, username: &str) {
        let func: libloading::Symbol<unsafe extern "C" fn(*const c_char, *const c_char, *const c_char)> = match unsafe { self.dll.get(ON_MESSAGE_SYMBOL_NAME) } {
            Ok(x) => x,
            Err(e) => {
                log::error!("Failed to find symbol {} in library {}: {}", ON_MESSAGE_SYMBOL_NAME_STR, self.pretty_name, e);
                return;
            }
        };
        let message = CString::new(message).unwrap_or_default();
        let channel = CString::new(channel).unwrap_or_default();
        let username = CString::new(username).unwrap_or_default();
        unsafe {
            func(message.as_c_str().as_ptr(), channel.as_c_str().as_ptr(), username.as_c_str().as_ptr());
        }
    }
}

impl crate::Plugin for ChatCPlugin {}
