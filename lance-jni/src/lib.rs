pub mod array;


use jni::JNIEnv;

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass, JLongArray, JString};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::{jlongArray, jobjectArray, jstring};
use crate::array::export_array_example;

#[no_mangle]
pub extern "system" fn Java_cinterface_ConverterJni_getInt32Arr<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>) -> jlongArray {
    let mut longArr = env.new_long_array(2).unwrap();
    let example = export_array_example();
    env.set_long_array_region(longArr, 0, &example).unwrap();
    longArr.as_raw()
}