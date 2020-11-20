pub enum FloatComparision{
    GreaterThan,
    LessThan,
    Equal
}

pub fn compare(a: f64, b: f64) -> FloatComparision{
    let difference = a - b;

    if difference.abs() < 0.001 {
        FloatComparision::Equal
    }   
    else if difference > 0.0 {
        FloatComparision::GreaterThan
    } else {
        FloatComparision::LessThan
    }
}