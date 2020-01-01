// This is the interface to the JVM that we'll
// call the majority of our methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native function.
// They carry extra lifetime information to prevent them escaping this context
// and getting used after being GC'd.
use jni::objects::{GlobalRef, JClass, JObject, JString};

// This is just a pointer. We'll be returning it from our function.
// We can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::{jbyteArray, jint, jlong, jstring, jdouble};

use std::{sync::mpsc, thread, time::Duration};

use hashbrown::HashMap;

#[no_mangle]
pub unsafe extern "system" fn Java_LongToDoubleSwissTable_newLongToDoubleSwissTable(
    env: JNIEnv,
    _class: JClass,
    size: jlong,
) -> jlong {
    let mut instance: HashMap<u64, f64> = HashMap::with_capacity(size as usize);
    let map_ptr = Box::into_raw(Box::new(instance)) as jlong;
    //println!("created instance {}", map_ptr);
    map_ptr
}

#[no_mangle]
pub unsafe extern "system" fn Java_LongToDoubleSwissTable_destroyTable(
    _env: JNIEnv,
    _class: JClass,
    map_ptr: jlong,
) {
    let instance = Box::from_raw(map_ptr as *mut HashMap<u64, f64>);
    //println!("destroying instance {}={:?}", map_ptr, instance);
}

// Needed for JIT
#[no_mangle]
pub unsafe extern "system" fn Java_LongToDoubleSwissTable_putAndAdd(
    env: JNIEnv,
    _class: JClass,
    map_ptr: jlong,
    jkey: jlong,
    jdelta: jdouble
) {
    //println!("Normal");
    put_and_add(map_ptr, jkey, jdelta);
}

// Called directly from Java when compiled, both versions are necessary for the interpretor
#[no_mangle]
pub unsafe extern "system" fn JavaCritical_LongToDoubleSwissTable_putAndAdd(
    map_ptr: jlong,
    jkey: jlong,
    jdelta: jdouble
) {
    //println!("Critical");
    put_and_add(map_ptr, jkey, jdelta);
}

unsafe fn put_and_add(
    map_ptr: jlong,
    jkey: jlong,
    jdelta: jdouble
) {
    let instance = &mut *(map_ptr as *mut HashMap<u64, f64>);
    let key = jkey as u64;
    let delta = jdelta as f64;
    //println!("before instance {}={:?}", map_ptr, instance);
    let value = instance.entry(key).or_insert(0.0);
    *value += delta;
    println!("after instance {}={:?}", map_ptr, instance);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
