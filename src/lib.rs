/*!
<a href="https://github.com/Nercury/di-rs">
    <img style="position: absolute; top: 0; left: 0; border: 0;" src="https://s3.amazonaws.com/github/ribbons/forkme_left_green_007200.png" alt="Fork me on GitHub">
</a>
<style>.sidebar { margin-top: 53px }</style>
*/

/*!

## Declarative feature configuration

The primary motivation of this library is to create a mechanism
that helps to separate concerns of a project from tools.

For example, my project will need X, Y and Z features. Idealy,
I should be able to simply declare a list of them, and they should
discover each other themselves.

## Dependency injection as a method

First of all, while __dependency injection__ term is just a fancy name for
"passing arguments to constructor", it also communicates a specific
intent: __the user of these arguments does not care about
the way they were created__.

The simpliest form of dependency abstraction is use of trait objects
as construction arguments.

For example, there is an argument for something that implements "Logger"
trait. Our library depends on "logger", and it does not care how the
actual "logger" logs the messages.

## Aggregate construction

Sometimes some dependency might be optional. Or maybe there
are multiple valid dependencies for the same interface.

Let's suppose there can be multiple "logger" backends. It would be
great if we could depend on any amount of "loggers" and get them
collected into one list. This library can do that.

## Features can be libraries

When project feature list grows, different features must use the same
resources and respond to the same events. So, this needs some kind of
mechanism to __plug-in__ our feature into them. And then another feature.
And then yet another.

This becomes a problem when the addition or removal of a feature
requires developer to go all over the code and mechanically modify
countless if/match statements. It is a symptom that it is possible
to make a mechanism for plugging this and similar features in, and
move the feature itself to another library.

## Dependency validation

Currently this library validates dependencies at startup.
There is an error printer that can output all errors into the terminal.

## Example

Let's say there is a `Logger` object that can log to any backend that implements
`Backend` trait, and if `redis_logs` feature is enabled, it will log to
`RedisBackend`.

Also, the `RedisBackend` requires `Redis` as a direct dependency.

```rust
use di::registry::Registry;
# struct Redis;
# impl Redis { fn new() -> Redis { Redis } }
# struct RedisBackend { runner: Redis };
# impl RedisBackend {
#     fn new(runner: Redis) -> RedisBackend {
#         RedisBackend { runner: runner }
#     }
# }
# impl Backend for RedisBackend {}
# trait Backend {}
# struct Logger { backends: Vec<Box<Backend + 'static>> }
# impl Logger {
#     fn new(backends: Vec<Box<Backend + 'static>>) -> Logger {
#         Logger {
#             backends: backends
#         }
#     }
#     fn log(&self, _m: &str) {}
# }

fn enable_redis_logs(registry: &mut Registry) {
    registry
        .one_of("backends", |redis| {
            box RedisBackend::new(redis) as Box<Backend>
        })
        .add_arg("redis") // It will need "redis"
        .insert();

    registry
        .one("redis", || Redis::new())
        .insert();
}

fn enable_logger(registry: &mut Registry) {
    registry
        .one("logger", |backends| {
            Logger::new(backends)
        })
        .add_arg("backends")
        .insert();

    registry.may_be_empty::<Box<Backend>>("backends");
}

let mut registry = Registry::new();

// List of features in our application.
enable_redis_logs(&mut registry);
enable_logger(&mut registry);

// The compilation phase is going to validate the registry
// definitions when the application is started.
match registry.compile() {
    Ok(container) => {
        //let view_factory = container.get("logger").unwrap();
        //let view = view_factory.take();

        // Use view:
        //view.show();
    },
    Err(errors) => {
        di::error_printer::pretty_print(&errors);
        panic!("expected no errors");
    }
}
```

Obviously, the above is equivalent to this simple code:

```rust
# struct Redis;
# impl Redis { fn new() -> Redis { Redis } }
# struct RedisBackend { runner: Redis };
# impl RedisBackend {
#     fn new(runner: Redis) -> RedisBackend {
#         RedisBackend { runner: runner }
#     }
# }
# impl Backend for RedisBackend {}
# trait Backend {}
# struct Logger { backends: Vec<Box<Backend + 'static>> }
# impl Logger {
#     fn new(backends: Vec<Box<Backend + 'static>>) -> Logger {
#         Logger {
#             backends: backends
#         }
#     }
#     fn log(&self, _m: &str) {}
# }

fn get_logger_factory() -> Logger {
    // Regis logs feature
    let redis = Redis::new();
    let redis_backend = box RedisBackend::new(redis) as Box<Backend>;

    // Logger feature
    let logger = Logger::new(vec![redis_backend]);

    logger
}

// Use logger:
let logger = get_logger_factory();
logger.log("hello");
```

In fact, `registry.compile` constructs internal execution structure
that is very similar to `get_logger_factory`, but it is done at
runtime.

If the "redis logs" is not enabled, the `enable_logger` code does not
need re-compilation, and constructed `get_logger_factory` won't
have any code related to redis:

```ignore
fn get_logger_factory() -> Logger {
    // Logger feature
    let logger = Logger::new(Vec::new());

    logger
}
```

## The roles of `Registry` and `Container`

`Registry` is mutable, `Container` is immutable.
`Registry` is changed on initialization, then it validates all definitions and
"compiles" the `Container`, which is used at runtime.
Further changes to `Registry` can be used to produce a new, different
`Container`.

HashMap and Any are used only on `Registry` configuration phase.
If there are any mistakes, the validator should produce error messages
that pinpoint problems accurately.

The compiled container contains initialization trees constructed from
registry definitions. If the `Container` construction succeeds, factories
returned by container should never fail. If they do, it is a bug.

## The true cost of a factory

`Container` will return copies of constructed factories,
which are nothing more than object-initialization trees that can be invoked.

While it was certainly possible to eliminate a lot of indirection in this
tree using unsafe hacks, this library currently uses __no__ unsafe code.

This is a approximate example of calls the
`Logger` construction with `redis_logs` enabled would make:

 - `Factory<Logger>.take()` - wraps `Logger` getter mechanism in struct
 - `Closure_ManyArg_1<Vec<Box<Backend>>, Logger>.take()` - closure dependency scope
     - `AggregateGetter<Vec<Box<Backend>>>.take()` - aggregates ("one_of") dependencies
        - `Factory<Box<Backend>>.take()` - wraps getter mechanism in struct
        - `Closure_ManyArg_1<Redis, Box<Backend>>.take()` - closure dependency scope
            - `Factory<Redis>.take()`
            - `Closure_ZeroArg<Redis>.take()` - call Redis construction closure
            - `Rc<RefCell<|| -> Redis>>.call()`
        - `Rc<RefCell<|Redis| -> Box<Backend>>>.call()` - call `RedisBackend` construction closure
 - `Rc<RefCell<|Vec<Box<Redis>>| -> Logger>>.call()` - call `Logger` construction closure

The constly point here is probably a `Rc<RefCell>` and allocation of
`Vec<Box<Backends>>`, though this is just a guess at this point.
But it shouldn't be a problem, especially when compared to `registry.compile`.
*/

#![feature(slicing_syntax)]
#![feature(default_type_params)]

extern crate term;
extern crate typedef;
extern crate metafactory;

pub mod registry;
pub mod error_printer;
pub mod container;
