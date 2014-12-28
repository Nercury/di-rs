/*!
<a href="https://github.com/Nercury/di-rs">
    <img style="position: absolute; top: 0; left: 0; border: 0;" src="https://s3.amazonaws.com/github/ribbons/forkme_left_green_007200.png" alt="Fork me on GitHub">
</a>
<style>.sidebar { margin-top: 53px }</style>
*/

/*!

# Dependency Injection container for Rust

This page contains an abstract (although with few examples) overview
of this dependency injection container. To see more fine-grained
usage examples without much prose, browse the [Registry](registry/index.html)
documentation.

# Overview

## Declarative feature configuration

The primary motivation of this library is to create a mechanism
that helps to separate concept of a "feature" from libraries used.

For example, if a project needs X, Y and Z features, it should be
possible to declare a list of these features, and the features should
discover each other and configure themselves.

## Dependency injection (DI) as a method

First of all, while __dependency injection__ term is just a fancy name for
"passing arguments to constructor", it also communicates a specific
intent: __the user of these arguments does not care about
the way they were created__.

The simpliest form of dependency abstraction is use of trait objects
as construction arguments.

As an example, a `Logger` trait can be passed as an argument to something
that needs a `Logger`. Our library would depend on the abstraction,
and it would not need to care how this actual `Logger` logs the messages.

Without using this "di" library:

```rust
trait Logger { fn log(&self, m: &str); }

struct ConsoleLogger;
impl Logger for ConsoleLogger {
    fn log(&self, m: &str) {
        println!("{}", m);
    }
}

struct OurLibrary;
impl OurLibrary {
    fn new(logger: Box<Logger>) -> OurLibrary {
        logger.log("Library created!");
        OurLibrary
    }
}

OurLibrary::new(box ConsoleLogger); // will print "Library created!"
```

Pluging it into "di" looks like this:

```rust
# trait Logger { fn log(&self, m: &str); }
# struct ConsoleLogger;
# impl Logger for ConsoleLogger {
#     fn log(&self, m: &str) {
#         println!("{}", m);
#     }
# }
# struct OurLibrary;
# impl OurLibrary {
#     fn new(logger: Box<Logger>) -> OurLibrary {
#         logger.log("Library created!");
#         OurLibrary
#     }
# }
let mut registry = di::Registry::new();

registry.insert_one("logger", || box ConsoleLogger as Box<Logger>);

registry
    .one("our_library", |logger: Box<Logger + 'static>| {
        logger.log("Hello world!");
        OurLibrary
    })
    .with_arg("logger")
    .insert();

// The compilation phase is going to validate the registry
// definitions when the application is started.
match registry.compile() {
    Ok(container) => {
        container.get::<OurLibrary>("our_library").unwrap()
            .take(); // will print "Library created!"
    },
    Err(errors) => di::error_printer::pretty_print(&errors),
}
```

## Aggregate construction

Sometimes some dependency might be optional. Or maybe there
are multiple valid dependencies for the same interface.

Let's say there can be multiple `Backend` implementations for `Logger`.
It would be great if our `Logger` could depend on any amount of `Backend`
abstractions (without actually depending on implementations)
and get them collected into one list which could be injected as
an argument. This library can do that.

In this example, the concrete implementation would be `RedisBackend`, which
would also require some kind of `Redis` as another dependency.

```rust
use di::Registry;
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

match registry.compile() {
    Ok(container) => {
        // Get a factory that constructs the logger object.
        let logger_factory = container.get::<Logger>("logger").unwrap();
        // Actually invoke construction.
        let logger = logger_factory.take();

        logger.log("Loaded!");
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

# Discussion

## Why did this library happen

I wanted to implement DI mechanism for Rust because I had great success
with it before. The idea of "container" and "registering" definitions to
it [comes from Symfony2][symfony2-container-component] framework.

However, I intentionaly chose to avoid implementing things the same way
Symfony2 does.

For example, the factories do not return singletons. This `di` library
will always return a new value. If you really, really need them singletons,
well, you will find out how to work around that. What? You need MUTABLE
SINGLETONS? Go away.

The initialization mechanism requires a closure. If you do not like
that, well, it is possible to implement `metafactory::ToMetafactory` trait
for anything you would like to use as definition.

There is no lazy loading. Frankly, this is not PHP.

There is one other pattern that is a bit burried in Symfony2 di: it is the
ability to "tag" services and then have the container inject all of them
based on that "tag". [Using that is not straightforward][symfony2-tagged-services].
However, I found that it was the key of decoupling features properly, so I chose
to make them __very__ easy to use equivalent functionality in this library.
That's where `one_of` method came from.

[symfony2-container-component]: http://symfony.com/doc/current/components/dependency_injection/introduction.html
[symfony2-tagged-services]: http://symfony.com/doc/current/components/dependency_injection/tags.html

## Features can be libraries

When project feature list grows, different features must use the same
resources and respond to the same events. So, there needs to be some kind of
mechanism to __plug-in__ our feature into them. And then another feature.
And then yet another.

This becomes a problem when the addition or removal of a feature
requires developer to go all over the code and mechanically modify
countless if/match statements. It is a symptom that it is possible
to make a mechanism for plugging this and similar features in, and
move the feature itself to another library.

Note that no special DI tool is needed to acomplish that: but it might be
necessary if you need to put your development on highly configurable "rails"
of same implementation mechanism. What I mean, learn it once, use
everywhere.

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

## Further ideas

Currently registry definitions are initialized from clonable values or closures.
It is possible to extend it with unboxed closures and channels by implementing
`ToMetafactory` trait for them in `metafactory` library.

It might be possible to compile the `Container` at actual compilation instead of
runtime. That would probably require a compiler plugin.
*/

#![feature(slicing_syntax)]
#![feature(default_type_params)]

extern crate term;
extern crate typedef;
extern crate metafactory;

pub use registry::Registry;
pub use container::Container;

pub mod registry;
pub mod error_printer;
pub mod container;
