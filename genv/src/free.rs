use rhai::{Engine,Scope, EvalAltResult,Position};
use rhai::module_resolvers::FileModuleResolver;
 use rhai::packages::{Package,StandardPackage};

fn main()-> Result<(), Box<EvalAltResult>>{
    // Create a raw scripting Engine
    let mut engine = Engine::new_raw();

// Use the file-based module resolver
    engine.set_module_resolver(FileModuleResolver::new());

// Default print/debug implementations
    engine.on_print(|text| println!("{text}"));

    engine.on_debug(|text, source, pos| match (source, pos) {
        (Some(source), crate::Position::NONE) => println!("{source} | {text}"),
        (Some(source), pos) => println!("{source} @ {pos:?} | {text}"),
        (None, crate::Position::NONE) => println!("{text}"),
        (None, pos) => println!("{pos:?} | {text}"),
    });

// Register the Standard Package
    let package = StandardPackage::new();

// Load the package into the [`Engine`]
    package.register_into_engine(&mut engine);


// First create the state
    let mut scope = Scope::new();

// Then push (i.e. add) some initialized variables into the state.
// Remember the system number types in Rhai are i64 (i32 if 'only_i32')
// and f64 (f32 if 'f32_float').
// Better stick to them or it gets hard working with the script.
    scope.push("y", 1_i64)
        .push("z", 2_i64)
        .push_constant("MY_NUMBER", 2_i64)       // constants can also be added
        .set_value("s", "hello, world!");          // 'set_value' adds a new variable when one doesn't exist

    let result = engine.eval::<i64>("40 + 2")?;
    //                      ^^^^^^^ required: cast the result to a type

    println!("Answer: {result}");             // prints 42

    // First invocation
    engine.run_with_scope(&mut scope,
                          "
    let x =  y + z + MY_NUMBER  ;
    y = 3;
")?;

// Second invocation using the same state.
// Notice that the new variable 'x', defined previously, is still here.
    let result = engine.eval_with_scope::<i64>(&mut scope, "x + y")?;

    println!("result: {result}");

    assert_eq!(scope.get_value::<i64>("y").expect("variable y should exist"), 3);

// We can modify scope variables directly with 'set_value'
    scope.set_value("y", 42_i64);
    assert_eq!(scope.get_value::<i64>("y").expect("variable y should exist"), 42);
    Ok(())
}
