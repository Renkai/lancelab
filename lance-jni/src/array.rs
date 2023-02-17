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

pub fn export_array_example() {
    // Export it
    let array = array_example();
    let data = array.data();
    let out_array = FFI_ArrowArray::new(&data);
    let out_schema = FFI_ArrowSchema::try_from(data.data_type()).unwrap();
    // TODO fill them to JNI
    //https://docs.rs/arrow/33.0.0/arrow/ffi/index.html
    //https://arrow.apache.org/docs/java/cdata.html#java-to-c
}