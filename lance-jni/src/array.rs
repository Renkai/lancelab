use std::mem::ManuallyDrop;
use arrow::array::{Array, Int32Array};
use arrow::ffi::FFI_ArrowSchema;
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

pub fn export_array_example() -> [i64; 2] {
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
    [schema_addr, array_addr]
    //https://docs.rs/arrow/33.0.0/arrow/ffi/index.html
    //https://arrow.apache.org/docs/java/cdata.html#java-to-c
    //https://github.com/apache/arrow-rs/blob/3761ac53cab55c269b06d9a13825dd81b03e0c11/arrow/src/ffi.rs#L579-L580
}