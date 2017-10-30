extern crate fraction;
extern crate num;
#[macro_use]
extern crate over;

use fraction::BigFraction;
use num::ToPrimitive;
use over::OverError;
use over::obj::Obj;
use over::value::Value;

fn get_int(obj: &Obj, field: &str) -> i64 {
    obj.get(field).unwrap().get_int().unwrap().to_i64().unwrap()
}

// Test parsing of empty file.
#[test]
fn empty() {
    let obj = Obj::from_file("tests/test_files/empty.over").unwrap();

    assert_eq!(obj.len(), 0);
}

// Test reading basic Ints, Strs, Bools, and Null.
// Also test that whitespace and comments are correctly ignored.
#[test]
fn basic() {
    let obj = Obj::from_file("tests/test_files/basic.over").unwrap();

    assert_eq!(get_int(&obj, "a1"), 1);
    assert_eq!(get_int(&obj, "a2"), 2);
    assert_eq!(get_int(&obj, "aa"), 0);
    assert_eq!(obj.get("b").unwrap(), "Smörgåsbord");
    assert_eq!(get_int(&obj, "c"), 10);
    assert_eq!(get_int(&obj, "d"), 20);
    assert_eq!(get_int(&obj, "eee"), 2);
    assert_eq!(get_int(&obj, "f"), 3);
    assert_eq!(get_int(&obj, "g_"), 4);
    assert_eq!(obj.get("Hello").unwrap(), "Hello");
    assert_eq!(obj.get("i_robot").unwrap(), "not #a comment");
    assert_eq!(get_int(&obj, "j"), 4);
    assert_eq!(obj.get("k").unwrap(), "hi");
    assert_eq!(obj.get("l").unwrap(), "$\\\"");
    assert_eq!(obj.get("m").unwrap(), "m");
    assert_eq!(obj.get("n").unwrap(), true);
    assert_eq!(obj.get("o").unwrap(), false);
    assert_eq!(obj.get("p").unwrap(), "Hello");
    assert_eq!(get_int(&obj, "q"), 0);
    assert_eq!(obj.get("r").unwrap(), Value::Null);
    assert_eq!(obj.get("s").unwrap(), '\'');
    assert_eq!(obj.get("t").unwrap(), '\n');
    assert_eq!(obj.get("u").unwrap(), ' ');
}

// Test the example from the README.
#[test]
fn example() {
    let obj = Obj::from_file("tests/test_files/example.over").unwrap();

    assert_eq!(obj.get("receipt").unwrap(), "Oz-Ware Purchase Invoice");
    assert_eq!(obj.get("date").unwrap(), "2012-08-06");
    assert_eq!(
        obj.get("customer").unwrap(),
        obj_map!{"first_name" => "Dorothy",
                 "family_name" => "Gale"}
    );

    assert_eq!(
        obj.get("items").unwrap(),
        arr_vec![
            obj_map!{"part_no" => "A4786",
                     "descrip" => "Water Bucket (Filled)",
                     "price" => 1.47,
                     "quantity" => 4},
            obj_map!{"part_no" => "E1628",
                     "descrip" => "High Heeled \"Ruby\" Slippers",
                     "size" => 8,
                     "price" => 133.7,
                     "quantity" => 1},
        ]
    );

    assert_eq!(
        obj.get("bill_to").unwrap(),
        obj_map!{"street" => "123 Tornado Alley\nSuite 16",
                 "city" => "East Centerville",
                 "state" => "KS",
        }
    );

    assert_eq!(obj.get("ship_to").unwrap(), obj.get("bill_to").unwrap());

    assert_eq!(
        obj.get("specialDelivery").unwrap(),
        "Follow the Yellow Brick Road to the Emerald City. \
         Pay no attention to the man behind the curtain."
    );
}

// Test parsing of sub-Objs.
#[test]
fn obj() {
    let obj = Obj::from_file("tests/test_files/obj.over").unwrap();

    assert_eq!(obj.get("empty").unwrap().get_obj().unwrap().len(), 0);
    assert_eq!(obj.get("empty2").unwrap().get_obj().unwrap().len(), 0);

    assert!(!obj.contains("bools"));
    let mut bools = Obj::new();
    bools.set("t", true.into());
    bools.set("f", false.into());

    let outie = obj.get("outie").unwrap().get_obj().unwrap();
    assert_eq!(outie.get_parent().unwrap(), bools);
    assert_eq!(get_int(&outie, "z"), 0);
    let inner = outie.get("inner").unwrap().get_obj().unwrap();
    assert_eq!(get_int(&inner, "z"), 1);
    let innie = inner.get("innie").unwrap().get_obj().unwrap();
    assert_eq!(get_int(&innie, "a"), 1);
    assert_eq!(inner.get("b").unwrap(), tup_vec!(1, 2,));
    assert_eq!(get_int(&outie, "c"), 3);
    assert_eq!(outie.get("d").unwrap(), Obj::new());

    let obj_arr = obj.get("obj_arr").unwrap().get_obj().unwrap();
    assert_eq!(obj_arr.get("arr").unwrap(), arr_vec![1, 2, 3]);
}

// Test that globals are referenced correctly and don't get included as fields.
#[test]
fn globals() {
    let obj = Obj::from_file("tests/test_files/globals.over").unwrap();

    let sub = obj.get("sub").unwrap().get_obj().unwrap();

    assert_eq!(sub.get("a").unwrap().get_int().unwrap().to_i64(), Some(1));
    assert_eq!(get_int(&sub, "b"), 2);
    assert_eq!(sub.len(), 2);

    assert_eq!(get_int(&obj, "c"), 2);
    assert_eq!(obj.len(), 2);
}

// Test parsing of numbers.
#[test]
fn numbers() {
    let obj = Obj::from_file("tests/test_files/numbers.over").unwrap();

    assert_eq!(get_int(&obj, "neg"), -4);
    assert_eq!(
        obj.get("pos").unwrap().get_frac().unwrap().to_f32(),
        Some(4f32)
    );
    assert_eq!(
        obj.get("neg_zero").unwrap().get_frac().unwrap().to_f32(),
        Some(0f32)
    );
    assert_eq!(
        obj.get("pos_zero").unwrap().get_frac().unwrap().to_f32(),
        Some(0f32)
    );

    assert_eq!(
        obj.get("frac_from_dec").unwrap(),
        BigFraction::new(13u8, 10u8)
    );
    assert_eq!(
        obj.get("neg_ffd").unwrap(),
        BigFraction::new_neg(13u8, 10u8)
    );
    assert_eq!(obj.get("pos_ffd").unwrap(), BigFraction::new(13u8, 10u8));

    let frac = obj.get("big_frac").unwrap().get_frac().unwrap();
    assert!(frac > BigFraction::new(91_000_000u64, 1u8));
    assert!(frac < BigFraction::new(92_000_000u64, 1u8));
}

// TODO: Test includes.over

// TODO: Test multi-line.over (need substitution)

// Test fuzz files; just make sure there was no error in parsing.
#[test]
fn fuzz() {
    let _ = Obj::from_file("tests/test_files/fuzz1.over").unwrap();
}

// Test that parsing malformed .over files results in correct errors being returned.
#[test]
fn errors() {
    macro_rules! error_helper {
        ( $filename:expr, $error:expr ) => {
            {
                match Obj::from_file(&format!("tests/test_files/errors/{}", $filename)) {
                    Err(OverError::ParseError(s)) => {
                        if s != $error {
                            panic!("Error in {}: {:?}", $filename, s);
                        }
                    }
                    res => panic!("{:?}", res),
                }
            }
        }
    }

    error_helper!(
        "arr_types.over",
        "Arr inner types do not match: found Arr(Tup(Int, Char)), \
                   expected Arr(Tup(Int, Int))"
    );
    error_helper!("decimal.over", "Invalid numeric value at line 1, column 10");
    error_helper!(
        "deep.over",
        "Exceeded maximum depth (128) for a container at line 1, column 142"
    );
    error_helper!(
        "dup_global.over",
        "Duplicate global \"@global\" at line 2, column 1"
    );
    error_helper!(
        "empty_field.over",
        "Invalid character \':\' for field at line 1, column 1"
    );
    error_helper!(
        "empty_number.over",
        "Invalid numeric value at line 1, column 6"
    );
    error_helper!(
        "field_true.over",
        "Invalid field name \"true\" at line 1, column 1"
    );
    error_helper!(
        "field_whitespace.over",
        "No whitespace after field at line 1, column 6"
    );
    error_helper!(
        "fuzz1.over",
        "Invalid closing bracket \')\' at line 20, column 1; expected \']\'"
    );
    error_helper!(
        "fuzz2.over",
        "Invalid closing bracket \')\' at line 22, column 2; expected none"
    );
    error_helper!(
        "fuzz3.over",
        "Invalid closing bracket \')\' at line 8, column 4; expected \']\'"
    );
    error_helper!("fuzz4.over", "Duplicate field \"M\" at line 22, column 1");
    error_helper!(
        "fuzz5.over",
        "Invalid character \'(\' for value at line 27, column 4"
    );
    error_helper!(
        "fuzz6.over",
        "Arr inner types do not match: found Frac, expected Int"
    );
    error_helper!(
        "unexpected_end1.over",
        "Unexpected end when reading value at line 2"
    );
    error_helper!(
        "unexpected_end2.over",
        "Unexpected end when reading value at line 3"
    );
    error_helper!("value_amp.over", "Invalid value \"@\" at line 1, column 8");
}
