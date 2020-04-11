use fluent_bundle::{FluentArgs, FluentValue};
use jni::JNIEnv;
use jni::objects::{JObject, JString};
use crate::{surrender_rust_pointer, get_rust_pointer, javastr_to_ruststr};

#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_FluentArgs_bind(
    env: JNIEnv,
    this: JObject
) {
    let args = FluentArgs::new();
    surrender_rust_pointer(&env, &this, args);
}

#[no_mangle]
pub extern "system" fn Java_io_github_javidaloca_FluentArgs_insert(
    env: JNIEnv,
    this: JObject,
    parameter: JString,
    value: JObject
) {
    let mut args = get_rust_pointer::<FluentArgs>(&env, &this);
    let parameter = javastr_to_ruststr(&env, parameter);
    let value = get_rust_pointer::<FluentValue>(&env, &value).clone();
    // FIXME fluent issue? insert String instead of &str
    args.insert(&parameter, value);
}