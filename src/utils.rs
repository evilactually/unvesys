
use polars::datatypes::AnyValue;

pub fn anyvalue_to_str(anyvalue: &AnyValue) -> std::string::String {
    match anyvalue {
        AnyValue::String(s) => s.to_string(),
        AnyValue::Float32(f) => f.to_string(),
        AnyValue::Float64(f) => f.to_string(),
        AnyValue::Int8(i) => i.to_string(),
        AnyValue::Int16(i) => i.to_string(),
        AnyValue::Int32(i) => i.to_string(),
        AnyValue::Int64(i) => i.to_string(),
        AnyValue::UInt8(i) => i.to_string(),
        AnyValue::UInt16(i) => i.to_string(),
        AnyValue::UInt32(i) => i.to_string(),
        AnyValue::Null => "N/A".to_string(),
        _ => anyvalue.to_string()
    }
}