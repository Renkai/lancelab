use std::mem::ManuallyDrop;
use arrow::array::{Array, ArrayData, export_array_into_raw, Int32Array, make_array};
use arrow::ffi;
use arrow::ffi::{ArrowArray, FFI_ArrowSchema};
use arrow::ffi::FFI_ArrowArray;

pub fn array_example() -> Int32Array
{
    let array = Int32Array::from(vec![Some(1), None, Some(3)]);
    assert_eq!(array.len(), 3);
    assert_eq!(array.value(0), 1);
    assert_eq!(array.is_null(1), true);

    let collected: Vec<_> = array.iter().collect();
    assert_eq!(collected, vec![Some(1), None, Some(3)]);
    assert_eq!(array.values(), [1, 0, 3]);
    array
}

pub fn export_example2(arr_addr: i64, schema_addr: i64) {
    let array = make_array(array_example().into_data());
    let arr_addr = arr_addr as *mut ffi::FFI_ArrowArray;
    let schema_addr = schema_addr as *mut ffi::FFI_ArrowSchema;
    unsafe { export_array_into_raw(array, arr_addr, schema_addr).unwrap() }
}

pub fn export_array_example() -> [i64; 2] {
    //It doesn't work, keep for study
    // Export it
    let array = array_example();
    let data = array.data();
    let out_array = FFI_ArrowArray::new(&data);
    let out_schema = FFI_ArrowSchema::try_from(data.data_type()).unwrap();

    // Use ManuallyDrop to avoid Box:Drop recursing
    let schema = Box::new(ManuallyDrop::new(out_schema));
    let array = Box::new(ManuallyDrop::new(out_array));

    let schema_ptr = &**schema as *const _;
    let array_ptr = &**array as *const _;
    let schema_addr = schema_ptr as i64;
    let array_addr = array_ptr as i64;

    let schema_addr2 = schema_addr as *const FFI_ArrowSchema;
    let array_addr2 = array_addr as *const FFI_ArrowArray;
    let array = unsafe {
        ArrowArray::new(std::ptr::read(array_addr2), std::ptr::read(schema_addr2))
    };

    let array = Int32Array::from(ArrayData::try_from(array).unwrap());
    println!("pointer schema as long: {}", schema_addr);
    println!("pointer array as long: {}", array_addr);
    println!("recovered arr: {:?}", array);

    [schema_addr, array_addr]
    //https://docs.rs/arrow/33.0.0/arrow/ffi/index.html
    //https://arrow.apache.org/docs/java/cdata.html#java-to-c
    //https://github.com/apache/arrow-rs/blob/3761ac53cab55c269b06d9a13825dd81b03e0c11/arrow/src/ffi.rs#L579-L580
}