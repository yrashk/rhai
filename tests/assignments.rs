use rhai::{Engine, EvalAltResult, ParseErrorType, INT};

#[test]
fn test_assignments() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    assert_eq!(engine.eval::<INT>("let x = 42; x = 123; x")?, 123);
    assert_eq!(engine.eval::<INT>("let x = 42; x += 123; x")?, 165);

    #[cfg(not(feature = "no_index"))]
    assert_eq!(engine.eval::<INT>("let x = [42]; x[0] += 123; x[0]")?, 165);

    #[cfg(not(feature = "no_object"))]
    assert_eq!(engine.eval::<INT>("let x = #{a:42}; x.a += 123; x.a")?, 165);

    Ok(())
}

#[test]
fn test_assignments_bad_lhs() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    assert_eq!(
        *engine.compile(r"(x+y) = 42;").expect_err("should error").0,
        ParseErrorType::AssignmentToInvalidLHS("".to_string())
    );
    assert_eq!(
        *engine.compile(r"foo(x) = 42;").expect_err("should error").0,
        ParseErrorType::AssignmentToInvalidLHS("".to_string())
    );
    assert_eq!(
        *engine.compile(r"true = 42;").expect_err("should error").0,
        ParseErrorType::AssignmentToConstant("".to_string())
    );
    assert_eq!(
        *engine.compile(r"123 = 42;").expect_err("should error").0,
        ParseErrorType::AssignmentToConstant("".to_string())
    );

    #[cfg(not(feature = "no_object"))]
    {
        assert_eq!(
            *engine
                .compile(r"x.foo() = 42;")
                .expect_err("should error")
                .0,
            ParseErrorType::AssignmentToInvalidLHS("".to_string())
        );
        assert_eq!(
            *engine
                .compile(r"x.foo().x.y = 42;")
                .expect_err("should error")
                .0,
            ParseErrorType::AssignmentToInvalidLHS("".to_string())
        );
        assert_eq!(
            *engine
                .compile(r"x.y.z.foo() = 42;")
                .expect_err("should error")
                .0,
            ParseErrorType::AssignmentToInvalidLHS("".to_string())
        );
        #[cfg(not(feature = "no_index"))]
        assert_eq!(
            *engine
                .compile(r"x.foo()[0] = 42;")
                .expect_err("should error")
                .0,
            ParseErrorType::AssignmentToInvalidLHS("".to_string())
        );
        #[cfg(not(feature = "no_index"))]
        assert_eq!(
            *engine
                .compile(r"x[y].z.foo() = 42;")
                .expect_err("should error")
                .0,
            ParseErrorType::AssignmentToInvalidLHS("".to_string())
        );
    }

    Ok(())
}
