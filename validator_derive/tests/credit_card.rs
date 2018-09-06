#[macro_use]
extern crate validator_derive;
extern crate validator;

use validator::{Validate, ValidationErrorsKind};

#[cfg(feature = "card")]
#[test]
fn can_validate_valid_card_number() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(credit_card)]
        val: String,
    }

    let s = TestStruct {
        val: "5236313877109142".to_string(),
    };

    assert!(s.validate().is_ok());
}

#[cfg(feature = "card")]
#[test]
fn bad_credit_card_fails_validation() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(credit_card)]
        val: String,
    }

    let s = TestStruct {
        val: "bob".to_string(),
    };
    let res = s.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("val"));
    if let ValidationErrorsKind::Field(ref err) = errs["val"] {
        assert_eq!(err.len(), 1);
        assert_eq!(err[0].code, "credit_card");
        assert_eq!(err[0].params["value"], "bob");
    } else {
        panic!("Expected field validation errors");
    }
}

#[cfg(feature = "card")]
#[test]
fn can_specify_code_for_credit_card() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(credit_card(code = "oops"))]
        val: String,
    }
    let s = TestStruct {
        val: "bob".to_string(),
    };
    let res = s.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("val"));
    if let ValidationErrorsKind::Field(ref err) = errs["val"] {
        assert_eq!(err.len(), 1);
        assert_eq!(err[0].code, "oops");
    } else {
        panic!("Expected field validation errors");
    }
}

#[cfg(feature = "card")]
#[test]
fn can_specify_message_for_credit_card() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(credit_card(message = "oops"))]
        val: String,
    }
    let s = TestStruct {
        val: "bob".to_string(),
    };
    let res = s.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("val"));
    if let ValidationErrorsKind::Field(ref err) = errs["val"] {
        assert_eq!(err.len(), 1);
        assert_eq!(err[0].clone().message.unwrap(), "oops");
    } else {
        panic!("Expected field validation errors");
    }
}
