//! Auto-generated bindings for WASI interfaces.
//!
//! This module contains the output of the [`bindgen!`] macro when run over
//! the `wasi:cli/imports` world.
//!
//! [`bindgen!`]: https://docs.rs/wasmtime/latest/wasmtime/component/macro.bindgen.html
//!
//! # Examples
//!
//! If you have a WIT world which refers to WASI interfaces you probably want to
//! use this modules's bindings rather than generate fresh bindings. That can be
//! done using the `with` option to [`bindgen!`]:
//!
//! ```rust
//! use wasmtime_wasi::{WasiCtx, WasiCtxView, WasiView};
//! use wasmtime::{Result, Engine, Config};
//! use wasmtime::component::{Linker, HasSelf, ResourceTable};
//!
//! wasmtime::component::bindgen!({
//!     inline: "
//!         package example:wasi;
//!
//!         // An example of extending the `wasi:cli/command` world with a
//!         // custom host interface.
//!         world my-world {
//!             include wasi:cli/command@0.3.0;
//!
//!             import custom-host;
//!         }
//!
//!         interface custom-host {
//!             my-custom-function: func();
//!         }
//!     ",
//!     path: "src/p3/wit",
//!     with: {
//!         "wasi": wasmtime_wasi::p3::bindings,
//!     },
//!     require_store_data_send: true,
//! });
//!
//! struct MyState {
//!     ctx: WasiCtx,
//!     table: ResourceTable,
//! }
//!
//! impl example::wasi::custom_host::Host for MyState {
//!     fn my_custom_function(&mut self) {
//!         // ..
//!     }
//! }
//!
//! impl WasiView for MyState {
//!     fn ctx(&mut self) -> WasiCtxView<'_> {
//!         WasiCtxView{
//!             ctx: &mut self.ctx,
//!             table: &mut self.table,
//!         }
//!     }
//! }
//!
//! fn main() -> Result<()> {
//!     let mut config = Config::default();
//!     config.wasm_component_model_async(true);
//!     let engine = Engine::new(&config)?;
//!     let mut linker: Linker<MyState> = Linker::new(&engine);
//!     wasmtime_wasi::p3::add_to_linker(&mut linker)?;
//!     example::wasi::custom_host::add_to_linker::<_, HasSelf<_>>(&mut linker, |state| state)?;
//!
//!     // .. use `Linker` to instantiate component ...
//!
//!     Ok(())
//! }
//! ```

mod generated {
    #[doc = "/ Link-time configurations."]
    #[derive(Clone, Debug, Default)]
    pub struct LinkOptions {
        clocks_timezone: bool,
    }
    impl LinkOptions {
        #[doc = "/ Enable members marked as `@unstable(feature = clocks-timezone)`"]
        pub fn clocks_timezone(&mut self, enabled: bool) -> &mut Self {
            self.clocks_timezone = enabled;
            self
        }
    }
    type _TrappableError0 = crate::p3::filesystem::FilesystemError;
    type _TrappableError1 = crate::p3::sockets::SocketError;
    #[doc(hidden)]
    pub use crate::filesystem::Descriptor as __with_name2;
    #[doc(hidden)]
    pub use crate::p3::cli::TerminalInput as __with_name0;
    #[doc(hidden)]
    pub use crate::p3::cli::TerminalOutput as __with_name1;
    #[doc(hidden)]
    pub use crate::sockets::TcpSocket as __with_name3;
    #[doc(hidden)]
    pub use crate::sockets::UdpSocket as __with_name4;
    impl core::convert::From<LinkOptions> for wasi::clocks::timezone::LinkOptions {
        fn from(src: LinkOptions) -> Self {
            (&src).into()
        }
    }
    impl core::convert::From<&LinkOptions> for wasi::clocks::timezone::LinkOptions {
        fn from(src: &LinkOptions) -> Self {
            let mut dest = Self::default();
            dest.clocks_timezone(src.clocks_timezone);
            dest
        }
    }
    #[doc = "/ Auto-generated bindings for a pre-instantiated version of a"]
    #[doc = "/ component which implements the world `command`."]
    #[doc = "/"]
    #[doc = "/ This structure is created through [`CommandPre::new`] which"]
    #[doc = "/ takes a [`InstancePre`](wasmtime::component::InstancePre) that"]
    #[doc = "/ has been created through a [`Linker`](wasmtime::component::Linker)."]
    #[doc = "/"]
    #[doc = "/ For more information see [`Command`] as well."]
    pub struct CommandPre<T: 'static> {
        instance_pre: wasmtime::component::InstancePre<T>,
        indices: CommandIndices,
    }
    impl<T: 'static> Clone for CommandPre<T> {
        fn clone(&self) -> Self {
            Self {
                instance_pre: self.instance_pre.clone(),
                indices: self.indices.clone(),
            }
        }
    }
    impl<_T: 'static> CommandPre<_T> {
        #[doc = "/ Creates a new copy of `CommandPre` bindings which can then"]
        #[doc = "/ be used to instantiate into a particular store."]
        #[doc = "/"]
        #[doc = "/ This method may fail if the component behind `instance_pre`"]
        #[doc = "/ does not have the required exports."]
        pub fn new(instance_pre: wasmtime::component::InstancePre<_T>) -> wasmtime::Result<Self> {
            let indices = CommandIndices::new(&instance_pre)?;
            Ok(Self {
                instance_pre,
                indices,
            })
        }
        pub fn engine(&self) -> &wasmtime::Engine {
            self.instance_pre.engine()
        }
        pub fn instance_pre(&self) -> &wasmtime::component::InstancePre<_T> {
            &self.instance_pre
        }
        #[doc = "/ Instantiates a new instance of [`Command`] within the"]
        #[doc = "/ `store` provided."]
        #[doc = "/"]
        #[doc = "/ This function will use `self` as the pre-instantiated"]
        #[doc = "/ instance to perform instantiation. Afterwards the preloaded"]
        #[doc = "/ indices in `self` are used to lookup all exports on the"]
        #[doc = "/ resulting instance."]
        pub fn instantiate(
            &self,
            mut store: impl wasmtime::AsContextMut<Data = _T>,
        ) -> wasmtime::Result<Command> {
            let mut store = store.as_context_mut();
            let instance = self.instance_pre.instantiate(&mut store)?;
            self.indices.load(&mut store, &instance)
        }
    }
    impl<_T: Send + 'static> CommandPre<_T> {
        #[doc = "/ Same as [`Self::instantiate`], except with `async`."]
        pub async fn instantiate_async(
            &self,
            mut store: impl wasmtime::AsContextMut<Data = _T>,
        ) -> wasmtime::Result<Command> {
            let mut store = store.as_context_mut();
            let instance = self.instance_pre.instantiate_async(&mut store).await?;
            self.indices.load(&mut store, &instance)
        }
    }
    #[doc = "/ Auto-generated bindings for index of the exports of"]
    #[doc = "/ `command`."]
    #[doc = "/"]
    #[doc = "/ This is an implementation detail of [`CommandPre`] and can"]
    #[doc = "/ be constructed if needed as well."]
    #[doc = "/"]
    #[doc = "/ For more information see [`Command`] as well."]
    #[derive(Clone)]
    pub struct CommandIndices {
        interface0: exports::wasi::cli::run::GuestIndices,
    }
    #[doc = "/ Auto-generated bindings for an instance a component which"]
    #[doc = "/ implements the world `command`."]
    #[doc = "/"]
    #[doc = "/ This structure can be created through a number of means"]
    #[doc = "/ depending on your requirements and what you have on hand:"]
    #[doc = "/"]
    #[doc = "/ * The most convenient way is to use"]
    #[doc = "/   [`Command::instantiate`] which only needs a"]
    #[doc = "/   [`Store`], [`Component`], and [`Linker`]."]
    #[doc = "/"]
    #[doc = "/ * Alternatively you can create a [`CommandPre`] ahead of"]
    #[doc = "/   time with a [`Component`] to front-load string lookups"]
    #[doc = "/   of exports once instead of per-instantiation. This"]
    #[doc = "/   method then uses [`CommandPre::instantiate`] to"]
    #[doc = "/   create a [`Command`]."]
    #[doc = "/"]
    #[doc = "/ * If you\'ve instantiated the instance yourself already"]
    #[doc = "/   then you can use [`Command::new`]."]
    #[doc = "/"]
    #[doc = "/ These methods are all equivalent to one another and move"]
    #[doc = "/ around the tradeoff of what work is performed when."]
    #[doc = "/"]
    #[doc = "/ [`Store`]: wasmtime::Store"]
    #[doc = "/ [`Component`]: wasmtime::component::Component"]
    #[doc = "/ [`Linker`]: wasmtime::component::Linker"]
    pub struct Command {
        interface0: exports::wasi::cli::run::Guest,
    }
    const _: () = {
        impl CommandIndices {
            #[doc = "/ Creates a new copy of `CommandIndices` bindings which can then"]
            #[doc = "/ be used to instantiate into a particular store."]
            #[doc = "/"]
            #[doc = "/ This method may fail if the component does not have the"]
            #[doc = "/ required exports."]
            pub fn new<_T>(
                _instance_pre: &wasmtime::component::InstancePre<_T>,
            ) -> wasmtime::Result<Self> {
                let _component = _instance_pre.component();
                let _instance_type = _instance_pre.instance_type();
                let interface0 = exports::wasi::cli::run::GuestIndices::new(_instance_pre)?;
                Ok(CommandIndices { interface0 })
            }
            #[doc = "/ Uses the indices stored in `self` to load an instance"]
            #[doc = "/ of [`Command`] from the instance provided."]
            #[doc = "/"]
            #[doc = "/ Note that at this time this method will additionally"]
            #[doc = "/ perform type-checks of all exports."]
            pub fn load(
                &self,
                mut store: impl wasmtime::AsContextMut,
                instance: &wasmtime::component::Instance,
            ) -> wasmtime::Result<Command> {
                let _ = &mut store;
                let _instance = instance;
                let interface0 = self.interface0.load(&mut store, &_instance)?;
                Ok(Command { interface0 })
            }
        }
        impl Command {
            #[doc = "/ Convenience wrapper around [`CommandPre::new`] and"]
            #[doc = "/ [`CommandPre::instantiate`]."]
            pub fn instantiate<_T>(
                store: impl wasmtime::AsContextMut<Data = _T>,
                component: &wasmtime::component::Component,
                linker: &wasmtime::component::Linker<_T>,
            ) -> wasmtime::Result<Command> {
                let pre = linker.instantiate_pre(component)?;
                CommandPre::new(pre)?.instantiate(store)
            }
            #[doc = "/ Convenience wrapper around [`CommandIndices::new`] and"]
            #[doc = "/ [`CommandIndices::load`]."]
            pub fn new(
                mut store: impl wasmtime::AsContextMut,
                instance: &wasmtime::component::Instance,
            ) -> wasmtime::Result<Command> {
                let indices = CommandIndices::new(&instance.instance_pre(&store))?;
                indices.load(&mut store, instance)
            }
            #[doc = "/ Convenience wrapper around [`CommandPre::new`] and"]
            #[doc = "/ [`CommandPre::instantiate_async`]."]
            pub async fn instantiate_async<_T>(
                store: impl wasmtime::AsContextMut<Data = _T>,
                component: &wasmtime::component::Component,
                linker: &wasmtime::component::Linker<_T>,
            ) -> wasmtime::Result<Command>
            where
                _T: Send,
            {
                let pre = linker.instantiate_pre(component)?;
                CommandPre::new(pre)?.instantiate_async(store).await
            }
            pub fn add_to_linker<T, D>(
                linker: &mut wasmtime::component::Linker<T>,
                options: &LinkOptions,
                host_getter: fn(&mut T) -> D::Data<'_>,
            ) -> wasmtime::Result<()>
            where
                D: wasi::cli::environment::HostWithStore<T>
                    + wasi::cli::exit::HostWithStore<T>
                    + wasi::cli::types::HostWithStore<T>
                    + wasi::cli::stdin::HostWithStore<T>
                    + wasi::cli::stdout::HostWithStore<T>
                    + wasi::cli::stderr::HostWithStore<T>
                    + wasi::cli::terminal_input::HostWithStore<T>
                    + wasi::cli::terminal_output::HostWithStore<T>
                    + wasi::cli::terminal_stdin::HostWithStore<T>
                    + wasi::cli::terminal_stdout::HostWithStore<T>
                    + wasi::cli::terminal_stderr::HostWithStore<T>
                    + wasi::clocks::types::HostWithStore<T>
                    + wasi::clocks::monotonic_clock::HostWithStore<T>
                    + wasi::clocks::system_clock::HostWithStore<T>
                    + wasi::clocks::timezone::HostWithStore<T>
                    + wasi::filesystem::types::HostWithStore<T>
                    + wasi::filesystem::preopens::HostWithStore<T>
                    + wasi::sockets::types::HostWithStore<T>
                    + wasi::sockets::ip_name_lookup::HostWithStore<T>
                    + wasi::random::random::HostWithStore<T>
                    + wasi::random::insecure::HostWithStore<T>
                    + wasi::random::insecure_seed::HostWithStore<T>
                    + Send,
                for<'a> D::Data<'a>: wasi::cli::environment::Host
                    + wasi::cli::exit::Host
                    + wasi::cli::types::Host
                    + wasi::cli::stdin::Host
                    + wasi::cli::stdout::Host
                    + wasi::cli::stderr::Host
                    + wasi::cli::terminal_input::Host
                    + wasi::cli::terminal_output::Host
                    + wasi::cli::terminal_stdin::Host
                    + wasi::cli::terminal_stdout::Host
                    + wasi::cli::terminal_stderr::Host
                    + wasi::clocks::types::Host
                    + wasi::clocks::monotonic_clock::Host
                    + wasi::clocks::system_clock::Host
                    + wasi::clocks::timezone::Host
                    + wasi::filesystem::types::Host
                    + wasi::filesystem::preopens::Host
                    + wasi::sockets::types::Host
                    + wasi::sockets::ip_name_lookup::Host
                    + wasi::random::random::Host
                    + wasi::random::insecure::Host
                    + wasi::random::insecure_seed::Host
                    + Send,
                T: 'static + Send,
            {
                wasi::cli::environment::add_to_linker::<T, D>(linker, host_getter)?;
                wasi::cli::exit::add_to_linker::<T, D>(linker, host_getter)?;
                wasi::cli::types::add_to_linker::<T, D>(linker, host_getter)?;
                wasi::cli::stdin::add_to_linker::<T, D>(linker, host_getter)?;
                wasi::cli::stdout::add_to_linker::<T, D>(linker, host_getter)?;
                wasi::cli::stderr::add_to_linker::<T, D>(linker, host_getter)?;
                wasi::cli::terminal_input::add_to_linker::<T, D>(linker, host_getter)?;
                wasi::cli::terminal_output::add_to_linker::<T, D>(linker, host_getter)?;
                wasi::cli::terminal_stdin::add_to_linker::<T, D>(linker, host_getter)?;
                wasi::cli::terminal_stdout::add_to_linker::<T, D>(linker, host_getter)?;
                wasi::cli::terminal_stderr::add_to_linker::<T, D>(linker, host_getter)?;
                wasi::clocks::types::add_to_linker::<T, D>(linker, host_getter)?;
                wasi::clocks::monotonic_clock::add_to_linker::<T, D>(linker, host_getter)?;
                wasi::clocks::system_clock::add_to_linker::<T, D>(linker, host_getter)?;
                if options.clocks_timezone {
                    wasi::clocks::timezone::add_to_linker::<T, D>(
                        linker,
                        &options.into(),
                        host_getter,
                    )?;
                }
                wasi::filesystem::types::add_to_linker::<T, D>(linker, host_getter)?;
                wasi::filesystem::preopens::add_to_linker::<T, D>(linker, host_getter)?;
                wasi::sockets::types::add_to_linker::<T, D>(linker, host_getter)?;
                wasi::sockets::ip_name_lookup::add_to_linker::<T, D>(linker, host_getter)?;
                wasi::random::random::add_to_linker::<T, D>(linker, host_getter)?;
                wasi::random::insecure::add_to_linker::<T, D>(linker, host_getter)?;
                wasi::random::insecure_seed::add_to_linker::<T, D>(linker, host_getter)?;
                Ok(())
            }
            pub fn wasi_cli_run(&self) -> &exports::wasi::cli::run::Guest {
                &self.interface0
            }
        }
    };
    pub mod wasi {
        pub mod cli {
            #[allow(clippy::all)]
            pub mod environment {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::Box;
                pub trait HostWithStore<T>: wasmtime::component::HasData {}
                impl<H: ?Sized, T> HostWithStore<T> for H where H: wasmtime::component::HasData {}
                pub trait Host {
                    #[doc = "/ Get the POSIX-style environment variables."]
                    #[doc = "/ "]
                    #[doc = "/ Each environment variable is provided as a pair of string variable names"]
                    #[doc = "/ and string value."]
                    #[doc = "/ "]
                    #[doc = "/ Morally, these are a value import, but until value imports are available"]
                    #[doc = "/ in the component model, this import function should return the same"]
                    #[doc = "/ values each time it is called."]
                    fn get_environment(
                        &mut self,
                    ) -> wasmtime::Result<
                        wasmtime::component::__internal::Vec<(
                            wasmtime::component::__internal::String,
                            wasmtime::component::__internal::String,
                        )>,
                    >;

                    #[doc = "/ Get the POSIX-style arguments to the program."]
                    fn get_arguments(
                        &mut self,
                    ) -> wasmtime::Result<
                        wasmtime::component::__internal::Vec<
                            wasmtime::component::__internal::String,
                        >,
                    >;

                    #[doc = "/ Return a path that programs should use as their initial current working"]
                    #[doc = "/ directory, interpreting `.` as shorthand for this."]
                    fn get_initial_cwd(
                        &mut self,
                    ) -> wasmtime::Result<Option<wasmtime::component::__internal::String>>;
                }
                impl<_T: Host + ?Sized> Host for &mut _T {
                    #[doc = "/ Get the POSIX-style environment variables."]
                    #[doc = "/ "]
                    #[doc = "/ Each environment variable is provided as a pair of string variable names"]
                    #[doc = "/ and string value."]
                    #[doc = "/ "]
                    #[doc = "/ Morally, these are a value import, but until value imports are available"]
                    #[doc = "/ in the component model, this import function should return the same"]
                    #[doc = "/ values each time it is called."]
                    fn get_environment(
                        &mut self,
                    ) -> wasmtime::Result<
                        wasmtime::component::__internal::Vec<(
                            wasmtime::component::__internal::String,
                            wasmtime::component::__internal::String,
                        )>,
                    > {
                        Host::get_environment(*self)
                    }
                    #[doc = "/ Get the POSIX-style arguments to the program."]
                    fn get_arguments(
                        &mut self,
                    ) -> wasmtime::Result<
                        wasmtime::component::__internal::Vec<
                            wasmtime::component::__internal::String,
                        >,
                    > {
                        Host::get_arguments(*self)
                    }
                    #[doc = "/ Return a path that programs should use as their initial current working"]
                    #[doc = "/ directory, interpreting `.` as shorthand for this."]
                    fn get_initial_cwd(
                        &mut self,
                    ) -> wasmtime::Result<Option<wasmtime::component::__internal::String>>
                    {
                        Host::get_initial_cwd(*self)
                    }
                }
                pub fn add_to_linker_instance<T, D>(
                    inst: &mut wasmtime::component::LinkerInstance<'_, T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    inst.func_wrap(
                        "get-environment",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "environment",
                                function = "get-environment",
                            );
                            let _enter = span.enter();
                            tracing::event!(tracing::Level::TRACE, "call");
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::get_environment(host);
                            tracing::event!(
                                tracing::Level::TRACE,
                                result = tracing::field::debug("..."),
                                "return"
                            );
                            Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                        },
                    )?;
                    inst.func_wrap(
                        "get-arguments",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "environment",
                                function = "get-arguments",
                            );
                            let _enter = span.enter();
                            tracing::event!(tracing::Level::TRACE, "call");
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::get_arguments(host);
                            tracing::event!(
                                tracing::Level::TRACE,
                                result = tracing::field::debug("..."),
                                "return"
                            );
                            Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                        },
                    )?;
                    inst.func_wrap(
                        "get-initial-cwd",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "environment",
                                function = "get-initial-cwd",
                            );
                            let _enter = span.enter();
                            tracing::event!(tracing::Level::TRACE, "call");
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::get_initial_cwd(host);
                            tracing::event!(
                                tracing::Level::TRACE,
                                result = tracing::field::debug(&r),
                                "return"
                            );
                            Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                        },
                    )?;
                    Ok(())
                }
                pub fn add_to_linker<T, D>(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    let mut inst = linker.instance("wasi:cli/environment@0.3.0")?;
                    add_to_linker_instance::<T, D>(&mut inst, host_getter)
                }
            }
            #[allow(clippy::all)]
            pub mod exit {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::Box;
                pub trait HostWithStore<T>: wasmtime::component::HasData {}
                impl<H: ?Sized, T> HostWithStore<T> for H where H: wasmtime::component::HasData {}
                pub trait Host {
                    #[doc = "/ Exit the current instance and any linked instances."]
                    fn exit(&mut self, status: Result<(), ()>) -> wasmtime::Result<()>;

                    #[doc = "/ Exit the current instance and any linked instances, reporting the"]
                    #[doc = "/ specified status code to the host."]
                    #[doc = "/ "]
                    #[doc = "/ The meaning of the code depends on the context, with 0 usually meaning"]
                    #[doc = "/ \"success\", and other values indicating various types of failure."]
                    #[doc = "/ "]
                    #[doc = "/ This function does not return; the effect is analogous to a trap, but"]
                    #[doc = "/ without the connotation that something bad has happened."]
                    fn exit_with_code(&mut self, status_code: u8) -> wasmtime::Result<()>;
                }
                impl<_T: Host + ?Sized> Host for &mut _T {
                    #[doc = "/ Exit the current instance and any linked instances."]
                    fn exit(&mut self, status: Result<(), ()>) -> wasmtime::Result<()> {
                        Host::exit(*self, status)
                    }
                    #[doc = "/ Exit the current instance and any linked instances, reporting the"]
                    #[doc = "/ specified status code to the host."]
                    #[doc = "/ "]
                    #[doc = "/ The meaning of the code depends on the context, with 0 usually meaning"]
                    #[doc = "/ \"success\", and other values indicating various types of failure."]
                    #[doc = "/ "]
                    #[doc = "/ This function does not return; the effect is analogous to a trap, but"]
                    #[doc = "/ without the connotation that something bad has happened."]
                    fn exit_with_code(&mut self, status_code: u8) -> wasmtime::Result<()> {
                        Host::exit_with_code(*self, status_code)
                    }
                }
                pub fn add_to_linker_instance<T, D>(
                    inst: &mut wasmtime::component::LinkerInstance<'_, T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    inst.func_wrap(
                        "exit",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>,
                              (arg0,): (Result<(), ()>,)| {
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "exit",
                                function = "exit",
                            );
                            let _enter = span.enter();
                            tracing::event!(
                                tracing::Level::TRACE,
                                status = tracing::field::debug(&arg0),
                                "call"
                            );
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::exit(host, arg0);
                            tracing::event!(
                                tracing::Level::TRACE,
                                result = tracing::field::debug(&r),
                                "return"
                            );
                            wasmtime::ToWasmtimeResult::to_wasmtime_result(r)
                        },
                    )?;
                    inst.func_wrap(
                        "exit-with-code",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (u8,)| {
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "exit",
                                function = "exit-with-code",
                            );
                            let _enter = span.enter();
                            tracing::event!(
                                tracing::Level::TRACE,
                                status_code = tracing::field::debug(&arg0),
                                "call"
                            );
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::exit_with_code(host, arg0);
                            tracing::event!(
                                tracing::Level::TRACE,
                                result = tracing::field::debug(&r),
                                "return"
                            );
                            wasmtime::ToWasmtimeResult::to_wasmtime_result(r)
                        },
                    )?;
                    Ok(())
                }
                pub fn add_to_linker<T, D>(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    let mut inst = linker.instance("wasi:cli/exit@0.3.0")?;
                    add_to_linker_instance::<T, D>(&mut inst, host_getter)
                }
            }
            #[allow(clippy::all)]
            pub mod types {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::Box;
                #[derive(
                    wasmtime::component::ComponentType,
                    wasmtime::component::Lift,
                    wasmtime::component::Lower,
                )]
                #[component(enum)]
                #[derive(Clone, Copy, Eq, PartialEq)]
                #[repr(u8)]
                pub enum ErrorCode {
                    #[doc = "/ Input/output error"]
                    #[component(name = "io")]
                    Io,
                    #[doc = "/ Invalid or incomplete multibyte or wide character"]
                    #[component(name = "illegal-byte-sequence")]
                    IllegalByteSequence,
                    #[doc = "/ Broken pipe"]
                    #[component(name = "pipe")]
                    Pipe,
                }
                impl core::fmt::Debug for ErrorCode {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        match self {
                            ErrorCode::Io => f.debug_tuple("ErrorCode::Io").finish(),
                            ErrorCode::IllegalByteSequence => {
                                f.debug_tuple("ErrorCode::IllegalByteSequence").finish()
                            }
                            ErrorCode::Pipe => f.debug_tuple("ErrorCode::Pipe").finish(),
                        }
                    }
                }
                const _: () = {
                    assert!(1 == <ErrorCode as wasmtime::component::ComponentType>::SIZE32);
                    assert!(1 == <ErrorCode as wasmtime::component::ComponentType>::ALIGN32);
                };
                pub trait HostWithStore<T>: wasmtime::component::HasData {}
                impl<H: ?Sized, T> HostWithStore<T> for H where H: wasmtime::component::HasData {}
                pub trait Host {}
                impl<_T: Host + ?Sized> Host for &mut _T {}
                pub fn add_to_linker_instance<T, D>(
                    inst: &mut wasmtime::component::LinkerInstance<'_, T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    Ok(())
                }
                pub fn add_to_linker<T, D>(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    let mut inst = linker.instance("wasi:cli/types@0.3.0")?;
                    add_to_linker_instance::<T, D>(&mut inst, host_getter)
                }
            }
            #[allow(clippy::all)]
            pub mod stdin {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::Box;
                pub type ErrorCode = super::super::super::wasi::cli::types::ErrorCode;
                const _: () = {
                    assert!(1 == <ErrorCode as wasmtime::component::ComponentType>::SIZE32);
                    assert!(1 == <ErrorCode as wasmtime::component::ComponentType>::ALIGN32);
                };
                pub trait HostWithStore<T>: wasmtime::component::HasData {
                    #[doc = "/ Return a stream for reading from stdin."]
                    #[doc = "/ "]
                    #[doc = "/ This function returns a stream which provides data read from stdin,"]
                    #[doc = "/ and a future to signal read results."]
                    #[doc = "/ "]
                    #[doc = "/ If the stream\'s readable end is dropped the future will resolve to success."]
                    #[doc = "/ "]
                    #[doc = "/ If the stream\'s writable end is dropped the future will either resolve to"]
                    #[doc = "/ success if stdin was closed by the writer or to an error-code if reading"]
                    #[doc = "/ failed for some other reason."]
                    #[doc = "/ "]
                    #[doc = "/ Multiple streams may be active at the same time. The behavior of concurrent"]
                    #[doc = "/ reads is implementation-specific."]
                    fn read_via_stream(
                        host: wasmtime::component::Access<T, Self>,
                    ) -> wasmtime::Result<(
                        wasmtime::component::StreamReader<u8>,
                        wasmtime::component::FutureReader<Result<(), ErrorCode>>,
                    )>;
                }
                pub trait Host {}
                impl<_T: Host + ?Sized> Host for &mut _T {}
                pub fn add_to_linker_instance<T, D>(
                    inst: &mut wasmtime::component::LinkerInstance<'_, T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    inst.func_wrap(
                        "read-via-stream",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "stdin",
                                function = "read-via-stream",
                            );
                            let _enter = span.enter();
                            tracing::event!(tracing::Level::TRACE, "call");
                            let access_cx = wasmtime::AsContextMut::as_context_mut(&mut caller);
                            let host =
                                wasmtime::component::Access::<T, D>::new(access_cx, host_getter);
                            let r = <D as HostWithStore<T>>::read_via_stream(host);
                            tracing::event!(
                                tracing::Level::TRACE,
                                result = tracing::field::debug(&r),
                                "return"
                            );
                            Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                        },
                    )?;
                    Ok(())
                }
                pub fn add_to_linker<T, D>(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    let mut inst = linker.instance("wasi:cli/stdin@0.3.0")?;
                    add_to_linker_instance::<T, D>(&mut inst, host_getter)
                }
            }
            #[allow(clippy::all)]
            pub mod stdout {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::Box;
                pub type ErrorCode = super::super::super::wasi::cli::types::ErrorCode;
                const _: () = {
                    assert!(1 == <ErrorCode as wasmtime::component::ComponentType>::SIZE32);
                    assert!(1 == <ErrorCode as wasmtime::component::ComponentType>::ALIGN32);
                };
                pub trait HostWithStore<T>: wasmtime::component::HasData {
                    #[doc = "/ Write the given stream to stdout."]
                    #[doc = "/ "]
                    #[doc = "/ If the stream\'s writable end is dropped this function will either return"]
                    #[doc = "/ success once the entire contents of the stream have been written or an"]
                    #[doc = "/ error-code representing a failure."]
                    #[doc = "/ "]
                    #[doc = "/ Otherwise if there is an error the readable end of the stream will be"]
                    #[doc = "/ dropped and this function will return an error-code."]
                    fn write_via_stream(
                        host: wasmtime::component::Access<T, Self>,
                        data: wasmtime::component::StreamReader<u8>,
                    ) -> wasmtime::Result<wasmtime::component::FutureReader<Result<(), ErrorCode>>>;
                }
                pub trait Host {}
                impl<_T: Host + ?Sized> Host for &mut _T {}
                pub fn add_to_linker_instance<T, D>(
                    inst: &mut wasmtime::component::LinkerInstance<'_, T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    inst.func_wrap("write-via-stream", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (wasmtime::component::StreamReader<u8>,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "stdout", function = "write-via-stream",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, data = tracing::field::debug(&arg0), "call");
                        let access_cx = wasmtime::AsContextMut::as_context_mut(&mut caller);
                        let host = wasmtime::component::Access::<T, D>::new(access_cx, host_getter);
                        let r =  <D as HostWithStore<T>>::write_via_stream(host, arg0,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                    })?;
                    Ok(())
                }
                pub fn add_to_linker<T, D>(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    let mut inst = linker.instance("wasi:cli/stdout@0.3.0")?;
                    add_to_linker_instance::<T, D>(&mut inst, host_getter)
                }
            }
            #[allow(clippy::all)]
            pub mod stderr {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::Box;
                pub type ErrorCode = super::super::super::wasi::cli::types::ErrorCode;
                const _: () = {
                    assert!(1 == <ErrorCode as wasmtime::component::ComponentType>::SIZE32);
                    assert!(1 == <ErrorCode as wasmtime::component::ComponentType>::ALIGN32);
                };
                pub trait HostWithStore<T>: wasmtime::component::HasData {
                    #[doc = "/ Write the given stream to stderr."]
                    #[doc = "/ "]
                    #[doc = "/ If the stream\'s writable end is dropped this function will either return"]
                    #[doc = "/ success once the entire contents of the stream have been written or an"]
                    #[doc = "/ error-code representing a failure."]
                    #[doc = "/ "]
                    #[doc = "/ Otherwise if there is an error the readable end of the stream will be"]
                    #[doc = "/ dropped and this function will return an error-code."]
                    fn write_via_stream(
                        host: wasmtime::component::Access<T, Self>,
                        data: wasmtime::component::StreamReader<u8>,
                    ) -> wasmtime::Result<wasmtime::component::FutureReader<Result<(), ErrorCode>>>;
                }
                pub trait Host {}
                impl<_T: Host + ?Sized> Host for &mut _T {}
                pub fn add_to_linker_instance<T, D>(
                    inst: &mut wasmtime::component::LinkerInstance<'_, T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    inst.func_wrap("write-via-stream", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (wasmtime::component::StreamReader<u8>,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "stderr", function = "write-via-stream",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, data = tracing::field::debug(&arg0), "call");
                        let access_cx = wasmtime::AsContextMut::as_context_mut(&mut caller);
                        let host = wasmtime::component::Access::<T, D>::new(access_cx, host_getter);
                        let r =  <D as HostWithStore<T>>::write_via_stream(host, arg0,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                    })?;
                    Ok(())
                }
                pub fn add_to_linker<T, D>(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    let mut inst = linker.instance("wasi:cli/stderr@0.3.0")?;
                    add_to_linker_instance::<T, D>(&mut inst, host_getter)
                }
            }
            #[allow(clippy::all)]
            pub mod terminal_input {
                #[doc = "/ The input side of a terminal."]
                pub use super::super::super::__with_name0 as TerminalInput;
                #[allow(unused_imports)]
                use wasmtime::component::__internal::Box;
                pub trait HostTerminalInputWithStore<T>: wasmtime::component::HasData {}
                impl<H: ?Sized, T> HostTerminalInputWithStore<T> for H where H: wasmtime::component::HasData {}
                pub trait HostTerminalInput {
                    fn drop(
                        &mut self,
                        rep: wasmtime::component::Resource<TerminalInput>,
                    ) -> wasmtime::Result<()>;
                }
                impl<_T: HostTerminalInput + ?Sized> HostTerminalInput for &mut _T {
                    fn drop(
                        &mut self,
                        rep: wasmtime::component::Resource<TerminalInput>,
                    ) -> wasmtime::Result<()> {
                        HostTerminalInput::drop(*self, rep)
                    }
                }
                pub trait HostWithStore<T>:
                    wasmtime::component::HasData + HostTerminalInputWithStore<T>
                {
                }
                impl<H: ?Sized, T> HostWithStore<T> for H where
                    H: wasmtime::component::HasData + HostTerminalInputWithStore<T>
                {
                }
                pub trait Host: HostTerminalInput {}
                impl<_T: Host + ?Sized> Host for &mut _T {}
                pub fn add_to_linker_instance<T, D>(
                    inst: &mut wasmtime::component::LinkerInstance<'_, T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    inst.resource(
                        "terminal-input",
                        wasmtime::component::ResourceType::host::<TerminalInput>(),
                        move |mut store, rep| -> wasmtime::Result<()> {
                            let resource = wasmtime::component::Resource::new_own(rep);
                            wasmtime::ToWasmtimeResult::to_wasmtime_result(HostTerminalInput::drop(
                                &mut host_getter(store.data_mut()),
                                resource,
                            ))
                        },
                    )?;
                    Ok(())
                }
                pub fn add_to_linker<T, D>(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    let mut inst = linker.instance("wasi:cli/terminal-input@0.3.0")?;
                    add_to_linker_instance::<T, D>(&mut inst, host_getter)
                }
            }
            #[allow(clippy::all)]
            pub mod terminal_output {
                #[doc = "/ The output side of a terminal."]
                pub use super::super::super::__with_name1 as TerminalOutput;
                #[allow(unused_imports)]
                use wasmtime::component::__internal::Box;
                pub trait HostTerminalOutputWithStore<T>: wasmtime::component::HasData {}
                impl<H: ?Sized, T> HostTerminalOutputWithStore<T> for H where H: wasmtime::component::HasData {}
                pub trait HostTerminalOutput {
                    fn drop(
                        &mut self,
                        rep: wasmtime::component::Resource<TerminalOutput>,
                    ) -> wasmtime::Result<()>;
                }
                impl<_T: HostTerminalOutput + ?Sized> HostTerminalOutput for &mut _T {
                    fn drop(
                        &mut self,
                        rep: wasmtime::component::Resource<TerminalOutput>,
                    ) -> wasmtime::Result<()> {
                        HostTerminalOutput::drop(*self, rep)
                    }
                }
                pub trait HostWithStore<T>:
                    wasmtime::component::HasData + HostTerminalOutputWithStore<T>
                {
                }
                impl<H: ?Sized, T> HostWithStore<T> for H where
                    H: wasmtime::component::HasData + HostTerminalOutputWithStore<T>
                {
                }
                pub trait Host: HostTerminalOutput {}
                impl<_T: Host + ?Sized> Host for &mut _T {}
                pub fn add_to_linker_instance<T, D>(
                    inst: &mut wasmtime::component::LinkerInstance<'_, T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    inst.resource(
                        "terminal-output",
                        wasmtime::component::ResourceType::host::<TerminalOutput>(),
                        move |mut store, rep| -> wasmtime::Result<()> {
                            let resource = wasmtime::component::Resource::new_own(rep);
                            wasmtime::ToWasmtimeResult::to_wasmtime_result(
                                HostTerminalOutput::drop(
                                    &mut host_getter(store.data_mut()),
                                    resource,
                                ),
                            )
                        },
                    )?;
                    Ok(())
                }
                pub fn add_to_linker<T, D>(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    let mut inst = linker.instance("wasi:cli/terminal-output@0.3.0")?;
                    add_to_linker_instance::<T, D>(&mut inst, host_getter)
                }
            }
            #[allow(clippy::all)]
            pub mod terminal_stdin {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::Box;
                pub type TerminalInput =
                    super::super::super::wasi::cli::terminal_input::TerminalInput;
                pub trait HostWithStore<T>: wasmtime::component::HasData {}
                impl<H: ?Sized, T> HostWithStore<T> for H where H: wasmtime::component::HasData {}
                pub trait Host {
                    #[doc = "/ If stdin is connected to a terminal, return a `terminal-input` handle"]
                    #[doc = "/ allowing further interaction with it."]
                    fn get_terminal_stdin(
                        &mut self,
                    ) -> wasmtime::Result<Option<wasmtime::component::Resource<TerminalInput>>>;
                }
                impl<_T: Host + ?Sized> Host for &mut _T {
                    #[doc = "/ If stdin is connected to a terminal, return a `terminal-input` handle"]
                    #[doc = "/ allowing further interaction with it."]
                    fn get_terminal_stdin(
                        &mut self,
                    ) -> wasmtime::Result<Option<wasmtime::component::Resource<TerminalInput>>>
                    {
                        Host::get_terminal_stdin(*self)
                    }
                }
                pub fn add_to_linker_instance<T, D>(
                    inst: &mut wasmtime::component::LinkerInstance<'_, T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    inst.func_wrap(
                        "get-terminal-stdin",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "terminal-stdin",
                                function = "get-terminal-stdin",
                            );
                            let _enter = span.enter();
                            tracing::event!(tracing::Level::TRACE, "call");
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::get_terminal_stdin(host);
                            tracing::event!(
                                tracing::Level::TRACE,
                                result = tracing::field::debug(&r),
                                "return"
                            );
                            Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                        },
                    )?;
                    Ok(())
                }
                pub fn add_to_linker<T, D>(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    let mut inst = linker.instance("wasi:cli/terminal-stdin@0.3.0")?;
                    add_to_linker_instance::<T, D>(&mut inst, host_getter)
                }
            }
            #[allow(clippy::all)]
            pub mod terminal_stdout {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::Box;
                pub type TerminalOutput =
                    super::super::super::wasi::cli::terminal_output::TerminalOutput;
                pub trait HostWithStore<T>: wasmtime::component::HasData {}
                impl<H: ?Sized, T> HostWithStore<T> for H where H: wasmtime::component::HasData {}
                pub trait Host {
                    #[doc = "/ If stdout is connected to a terminal, return a `terminal-output` handle"]
                    #[doc = "/ allowing further interaction with it."]
                    fn get_terminal_stdout(
                        &mut self,
                    ) -> wasmtime::Result<Option<wasmtime::component::Resource<TerminalOutput>>>;
                }
                impl<_T: Host + ?Sized> Host for &mut _T {
                    #[doc = "/ If stdout is connected to a terminal, return a `terminal-output` handle"]
                    #[doc = "/ allowing further interaction with it."]
                    fn get_terminal_stdout(
                        &mut self,
                    ) -> wasmtime::Result<Option<wasmtime::component::Resource<TerminalOutput>>>
                    {
                        Host::get_terminal_stdout(*self)
                    }
                }
                pub fn add_to_linker_instance<T, D>(
                    inst: &mut wasmtime::component::LinkerInstance<'_, T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    inst.func_wrap(
                        "get-terminal-stdout",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "terminal-stdout",
                                function = "get-terminal-stdout",
                            );
                            let _enter = span.enter();
                            tracing::event!(tracing::Level::TRACE, "call");
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::get_terminal_stdout(host);
                            tracing::event!(
                                tracing::Level::TRACE,
                                result = tracing::field::debug(&r),
                                "return"
                            );
                            Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                        },
                    )?;
                    Ok(())
                }
                pub fn add_to_linker<T, D>(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    let mut inst = linker.instance("wasi:cli/terminal-stdout@0.3.0")?;
                    add_to_linker_instance::<T, D>(&mut inst, host_getter)
                }
            }
            #[allow(clippy::all)]
            pub mod terminal_stderr {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::Box;
                pub type TerminalOutput =
                    super::super::super::wasi::cli::terminal_output::TerminalOutput;
                pub trait HostWithStore<T>: wasmtime::component::HasData {}
                impl<H: ?Sized, T> HostWithStore<T> for H where H: wasmtime::component::HasData {}
                pub trait Host {
                    #[doc = "/ If stderr is connected to a terminal, return a `terminal-output` handle"]
                    #[doc = "/ allowing further interaction with it."]
                    fn get_terminal_stderr(
                        &mut self,
                    ) -> wasmtime::Result<Option<wasmtime::component::Resource<TerminalOutput>>>;
                }
                impl<_T: Host + ?Sized> Host for &mut _T {
                    #[doc = "/ If stderr is connected to a terminal, return a `terminal-output` handle"]
                    #[doc = "/ allowing further interaction with it."]
                    fn get_terminal_stderr(
                        &mut self,
                    ) -> wasmtime::Result<Option<wasmtime::component::Resource<TerminalOutput>>>
                    {
                        Host::get_terminal_stderr(*self)
                    }
                }
                pub fn add_to_linker_instance<T, D>(
                    inst: &mut wasmtime::component::LinkerInstance<'_, T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    inst.func_wrap(
                        "get-terminal-stderr",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "terminal-stderr",
                                function = "get-terminal-stderr",
                            );
                            let _enter = span.enter();
                            tracing::event!(tracing::Level::TRACE, "call");
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::get_terminal_stderr(host);
                            tracing::event!(
                                tracing::Level::TRACE,
                                result = tracing::field::debug(&r),
                                "return"
                            );
                            Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                        },
                    )?;
                    Ok(())
                }
                pub fn add_to_linker<T, D>(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    let mut inst = linker.instance("wasi:cli/terminal-stderr@0.3.0")?;
                    add_to_linker_instance::<T, D>(&mut inst, host_getter)
                }
            }
        }
        pub mod clocks {
            #[allow(clippy::all)]
            pub mod types {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::Box;
                #[doc = "/ A duration of time, in nanoseconds."]
                pub type Duration = u64;
                const _: () = {
                    assert!(8 == <Duration as wasmtime::component::ComponentType>::SIZE32);
                    assert!(8 == <Duration as wasmtime::component::ComponentType>::ALIGN32);
                };
                pub trait HostWithStore<T>: wasmtime::component::HasData {}
                impl<H: ?Sized, T> HostWithStore<T> for H where H: wasmtime::component::HasData {}
                pub trait Host {}
                impl<_T: Host + ?Sized> Host for &mut _T {}
                pub fn add_to_linker_instance<T, D>(
                    inst: &mut wasmtime::component::LinkerInstance<'_, T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    Ok(())
                }
                pub fn add_to_linker<T, D>(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    let mut inst = linker.instance("wasi:clocks/types@0.3.0")?;
                    add_to_linker_instance::<T, D>(&mut inst, host_getter)
                }
            }
            #[allow(clippy::all)]
            pub mod monotonic_clock {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::Box;
                pub type Duration = super::super::super::wasi::clocks::types::Duration;
                const _: () = {
                    assert!(8 == <Duration as wasmtime::component::ComponentType>::SIZE32);
                    assert!(8 == <Duration as wasmtime::component::ComponentType>::ALIGN32);
                };
                #[doc = "/ A mark on a monotonic clock is a number of nanoseconds since an"]
                #[doc = "/ unspecified initial value, and can only be compared to instances from"]
                #[doc = "/ the same monotonic-clock."]
                pub type Mark = u64;
                const _: () = {
                    assert!(8 == <Mark as wasmtime::component::ComponentType>::SIZE32);
                    assert!(8 == <Mark as wasmtime::component::ComponentType>::ALIGN32);
                };
                pub trait HostWithStore<T>: wasmtime::component::HasData + Send {
                    #[doc = "/ Wait until the specified mark has occurred."]
                    fn wait_until(
                        accessor: &wasmtime::component::Accessor<T, Self>,
                        when: Mark,
                    ) -> impl ::core::future::Future<Output = wasmtime::Result<()>> + Send;

                    #[doc = "/ Wait for the specified duration to elapse."]
                    fn wait_for(
                        accessor: &wasmtime::component::Accessor<T, Self>,
                        how_long: Duration,
                    ) -> impl ::core::future::Future<Output = wasmtime::Result<()>> + Send;
                }
                pub trait Host: Send {
                    #[doc = "/ Read the current value of the clock."]
                    #[doc = "/ "]
                    #[doc = "/ The clock is monotonic, therefore calling this function repeatedly will"]
                    #[doc = "/ produce a sequence of non-decreasing values."]
                    #[doc = "/ "]
                    #[doc = "/ For completeness, this function traps if it\'s not possible to represent"]
                    #[doc = "/ the value of the clock in a `mark`. Consequently, implementations"]
                    #[doc = "/ should ensure that the starting time is low enough to avoid the"]
                    #[doc = "/ possibility of overflow in practice."]
                    fn now(&mut self) -> wasmtime::Result<Mark>;

                    #[doc = "/ Query the resolution of the clock. Returns the duration of time"]
                    #[doc = "/ corresponding to a clock tick."]
                    fn get_resolution(&mut self) -> wasmtime::Result<Duration>;
                }
                impl<_T: Host + ?Sized + Send> Host for &mut _T {
                    #[doc = "/ Read the current value of the clock."]
                    #[doc = "/ "]
                    #[doc = "/ The clock is monotonic, therefore calling this function repeatedly will"]
                    #[doc = "/ produce a sequence of non-decreasing values."]
                    #[doc = "/ "]
                    #[doc = "/ For completeness, this function traps if it\'s not possible to represent"]
                    #[doc = "/ the value of the clock in a `mark`. Consequently, implementations"]
                    #[doc = "/ should ensure that the starting time is low enough to avoid the"]
                    #[doc = "/ possibility of overflow in practice."]
                    fn now(&mut self) -> wasmtime::Result<Mark> {
                        Host::now(*self)
                    }
                    #[doc = "/ Query the resolution of the clock. Returns the duration of time"]
                    #[doc = "/ corresponding to a clock tick."]
                    fn get_resolution(&mut self) -> wasmtime::Result<Duration> {
                        Host::get_resolution(*self)
                    }
                }
                pub fn add_to_linker_instance<T, D>(
                    inst: &mut wasmtime::component::LinkerInstance<'_, T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static + Send,
                {
                    inst.func_wrap(
                        "now",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "monotonic-clock",
                                function = "now",
                            );
                            let _enter = span.enter();
                            tracing::event!(tracing::Level::TRACE, "call");
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::now(host);
                            tracing::event!(
                                tracing::Level::TRACE,
                                result = tracing::field::debug(&r),
                                "return"
                            );
                            Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                        },
                    )?;
                    inst.func_wrap(
                        "get-resolution",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "monotonic-clock",
                                function = "get-resolution",
                            );
                            let _enter = span.enter();
                            tracing::event!(tracing::Level::TRACE, "call");
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::get_resolution(host);
                            tracing::event!(
                                tracing::Level::TRACE,
                                result = tracing::field::debug(&r),
                                "return"
                            );
                            Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                        },
                    )?;
                    inst.func_wrap_concurrent(
                        "wait-until",
                        move |caller: &wasmtime::component::Accessor<T>, (arg0,): (Mark,)| {
                            use tracing::Instrument;
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "monotonic-clock",
                                function = "wait-until",
                            );
                            wasmtime::component::__internal::Box::pin(
                                async move {
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        when = tracing::field::debug(&arg0),
                                        "call"
                                    );
                                    let host = &caller.with_getter::<D>(host_getter);
                                    let r = <D as HostWithStore<T>>::wait_until(host, arg0).await;
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        result = tracing::field::debug(&r),
                                        "return"
                                    );
                                    wasmtime::ToWasmtimeResult::to_wasmtime_result(r)
                                }
                                .instrument(span),
                            )
                        },
                    )?;
                    inst.func_wrap_concurrent(
                        "wait-for",
                        move |caller: &wasmtime::component::Accessor<T>, (arg0,): (Duration,)| {
                            use tracing::Instrument;
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "monotonic-clock",
                                function = "wait-for",
                            );
                            wasmtime::component::__internal::Box::pin(
                                async move {
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        how_long = tracing::field::debug(&arg0),
                                        "call"
                                    );
                                    let host = &caller.with_getter::<D>(host_getter);
                                    let r = <D as HostWithStore<T>>::wait_for(host, arg0).await;
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        result = tracing::field::debug(&r),
                                        "return"
                                    );
                                    wasmtime::ToWasmtimeResult::to_wasmtime_result(r)
                                }
                                .instrument(span),
                            )
                        },
                    )?;
                    Ok(())
                }
                pub fn add_to_linker<T, D>(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static + Send,
                {
                    let mut inst = linker.instance("wasi:clocks/monotonic-clock@0.3.0")?;
                    add_to_linker_instance::<T, D>(&mut inst, host_getter)
                }
            }
            #[allow(clippy::all)]
            pub mod system_clock {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::Box;
                pub type Duration = super::super::super::wasi::clocks::types::Duration;
                const _: () = {
                    assert!(8 == <Duration as wasmtime::component::ComponentType>::SIZE32);
                    assert!(8 == <Duration as wasmtime::component::ComponentType>::ALIGN32);
                };
                #[doc = "/ An \"instant\", or \"exact time\", is a point in time without regard to any"]
                #[doc = "/ time zone: just the time since a particular external reference point,"]
                #[doc = "/ often called an \"epoch\"."]
                #[doc = "/ "]
                #[doc = "/ Here, the epoch is 1970-01-01T00:00:00Z, also known as"]
                #[doc = "/ [POSIX\'s Seconds Since the Epoch], also known as [Unix Time]."]
                #[doc = "/ "]
                #[doc = "/ Note that even if the seconds field is negative, incrementing"]
                #[doc = "/ nanoseconds always represents moving forwards in time."]
                #[doc = "/ For example, `{ -1 seconds, 999999999 nanoseconds }` represents the"]
                #[doc = "/ instant one nanosecond before the epoch."]
                #[doc = "/ For more on various different ways to represent time, see"]
                #[doc = "/ https://tc39.es/proposal-temporal/docs/timezone.html"]
                #[doc = "/ "]
                #[doc = "/ [POSIX\'s Seconds Since the Epoch]: https://pubs.opengroup.org/onlinepubs/9699919799/xrat/V4_xbd_chap04.html#tag_21_04_16"]
                #[doc = "/ [Unix Time]: https://en.wikipedia.org/wiki/Unix_time"]
                #[derive(
                    wasmtime::component::ComponentType,
                    wasmtime::component::Lift,
                    wasmtime::component::Lower,
                )]
                #[component(record)]
                #[derive(Clone, Copy)]
                pub struct Instant {
                    #[component(name = "seconds")]
                    pub seconds: i64,
                    #[component(name = "nanoseconds")]
                    pub nanoseconds: u32,
                }
                impl core::fmt::Debug for Instant {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.debug_struct("Instant")
                            .field("seconds", &self.seconds)
                            .field("nanoseconds", &self.nanoseconds)
                            .finish()
                    }
                }
                const _: () = {
                    assert!(16 == <Instant as wasmtime::component::ComponentType>::SIZE32);
                    assert!(8 == <Instant as wasmtime::component::ComponentType>::ALIGN32);
                };
                pub trait HostWithStore<T>: wasmtime::component::HasData {}
                impl<H: ?Sized, T> HostWithStore<T> for H where H: wasmtime::component::HasData {}
                pub trait Host {
                    #[doc = "/ Read the current value of the clock."]
                    #[doc = "/ "]
                    #[doc = "/ This clock is not monotonic, therefore calling this function repeatedly"]
                    #[doc = "/ will not necessarily produce a sequence of non-decreasing values."]
                    #[doc = "/ "]
                    #[doc = "/ The nanoseconds field of the output is always less than 1000000000."]
                    fn now(&mut self) -> wasmtime::Result<Instant>;

                    #[doc = "/ Query the resolution of the clock. Returns the smallest duration of time"]
                    #[doc = "/ that the implementation permits distinguishing."]
                    fn get_resolution(&mut self) -> wasmtime::Result<Duration>;
                }
                impl<_T: Host + ?Sized> Host for &mut _T {
                    #[doc = "/ Read the current value of the clock."]
                    #[doc = "/ "]
                    #[doc = "/ This clock is not monotonic, therefore calling this function repeatedly"]
                    #[doc = "/ will not necessarily produce a sequence of non-decreasing values."]
                    #[doc = "/ "]
                    #[doc = "/ The nanoseconds field of the output is always less than 1000000000."]
                    fn now(&mut self) -> wasmtime::Result<Instant> {
                        Host::now(*self)
                    }
                    #[doc = "/ Query the resolution of the clock. Returns the smallest duration of time"]
                    #[doc = "/ that the implementation permits distinguishing."]
                    fn get_resolution(&mut self) -> wasmtime::Result<Duration> {
                        Host::get_resolution(*self)
                    }
                }
                pub fn add_to_linker_instance<T, D>(
                    inst: &mut wasmtime::component::LinkerInstance<'_, T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    inst.func_wrap(
                        "now",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "system-clock",
                                function = "now",
                            );
                            let _enter = span.enter();
                            tracing::event!(tracing::Level::TRACE, "call");
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::now(host);
                            tracing::event!(
                                tracing::Level::TRACE,
                                result = tracing::field::debug(&r),
                                "return"
                            );
                            Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                        },
                    )?;
                    inst.func_wrap(
                        "get-resolution",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "system-clock",
                                function = "get-resolution",
                            );
                            let _enter = span.enter();
                            tracing::event!(tracing::Level::TRACE, "call");
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::get_resolution(host);
                            tracing::event!(
                                tracing::Level::TRACE,
                                result = tracing::field::debug(&r),
                                "return"
                            );
                            Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                        },
                    )?;
                    Ok(())
                }
                pub fn add_to_linker<T, D>(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    let mut inst = linker.instance("wasi:clocks/system-clock@0.3.0")?;
                    add_to_linker_instance::<T, D>(&mut inst, host_getter)
                }
            }
            #[allow(clippy::all)]
            pub mod timezone {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::Box;
                #[doc = "/ Link-time configurations."]
                #[derive(Clone, Debug, Default)]
                pub struct LinkOptions {
                    clocks_timezone: bool,
                }
                impl LinkOptions {
                    #[doc = "/ Enable members marked as `@unstable(feature = clocks-timezone)`"]
                    pub fn clocks_timezone(&mut self, enabled: bool) -> &mut Self {
                        self.clocks_timezone = enabled;
                        self
                    }
                }
                pub type Instant = super::super::super::wasi::clocks::system_clock::Instant;
                const _: () = {
                    assert!(16 == <Instant as wasmtime::component::ComponentType>::SIZE32);
                    assert!(8 == <Instant as wasmtime::component::ComponentType>::ALIGN32);
                };
                pub trait HostWithStore<T>: wasmtime::component::HasData {}
                impl<H: ?Sized, T> HostWithStore<T> for H where H: wasmtime::component::HasData {}
                pub trait Host {
                    #[doc = "/ Return the IANA identifier of the currently configured timezone. This"]
                    #[doc = "/ should be an identifier from the IANA Time Zone Database."]
                    #[doc = "/ "]
                    #[doc = "/ For displaying to a user, the identifier should be converted into a"]
                    #[doc = "/ localized name by means of an internationalization API."]
                    #[doc = "/ "]
                    #[doc = "/ If the implementation does not expose an actual timezone, or is unable"]
                    #[doc = "/ to provide mappings from times to deltas between the configured timezone"]
                    #[doc = "/ and UTC, or determining the current timezone fails, or the timezone does"]
                    #[doc = "/ not have an IANA identifier, this returns nothing."]
                    fn iana_id(
                        &mut self,
                    ) -> wasmtime::Result<Option<wasmtime::component::__internal::String>>;

                    #[doc = "/ The number of nanoseconds difference between UTC time and the local"]
                    #[doc = "/ time of the currently configured timezone, at the exact time of"]
                    #[doc = "/ `instant`."]
                    #[doc = "/ "]
                    #[doc = "/ The magnitude of the returned value will always be less than"]
                    #[doc = "/ 86,400,000,000,000 which is the number of nanoseconds in a day"]
                    #[doc = "/ (24*60*60*1e9)."]
                    #[doc = "/ "]
                    #[doc = "/ If the implementation does not expose an actual timezone, or is unable"]
                    #[doc = "/ to provide mappings from times to deltas between the configured timezone"]
                    #[doc = "/ and UTC, or determining the current timezone fails, this returns"]
                    #[doc = "/ nothing."]
                    fn utc_offset(&mut self, when: Instant) -> wasmtime::Result<Option<i64>>;

                    #[doc = "/ Returns a string that is suitable to assist humans in debugging whether"]
                    #[doc = "/ any timezone is available, and if so, which. This may be the same string"]
                    #[doc = "/ as `iana-id`, or a formatted representation of the UTC offset such as"]
                    #[doc = "/ `-04:00`, or something else."]
                    #[doc = "/ "]
                    #[doc = "/ WARNING: The returned string should not be consumed mechanically! It may"]
                    #[doc = "/ change across platforms, hosts, or other implementation details. Parsing"]
                    #[doc = "/ this string is a major platform-compatibility hazard."]
                    fn to_debug_string(
                        &mut self,
                    ) -> wasmtime::Result<wasmtime::component::__internal::String>;
                }
                impl<_T: Host + ?Sized> Host for &mut _T {
                    #[doc = "/ Return the IANA identifier of the currently configured timezone. This"]
                    #[doc = "/ should be an identifier from the IANA Time Zone Database."]
                    #[doc = "/ "]
                    #[doc = "/ For displaying to a user, the identifier should be converted into a"]
                    #[doc = "/ localized name by means of an internationalization API."]
                    #[doc = "/ "]
                    #[doc = "/ If the implementation does not expose an actual timezone, or is unable"]
                    #[doc = "/ to provide mappings from times to deltas between the configured timezone"]
                    #[doc = "/ and UTC, or determining the current timezone fails, or the timezone does"]
                    #[doc = "/ not have an IANA identifier, this returns nothing."]
                    fn iana_id(
                        &mut self,
                    ) -> wasmtime::Result<Option<wasmtime::component::__internal::String>>
                    {
                        Host::iana_id(*self)
                    }
                    #[doc = "/ The number of nanoseconds difference between UTC time and the local"]
                    #[doc = "/ time of the currently configured timezone, at the exact time of"]
                    #[doc = "/ `instant`."]
                    #[doc = "/ "]
                    #[doc = "/ The magnitude of the returned value will always be less than"]
                    #[doc = "/ 86,400,000,000,000 which is the number of nanoseconds in a day"]
                    #[doc = "/ (24*60*60*1e9)."]
                    #[doc = "/ "]
                    #[doc = "/ If the implementation does not expose an actual timezone, or is unable"]
                    #[doc = "/ to provide mappings from times to deltas between the configured timezone"]
                    #[doc = "/ and UTC, or determining the current timezone fails, this returns"]
                    #[doc = "/ nothing."]
                    fn utc_offset(&mut self, when: Instant) -> wasmtime::Result<Option<i64>> {
                        Host::utc_offset(*self, when)
                    }
                    #[doc = "/ Returns a string that is suitable to assist humans in debugging whether"]
                    #[doc = "/ any timezone is available, and if so, which. This may be the same string"]
                    #[doc = "/ as `iana-id`, or a formatted representation of the UTC offset such as"]
                    #[doc = "/ `-04:00`, or something else."]
                    #[doc = "/ "]
                    #[doc = "/ WARNING: The returned string should not be consumed mechanically! It may"]
                    #[doc = "/ change across platforms, hosts, or other implementation details. Parsing"]
                    #[doc = "/ this string is a major platform-compatibility hazard."]
                    fn to_debug_string(
                        &mut self,
                    ) -> wasmtime::Result<wasmtime::component::__internal::String>
                    {
                        Host::to_debug_string(*self)
                    }
                }
                pub fn add_to_linker_instance<T, D>(
                    inst: &mut wasmtime::component::LinkerInstance<'_, T>,
                    options: &LinkOptions,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    if options.clocks_timezone {
                        if options.clocks_timezone {
                            inst.func_wrap(
                                "iana-id",
                                move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                                    let span = tracing::span!(
                                        tracing::Level::TRACE,
                                        "wit-bindgen import",
                                        module = "timezone",
                                        function = "iana-id",
                                    );
                                    let _enter = span.enter();
                                    tracing::event!(tracing::Level::TRACE, "call");
                                    let host = &mut host_getter(caller.data_mut());
                                    let r = Host::iana_id(host);
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        result = tracing::field::debug(&r),
                                        "return"
                                    );
                                    Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                                },
                            )?;
                        }
                        if options.clocks_timezone {
                            inst.func_wrap(
                                "utc-offset",
                                move |mut caller: wasmtime::StoreContextMut<'_, T>,
                                      (arg0,): (Instant,)| {
                                    let span = tracing::span!(
                                        tracing::Level::TRACE,
                                        "wit-bindgen import",
                                        module = "timezone",
                                        function = "utc-offset",
                                    );
                                    let _enter = span.enter();
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        when = tracing::field::debug(&arg0),
                                        "call"
                                    );
                                    let host = &mut host_getter(caller.data_mut());
                                    let r = Host::utc_offset(host, arg0);
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        result = tracing::field::debug(&r),
                                        "return"
                                    );
                                    Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                                },
                            )?;
                        }
                        if options.clocks_timezone {
                            inst.func_wrap(
                                "to-debug-string",
                                move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                                    let span = tracing::span!(
                                        tracing::Level::TRACE,
                                        "wit-bindgen import",
                                        module = "timezone",
                                        function = "to-debug-string",
                                    );
                                    let _enter = span.enter();
                                    tracing::event!(tracing::Level::TRACE, "call");
                                    let host = &mut host_getter(caller.data_mut());
                                    let r = Host::to_debug_string(host);
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        result = tracing::field::debug(&r),
                                        "return"
                                    );
                                    Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                                },
                            )?;
                        }
                    }
                    Ok(())
                }
                pub fn add_to_linker<T, D>(
                    linker: &mut wasmtime::component::Linker<T>,
                    options: &LinkOptions,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    let mut inst = linker.instance("wasi:clocks/timezone@0.3.0")?;
                    add_to_linker_instance::<T, D>(&mut inst, options, host_getter)
                }
            }
        }
        pub mod filesystem {
            #[allow(clippy::all)]
            pub mod types {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::Box;
                pub type Instant = super::super::super::wasi::clocks::system_clock::Instant;
                const _: () = {
                    assert!(16 == <Instant as wasmtime::component::ComponentType>::SIZE32);
                    assert!(8 == <Instant as wasmtime::component::ComponentType>::ALIGN32);
                };
                #[doc = "/ File size or length of a region within a file."]
                pub type Filesize = u64;
                const _: () = {
                    assert!(8 == <Filesize as wasmtime::component::ComponentType>::SIZE32);
                    assert!(8 == <Filesize as wasmtime::component::ComponentType>::ALIGN32);
                };
                #[doc = "/ The type of a filesystem object referenced by a descriptor."]
                #[doc = "/ "]
                #[doc = "/ Note: This was called `filetype` in earlier versions of WASI."]
                #[derive(
                    wasmtime::component::ComponentType,
                    wasmtime::component::Lift,
                    wasmtime::component::Lower,
                )]
                #[component(variant)]
                #[derive(Clone)]
                pub enum DescriptorType {
                    #[doc = "/ The descriptor refers to a block device inode."]
                    #[component(name = "block-device")]
                    BlockDevice,
                    #[doc = "/ The descriptor refers to a character device inode."]
                    #[component(name = "character-device")]
                    CharacterDevice,
                    #[doc = "/ The descriptor refers to a directory inode."]
                    #[component(name = "directory")]
                    Directory,
                    #[doc = "/ The descriptor refers to a named pipe."]
                    #[component(name = "fifo")]
                    Fifo,
                    #[doc = "/ The file refers to a symbolic link inode."]
                    #[component(name = "symbolic-link")]
                    SymbolicLink,
                    #[doc = "/ The descriptor refers to a regular file inode."]
                    #[component(name = "regular-file")]
                    RegularFile,
                    #[doc = "/ The descriptor refers to a socket."]
                    #[component(name = "socket")]
                    Socket,
                    #[doc = "/ The type of the descriptor or file is different from any of the"]
                    #[doc = "/ other types specified."]
                    #[component(name = "other")]
                    Other(Option<wasmtime::component::__internal::String>),
                }
                impl core::fmt::Debug for DescriptorType {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        match self {
                            DescriptorType::BlockDevice => {
                                f.debug_tuple("DescriptorType::BlockDevice").finish()
                            }
                            DescriptorType::CharacterDevice => {
                                f.debug_tuple("DescriptorType::CharacterDevice").finish()
                            }
                            DescriptorType::Directory => {
                                f.debug_tuple("DescriptorType::Directory").finish()
                            }
                            DescriptorType::Fifo => f.debug_tuple("DescriptorType::Fifo").finish(),
                            DescriptorType::SymbolicLink => {
                                f.debug_tuple("DescriptorType::SymbolicLink").finish()
                            }
                            DescriptorType::RegularFile => {
                                f.debug_tuple("DescriptorType::RegularFile").finish()
                            }
                            DescriptorType::Socket => {
                                f.debug_tuple("DescriptorType::Socket").finish()
                            }
                            DescriptorType::Other(e) => {
                                f.debug_tuple("DescriptorType::Other").field(e).finish()
                            }
                        }
                    }
                }
                const _: () = {
                    assert!(16 == <DescriptorType as wasmtime::component::ComponentType>::SIZE32);
                    assert!(4 == <DescriptorType as wasmtime::component::ComponentType>::ALIGN32);
                };
                #[doc = "/ Descriptor flags."]
                #[doc = "/ "]
                #[doc = "/ Note: This was called `fdflags` in earlier versions of WASI."]
                wasmtime::component::flags!(DescriptorFlags {
                    #[component(name = "read")]const READ;
                    #[component(name = "write")]const WRITE;
                    #[component(name = "file-integrity-sync")]const FILE_INTEGRITY_SYNC;
                    #[component(name = "data-integrity-sync")]const DATA_INTEGRITY_SYNC;
                    #[component(name = "requested-write-sync")]const REQUESTED_WRITE_SYNC;
                    #[component(name = "mutate-directory")]const MUTATE_DIRECTORY;
                });
                const _: () = {
                    assert!(1 == <DescriptorFlags as wasmtime::component::ComponentType>::SIZE32);
                    assert!(1 == <DescriptorFlags as wasmtime::component::ComponentType>::ALIGN32);
                };
                #[doc = "/ Flags determining the method of how paths are resolved."]
                wasmtime::component::flags!(PathFlags {
                    #[component(name = "symlink-follow")]const SYMLINK_FOLLOW;
                });
                const _: () = {
                    assert!(1 == <PathFlags as wasmtime::component::ComponentType>::SIZE32);
                    assert!(1 == <PathFlags as wasmtime::component::ComponentType>::ALIGN32);
                };
                #[doc = "/ Open flags used by `open-at`."]
                wasmtime::component::flags!(OpenFlags {
                    #[component(name = "create")]const CREATE;
                    #[component(name = "directory")]const DIRECTORY;
                    #[component(name = "exclusive")]const EXCLUSIVE;
                    #[component(name = "truncate")]const TRUNCATE;
                });
                const _: () = {
                    assert!(1 == <OpenFlags as wasmtime::component::ComponentType>::SIZE32);
                    assert!(1 == <OpenFlags as wasmtime::component::ComponentType>::ALIGN32);
                };
                #[doc = "/ Number of hard links to an inode."]
                pub type LinkCount = u64;
                const _: () = {
                    assert!(8 == <LinkCount as wasmtime::component::ComponentType>::SIZE32);
                    assert!(8 == <LinkCount as wasmtime::component::ComponentType>::ALIGN32);
                };
                #[doc = "/ File attributes."]
                #[doc = "/ "]
                #[doc = "/ Note: This was called `filestat` in earlier versions of WASI."]
                #[derive(
                    wasmtime::component::ComponentType,
                    wasmtime::component::Lift,
                    wasmtime::component::Lower,
                )]
                #[component(record)]
                #[derive(Clone)]
                pub struct DescriptorStat {
                    #[doc = "/ File type."]
                    #[component(name = "type")]
                    pub type_: DescriptorType,
                    #[doc = "/ Number of hard links to the file."]
                    #[component(name = "link-count")]
                    pub link_count: LinkCount,
                    #[doc = "/ For regular files, the file size in bytes. For symbolic links, the"]
                    #[doc = "/ length in bytes of the pathname contained in the symbolic link."]
                    #[component(name = "size")]
                    pub size: Filesize,
                    #[doc = "/ Last data access timestamp."]
                    #[doc = "/ "]
                    #[doc = "/ If the `option` is none, the platform doesn\'t maintain an access"]
                    #[doc = "/ timestamp for this file."]
                    #[component(name = "data-access-timestamp")]
                    pub data_access_timestamp: Option<Instant>,
                    #[doc = "/ Last data modification timestamp."]
                    #[doc = "/ "]
                    #[doc = "/ If the `option` is none, the platform doesn\'t maintain a"]
                    #[doc = "/ modification timestamp for this file."]
                    #[component(name = "data-modification-timestamp")]
                    pub data_modification_timestamp: Option<Instant>,
                    #[doc = "/ Last file status-change timestamp."]
                    #[doc = "/ "]
                    #[doc = "/ If the `option` is none, the platform doesn\'t maintain a"]
                    #[doc = "/ status-change timestamp for this file."]
                    #[component(name = "status-change-timestamp")]
                    pub status_change_timestamp: Option<Instant>,
                }
                impl core::fmt::Debug for DescriptorStat {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.debug_struct("DescriptorStat")
                            .field("type", &self.type_)
                            .field("link-count", &self.link_count)
                            .field("size", &self.size)
                            .field("data-access-timestamp", &self.data_access_timestamp)
                            .field(
                                "data-modification-timestamp",
                                &self.data_modification_timestamp,
                            )
                            .field("status-change-timestamp", &self.status_change_timestamp)
                            .finish()
                    }
                }
                const _: () = {
                    assert!(104 == <DescriptorStat as wasmtime::component::ComponentType>::SIZE32);
                    assert!(8 == <DescriptorStat as wasmtime::component::ComponentType>::ALIGN32);
                };
                #[doc = "/ When setting a timestamp, this gives the value to set it to."]
                #[derive(
                    wasmtime::component::ComponentType,
                    wasmtime::component::Lift,
                    wasmtime::component::Lower,
                )]
                #[component(variant)]
                #[derive(Clone, Copy)]
                pub enum NewTimestamp {
                    #[doc = "/ Leave the timestamp set to its previous value."]
                    #[component(name = "no-change")]
                    NoChange,
                    #[doc = "/ Set the timestamp to the current time of the system clock associated"]
                    #[doc = "/ with the filesystem."]
                    #[component(name = "now")]
                    Now,
                    #[doc = "/ Set the timestamp to the given value."]
                    #[component(name = "timestamp")]
                    Timestamp(Instant),
                }
                impl core::fmt::Debug for NewTimestamp {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        match self {
                            NewTimestamp::NoChange => {
                                f.debug_tuple("NewTimestamp::NoChange").finish()
                            }
                            NewTimestamp::Now => f.debug_tuple("NewTimestamp::Now").finish(),
                            NewTimestamp::Timestamp(e) => {
                                f.debug_tuple("NewTimestamp::Timestamp").field(e).finish()
                            }
                        }
                    }
                }
                const _: () = {
                    assert!(24 == <NewTimestamp as wasmtime::component::ComponentType>::SIZE32);
                    assert!(8 == <NewTimestamp as wasmtime::component::ComponentType>::ALIGN32);
                };
                #[doc = "/ A directory entry."]
                #[derive(
                    wasmtime::component::ComponentType,
                    wasmtime::component::Lift,
                    wasmtime::component::Lower,
                )]
                #[component(record)]
                #[derive(Clone)]
                pub struct DirectoryEntry {
                    #[doc = "/ The type of the file referred to by this directory entry."]
                    #[component(name = "type")]
                    pub type_: DescriptorType,
                    #[doc = "/ The name of the object."]
                    #[component(name = "name")]
                    pub name: wasmtime::component::__internal::String,
                }
                impl core::fmt::Debug for DirectoryEntry {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.debug_struct("DirectoryEntry")
                            .field("type", &self.type_)
                            .field("name", &self.name)
                            .finish()
                    }
                }
                const _: () = {
                    assert!(24 == <DirectoryEntry as wasmtime::component::ComponentType>::SIZE32);
                    assert!(4 == <DirectoryEntry as wasmtime::component::ComponentType>::ALIGN32);
                };
                #[doc = "/ Error codes returned by functions, similar to `errno` in POSIX."]
                #[doc = "/ Not all of these error codes are returned by the functions provided by this"]
                #[doc = "/ API; some are used in higher-level library layers, and others are provided"]
                #[doc = "/ merely for alignment with POSIX."]
                #[derive(
                    wasmtime::component::ComponentType,
                    wasmtime::component::Lift,
                    wasmtime::component::Lower,
                )]
                #[component(variant)]
                #[derive(Clone)]
                pub enum ErrorCode {
                    #[doc = "/ Permission denied, similar to `EACCES` in POSIX."]
                    #[component(name = "access")]
                    Access,
                    #[doc = "/ Connection already in progress, similar to `EALREADY` in POSIX."]
                    #[component(name = "already")]
                    Already,
                    #[doc = "/ Bad descriptor, similar to `EBADF` in POSIX."]
                    #[component(name = "bad-descriptor")]
                    BadDescriptor,
                    #[doc = "/ Device or resource busy, similar to `EBUSY` in POSIX."]
                    #[component(name = "busy")]
                    Busy,
                    #[doc = "/ Resource deadlock would occur, similar to `EDEADLK` in POSIX."]
                    #[component(name = "deadlock")]
                    Deadlock,
                    #[doc = "/ Storage quota exceeded, similar to `EDQUOT` in POSIX."]
                    #[component(name = "quota")]
                    Quota,
                    #[doc = "/ File exists, similar to `EEXIST` in POSIX."]
                    #[component(name = "exist")]
                    Exist,
                    #[doc = "/ File too large, similar to `EFBIG` in POSIX."]
                    #[component(name = "file-too-large")]
                    FileTooLarge,
                    #[doc = "/ Illegal byte sequence, similar to `EILSEQ` in POSIX."]
                    #[component(name = "illegal-byte-sequence")]
                    IllegalByteSequence,
                    #[doc = "/ Operation in progress, similar to `EINPROGRESS` in POSIX."]
                    #[component(name = "in-progress")]
                    InProgress,
                    #[doc = "/ Interrupted function, similar to `EINTR` in POSIX."]
                    #[component(name = "interrupted")]
                    Interrupted,
                    #[doc = "/ Invalid argument, similar to `EINVAL` in POSIX."]
                    #[component(name = "invalid")]
                    Invalid,
                    #[doc = "/ I/O error, similar to `EIO` in POSIX."]
                    #[component(name = "io")]
                    Io,
                    #[doc = "/ Is a directory, similar to `EISDIR` in POSIX."]
                    #[component(name = "is-directory")]
                    IsDirectory,
                    #[doc = "/ Too many levels of symbolic links, similar to `ELOOP` in POSIX."]
                    #[component(name = "loop")]
                    Loop,
                    #[doc = "/ Too many links, similar to `EMLINK` in POSIX."]
                    #[component(name = "too-many-links")]
                    TooManyLinks,
                    #[doc = "/ Message too large, similar to `EMSGSIZE` in POSIX."]
                    #[component(name = "message-size")]
                    MessageSize,
                    #[doc = "/ Filename too long, similar to `ENAMETOOLONG` in POSIX."]
                    #[component(name = "name-too-long")]
                    NameTooLong,
                    #[doc = "/ No such device, similar to `ENODEV` in POSIX."]
                    #[component(name = "no-device")]
                    NoDevice,
                    #[doc = "/ No such file or directory, similar to `ENOENT` in POSIX."]
                    #[component(name = "no-entry")]
                    NoEntry,
                    #[doc = "/ No locks available, similar to `ENOLCK` in POSIX."]
                    #[component(name = "no-lock")]
                    NoLock,
                    #[doc = "/ Not enough space, similar to `ENOMEM` in POSIX."]
                    #[component(name = "insufficient-memory")]
                    InsufficientMemory,
                    #[doc = "/ No space left on device, similar to `ENOSPC` in POSIX."]
                    #[component(name = "insufficient-space")]
                    InsufficientSpace,
                    #[doc = "/ Not a directory or a symbolic link to a directory, similar to `ENOTDIR` in POSIX."]
                    #[component(name = "not-directory")]
                    NotDirectory,
                    #[doc = "/ Directory not empty, similar to `ENOTEMPTY` in POSIX."]
                    #[component(name = "not-empty")]
                    NotEmpty,
                    #[doc = "/ State not recoverable, similar to `ENOTRECOVERABLE` in POSIX."]
                    #[component(name = "not-recoverable")]
                    NotRecoverable,
                    #[doc = "/ Not supported, similar to `ENOTSUP` and `ENOSYS` in POSIX."]
                    #[component(name = "unsupported")]
                    Unsupported,
                    #[doc = "/ Inappropriate I/O control operation, similar to `ENOTTY` in POSIX."]
                    #[component(name = "no-tty")]
                    NoTty,
                    #[doc = "/ No such device or address, similar to `ENXIO` in POSIX."]
                    #[component(name = "no-such-device")]
                    NoSuchDevice,
                    #[doc = "/ Value too large to be stored in data type, similar to `EOVERFLOW` in POSIX."]
                    #[component(name = "overflow")]
                    Overflow,
                    #[doc = "/ Operation not permitted, similar to `EPERM` in POSIX."]
                    #[component(name = "not-permitted")]
                    NotPermitted,
                    #[doc = "/ Broken pipe, similar to `EPIPE` in POSIX."]
                    #[component(name = "pipe")]
                    Pipe,
                    #[doc = "/ Read-only file system, similar to `EROFS` in POSIX."]
                    #[component(name = "read-only")]
                    ReadOnly,
                    #[doc = "/ Invalid seek, similar to `ESPIPE` in POSIX."]
                    #[component(name = "invalid-seek")]
                    InvalidSeek,
                    #[doc = "/ Text file busy, similar to `ETXTBSY` in POSIX."]
                    #[component(name = "text-file-busy")]
                    TextFileBusy,
                    #[doc = "/ Cross-device link, similar to `EXDEV` in POSIX."]
                    #[component(name = "cross-device")]
                    CrossDevice,
                    #[doc = "/ A catch-all for errors not captured by the existing variants."]
                    #[doc = "/ Implementations can use this to extend the error type without"]
                    #[doc = "/ breaking existing code."]
                    #[component(name = "other")]
                    Other(Option<wasmtime::component::__internal::String>),
                }
                impl core::fmt::Debug for ErrorCode {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        match self {
                            ErrorCode::Access => f.debug_tuple("ErrorCode::Access").finish(),
                            ErrorCode::Already => f.debug_tuple("ErrorCode::Already").finish(),
                            ErrorCode::BadDescriptor => {
                                f.debug_tuple("ErrorCode::BadDescriptor").finish()
                            }
                            ErrorCode::Busy => f.debug_tuple("ErrorCode::Busy").finish(),
                            ErrorCode::Deadlock => f.debug_tuple("ErrorCode::Deadlock").finish(),
                            ErrorCode::Quota => f.debug_tuple("ErrorCode::Quota").finish(),
                            ErrorCode::Exist => f.debug_tuple("ErrorCode::Exist").finish(),
                            ErrorCode::FileTooLarge => {
                                f.debug_tuple("ErrorCode::FileTooLarge").finish()
                            }
                            ErrorCode::IllegalByteSequence => {
                                f.debug_tuple("ErrorCode::IllegalByteSequence").finish()
                            }
                            ErrorCode::InProgress => {
                                f.debug_tuple("ErrorCode::InProgress").finish()
                            }
                            ErrorCode::Interrupted => {
                                f.debug_tuple("ErrorCode::Interrupted").finish()
                            }
                            ErrorCode::Invalid => f.debug_tuple("ErrorCode::Invalid").finish(),
                            ErrorCode::Io => f.debug_tuple("ErrorCode::Io").finish(),
                            ErrorCode::IsDirectory => {
                                f.debug_tuple("ErrorCode::IsDirectory").finish()
                            }
                            ErrorCode::Loop => f.debug_tuple("ErrorCode::Loop").finish(),
                            ErrorCode::TooManyLinks => {
                                f.debug_tuple("ErrorCode::TooManyLinks").finish()
                            }
                            ErrorCode::MessageSize => {
                                f.debug_tuple("ErrorCode::MessageSize").finish()
                            }
                            ErrorCode::NameTooLong => {
                                f.debug_tuple("ErrorCode::NameTooLong").finish()
                            }
                            ErrorCode::NoDevice => f.debug_tuple("ErrorCode::NoDevice").finish(),
                            ErrorCode::NoEntry => f.debug_tuple("ErrorCode::NoEntry").finish(),
                            ErrorCode::NoLock => f.debug_tuple("ErrorCode::NoLock").finish(),
                            ErrorCode::InsufficientMemory => {
                                f.debug_tuple("ErrorCode::InsufficientMemory").finish()
                            }
                            ErrorCode::InsufficientSpace => {
                                f.debug_tuple("ErrorCode::InsufficientSpace").finish()
                            }
                            ErrorCode::NotDirectory => {
                                f.debug_tuple("ErrorCode::NotDirectory").finish()
                            }
                            ErrorCode::NotEmpty => f.debug_tuple("ErrorCode::NotEmpty").finish(),
                            ErrorCode::NotRecoverable => {
                                f.debug_tuple("ErrorCode::NotRecoverable").finish()
                            }
                            ErrorCode::Unsupported => {
                                f.debug_tuple("ErrorCode::Unsupported").finish()
                            }
                            ErrorCode::NoTty => f.debug_tuple("ErrorCode::NoTty").finish(),
                            ErrorCode::NoSuchDevice => {
                                f.debug_tuple("ErrorCode::NoSuchDevice").finish()
                            }
                            ErrorCode::Overflow => f.debug_tuple("ErrorCode::Overflow").finish(),
                            ErrorCode::NotPermitted => {
                                f.debug_tuple("ErrorCode::NotPermitted").finish()
                            }
                            ErrorCode::Pipe => f.debug_tuple("ErrorCode::Pipe").finish(),
                            ErrorCode::ReadOnly => f.debug_tuple("ErrorCode::ReadOnly").finish(),
                            ErrorCode::InvalidSeek => {
                                f.debug_tuple("ErrorCode::InvalidSeek").finish()
                            }
                            ErrorCode::TextFileBusy => {
                                f.debug_tuple("ErrorCode::TextFileBusy").finish()
                            }
                            ErrorCode::CrossDevice => {
                                f.debug_tuple("ErrorCode::CrossDevice").finish()
                            }
                            ErrorCode::Other(e) => {
                                f.debug_tuple("ErrorCode::Other").field(e).finish()
                            }
                        }
                    }
                }
                impl core::fmt::Display for ErrorCode {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(f, "{:?}", self)
                    }
                }
                impl core::error::Error for ErrorCode {}
                const _: () = {
                    assert!(16 == <ErrorCode as wasmtime::component::ComponentType>::SIZE32);
                    assert!(4 == <ErrorCode as wasmtime::component::ComponentType>::ALIGN32);
                };
                #[doc = "/ File or memory access pattern advisory information."]
                #[derive(
                    wasmtime::component::ComponentType,
                    wasmtime::component::Lift,
                    wasmtime::component::Lower,
                )]
                #[component(enum)]
                #[derive(Clone, Copy, Eq, PartialEq)]
                #[repr(u8)]
                pub enum Advice {
                    #[doc = "/ The application has no advice to give on its behavior with respect"]
                    #[doc = "/ to the specified data."]
                    #[component(name = "normal")]
                    Normal,
                    #[doc = "/ The application expects to access the specified data sequentially"]
                    #[doc = "/ from lower offsets to higher offsets."]
                    #[component(name = "sequential")]
                    Sequential,
                    #[doc = "/ The application expects to access the specified data in a random"]
                    #[doc = "/ order."]
                    #[component(name = "random")]
                    Random,
                    #[doc = "/ The application expects to access the specified data in the near"]
                    #[doc = "/ future."]
                    #[component(name = "will-need")]
                    WillNeed,
                    #[doc = "/ The application expects that it will not access the specified data"]
                    #[doc = "/ in the near future."]
                    #[component(name = "dont-need")]
                    DontNeed,
                    #[doc = "/ The application expects to access the specified data once and then"]
                    #[doc = "/ not reuse it thereafter."]
                    #[component(name = "no-reuse")]
                    NoReuse,
                }
                impl core::fmt::Debug for Advice {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        match self {
                            Advice::Normal => f.debug_tuple("Advice::Normal").finish(),
                            Advice::Sequential => f.debug_tuple("Advice::Sequential").finish(),
                            Advice::Random => f.debug_tuple("Advice::Random").finish(),
                            Advice::WillNeed => f.debug_tuple("Advice::WillNeed").finish(),
                            Advice::DontNeed => f.debug_tuple("Advice::DontNeed").finish(),
                            Advice::NoReuse => f.debug_tuple("Advice::NoReuse").finish(),
                        }
                    }
                }
                const _: () = {
                    assert!(1 == <Advice as wasmtime::component::ComponentType>::SIZE32);
                    assert!(1 == <Advice as wasmtime::component::ComponentType>::ALIGN32);
                };
                #[doc = "/ A 128-bit hash value, split into parts because wasm doesn\'t have a"]
                #[doc = "/ 128-bit integer type."]
                #[derive(
                    wasmtime::component::ComponentType,
                    wasmtime::component::Lift,
                    wasmtime::component::Lower,
                )]
                #[component(record)]
                #[derive(Clone, Copy)]
                pub struct MetadataHashValue {
                    #[doc = "/ 64 bits of a 128-bit hash value."]
                    #[component(name = "lower")]
                    pub lower: u64,
                    #[doc = "/ Another 64 bits of a 128-bit hash value."]
                    #[component(name = "upper")]
                    pub upper: u64,
                }
                impl core::fmt::Debug for MetadataHashValue {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.debug_struct("MetadataHashValue")
                            .field("lower", &self.lower)
                            .field("upper", &self.upper)
                            .finish()
                    }
                }
                const _: () = {
                    assert!(
                        16 == <MetadataHashValue as wasmtime::component::ComponentType>::SIZE32
                    );
                    assert!(
                        8 == <MetadataHashValue as wasmtime::component::ComponentType>::ALIGN32
                    );
                };
                #[doc = "/ A descriptor is a reference to a filesystem object, which may be a file,"]
                #[doc = "/ directory, named pipe, special file, or other object on which filesystem"]
                #[doc = "/ calls may be made."]
                pub use super::super::super::__with_name2 as Descriptor;
                pub trait HostDescriptorWithStore<T>: wasmtime::component::HasData + Send {
                    #[doc = "/ Return a stream for reading from a file."]
                    #[doc = "/ "]
                    #[doc = "/ Multiple read, write, and append streams may be active on the same open"]
                    #[doc = "/ file and they do not interfere with each other."]
                    #[doc = "/ "]
                    #[doc = "/ This function returns a `stream` which provides the data received from the"]
                    #[doc = "/ file, and a `future` providing additional error information in case an"]
                    #[doc = "/ error is encountered."]
                    #[doc = "/ "]
                    #[doc = "/ If no error is encountered, `stream.read` on the `stream` will return"]
                    #[doc = "/ `read-status::closed` with no `error-context` and the future resolves to"]
                    #[doc = "/ the value `ok`. If an error is encountered, `stream.read` on the"]
                    #[doc = "/ `stream` returns `read-status::closed` with an `error-context` and the future"]
                    #[doc = "/ resolves to `err` with an `error-code`."]
                    #[doc = "/ "]
                    #[doc = "/ Note: This is similar to `pread` in POSIX."]
                    fn read_via_stream(
                        host: wasmtime::component::Access<T, Self>,
                        self_: wasmtime::component::Resource<Descriptor>,
                        offset: Filesize,
                    ) -> wasmtime::Result<(
                        wasmtime::component::StreamReader<u8>,
                        wasmtime::component::FutureReader<Result<(), ErrorCode>>,
                    )>;

                    #[doc = "/ Return a stream for writing to a file, if available."]
                    #[doc = "/ "]
                    #[doc = "/ May fail with an error-code describing why the file cannot be written."]
                    #[doc = "/ "]
                    #[doc = "/ It is valid to write past the end of a file; the file is extended to the"]
                    #[doc = "/ extent of the write, with bytes between the previous end and the start of"]
                    #[doc = "/ the write set to zero."]
                    #[doc = "/ "]
                    #[doc = "/ This function returns once either full contents of the stream are"]
                    #[doc = "/ written or an error is encountered."]
                    #[doc = "/ "]
                    #[doc = "/ Note: This is similar to `pwrite` in POSIX."]
                    fn write_via_stream(
                        host: wasmtime::component::Access<T, Self>,
                        self_: wasmtime::component::Resource<Descriptor>,
                        data: wasmtime::component::StreamReader<u8>,
                        offset: Filesize,
                    ) -> wasmtime::Result<wasmtime::component::FutureReader<Result<(), ErrorCode>>>;

                    #[doc = "/ Return a stream for appending to a file, if available."]
                    #[doc = "/ "]
                    #[doc = "/ May fail with an error-code describing why the file cannot be appended."]
                    #[doc = "/ "]
                    #[doc = "/ This function returns once either full contents of the stream are"]
                    #[doc = "/ written or an error is encountered."]
                    #[doc = "/ "]
                    #[doc = "/ Note: This is similar to `write` with `O_APPEND` in POSIX."]
                    fn append_via_stream(
                        host: wasmtime::component::Access<T, Self>,
                        self_: wasmtime::component::Resource<Descriptor>,
                        data: wasmtime::component::StreamReader<u8>,
                    ) -> wasmtime::Result<wasmtime::component::FutureReader<Result<(), ErrorCode>>>;

                    #[doc = "/ Provide file advisory information on a descriptor."]
                    #[doc = "/ "]
                    #[doc = "/ This is similar to `posix_fadvise` in POSIX."]
                    fn advise(
                        accessor: &wasmtime::component::Accessor<T, Self>,
                        self_: wasmtime::component::Resource<Descriptor>,
                        offset: Filesize,
                        length: Filesize,
                        advice: Advice,
                    ) -> impl ::core::future::Future<
                        Output = Result<(), super::super::super::_TrappableError0>,
                    > + Send;

                    #[doc = "/ Synchronize the data of a file to disk."]
                    #[doc = "/ "]
                    #[doc = "/ This function succeeds with no effect if the file descriptor is not"]
                    #[doc = "/ opened for writing."]
                    #[doc = "/ "]
                    #[doc = "/ Note: This is similar to `fdatasync` in POSIX."]
                    fn sync_data(
                        accessor: &wasmtime::component::Accessor<T, Self>,
                        self_: wasmtime::component::Resource<Descriptor>,
                    ) -> impl ::core::future::Future<
                        Output = Result<(), super::super::super::_TrappableError0>,
                    > + Send;

                    #[doc = "/ Get flags associated with a descriptor."]
                    #[doc = "/ "]
                    #[doc = "/ Note: This returns similar flags to `fcntl(fd, F_GETFL)` in POSIX."]
                    #[doc = "/ "]
                    #[doc = "/ Note: This returns the value that was the `fs_flags` value returned"]
                    #[doc = "/ from `fdstat_get` in earlier versions of WASI."]
                    fn get_flags(
                        accessor: &wasmtime::component::Accessor<T, Self>,
                        self_: wasmtime::component::Resource<Descriptor>,
                    ) -> impl ::core::future::Future<
                        Output = Result<DescriptorFlags, super::super::super::_TrappableError0>,
                    > + Send;

                    #[doc = "/ Get the dynamic type of a descriptor."]
                    #[doc = "/ "]
                    #[doc = "/ Note: This returns the same value as the `type` field of the `fd-stat`"]
                    #[doc = "/ returned by `stat`, `stat-at` and similar."]
                    #[doc = "/ "]
                    #[doc = "/ Note: This returns similar flags to the `st_mode & S_IFMT` value provided"]
                    #[doc = "/ by `fstat` in POSIX."]
                    #[doc = "/ "]
                    #[doc = "/ Note: This returns the value that was the `fs_filetype` value returned"]
                    #[doc = "/ from `fdstat_get` in earlier versions of WASI."]
                    fn get_type(
                        accessor: &wasmtime::component::Accessor<T, Self>,
                        self_: wasmtime::component::Resource<Descriptor>,
                    ) -> impl ::core::future::Future<
                        Output = Result<DescriptorType, super::super::super::_TrappableError0>,
                    > + Send;

                    #[doc = "/ Adjust the size of an open file. If this increases the file\'s size, the"]
                    #[doc = "/ extra bytes are filled with zeros."]
                    #[doc = "/ "]
                    #[doc = "/ Note: This was called `fd_filestat_set_size` in earlier versions of WASI."]
                    fn set_size(
                        accessor: &wasmtime::component::Accessor<T, Self>,
                        self_: wasmtime::component::Resource<Descriptor>,
                        size: Filesize,
                    ) -> impl ::core::future::Future<
                        Output = Result<(), super::super::super::_TrappableError0>,
                    > + Send;

                    #[doc = "/ Adjust the timestamps of an open file or directory."]
                    #[doc = "/ "]
                    #[doc = "/ Note: This is similar to `futimens` in POSIX."]
                    #[doc = "/ "]
                    #[doc = "/ Note: This was called `fd_filestat_set_times` in earlier versions of WASI."]
                    fn set_times(
                        accessor: &wasmtime::component::Accessor<T, Self>,
                        self_: wasmtime::component::Resource<Descriptor>,
                        data_access_timestamp: NewTimestamp,
                        data_modification_timestamp: NewTimestamp,
                    ) -> impl ::core::future::Future<
                        Output = Result<(), super::super::super::_TrappableError0>,
                    > + Send;

                    #[doc = "/ Read directory entries from a directory."]
                    #[doc = "/ "]
                    #[doc = "/ On filesystems where directories contain entries referring to themselves"]
                    #[doc = "/ and their parents, often named `.` and `..` respectively, these entries"]
                    #[doc = "/ are omitted."]
                    #[doc = "/ "]
                    #[doc = "/ This always returns a new stream which starts at the beginning of the"]
                    #[doc = "/ directory. Multiple streams may be active on the same directory, and they"]
                    #[doc = "/ do not interfere with each other."]
                    #[doc = "/ "]
                    #[doc = "/ This function returns a future, which will resolve to an error code if"]
                    #[doc = "/ reading full contents of the directory fails."]
                    fn read_directory(
                        host: wasmtime::component::Access<T, Self>,
                        self_: wasmtime::component::Resource<Descriptor>,
                    ) -> wasmtime::Result<(
                        wasmtime::component::StreamReader<DirectoryEntry>,
                        wasmtime::component::FutureReader<Result<(), ErrorCode>>,
                    )>;

                    #[doc = "/ Synchronize the data and metadata of a file to disk."]
                    #[doc = "/ "]
                    #[doc = "/ This function succeeds with no effect if the file descriptor is not"]
                    #[doc = "/ opened for writing."]
                    #[doc = "/ "]
                    #[doc = "/ Note: This is similar to `fsync` in POSIX."]
                    fn sync(
                        accessor: &wasmtime::component::Accessor<T, Self>,
                        self_: wasmtime::component::Resource<Descriptor>,
                    ) -> impl ::core::future::Future<
                        Output = Result<(), super::super::super::_TrappableError0>,
                    > + Send;

                    #[doc = "/ Create a directory."]
                    #[doc = "/ "]
                    #[doc = "/ Note: This is similar to `mkdirat` in POSIX."]
                    fn create_directory_at(
                        accessor: &wasmtime::component::Accessor<T, Self>,
                        self_: wasmtime::component::Resource<Descriptor>,
                        path: wasmtime::component::__internal::String,
                    ) -> impl ::core::future::Future<
                        Output = Result<(), super::super::super::_TrappableError0>,
                    > + Send;

                    #[doc = "/ Return the attributes of an open file or directory."]
                    #[doc = "/ "]
                    #[doc = "/ Note: This is similar to `fstat` in POSIX, except that it does not return"]
                    #[doc = "/ device and inode information. For testing whether two descriptors refer to"]
                    #[doc = "/ the same underlying filesystem object, use `is-same-object`. To obtain"]
                    #[doc = "/ additional data that can be used do determine whether a file has been"]
                    #[doc = "/ modified, use `metadata-hash`."]
                    #[doc = "/ "]
                    #[doc = "/ Note: This was called `fd_filestat_get` in earlier versions of WASI."]
                    fn stat(
                        accessor: &wasmtime::component::Accessor<T, Self>,
                        self_: wasmtime::component::Resource<Descriptor>,
                    ) -> impl ::core::future::Future<
                        Output = Result<DescriptorStat, super::super::super::_TrappableError0>,
                    > + Send;

                    #[doc = "/ Return the attributes of a file or directory."]
                    #[doc = "/ "]
                    #[doc = "/ Note: This is similar to `fstatat` in POSIX, except that it does not"]
                    #[doc = "/ return device and inode information. See the `stat` description for a"]
                    #[doc = "/ discussion of alternatives."]
                    #[doc = "/ "]
                    #[doc = "/ Note: This was called `path_filestat_get` in earlier versions of WASI."]
                    fn stat_at(
                        accessor: &wasmtime::component::Accessor<T, Self>,
                        self_: wasmtime::component::Resource<Descriptor>,
                        path_flags: PathFlags,
                        path: wasmtime::component::__internal::String,
                    ) -> impl ::core::future::Future<
                        Output = Result<DescriptorStat, super::super::super::_TrappableError0>,
                    > + Send;

                    #[doc = "/ Adjust the timestamps of a file or directory."]
                    #[doc = "/ "]
                    #[doc = "/ Note: This is similar to `utimensat` in POSIX."]
                    #[doc = "/ "]
                    #[doc = "/ Note: This was called `path_filestat_set_times` in earlier versions of"]
                    #[doc = "/ WASI."]
                    fn set_times_at(
                        accessor: &wasmtime::component::Accessor<T, Self>,
                        self_: wasmtime::component::Resource<Descriptor>,
                        path_flags: PathFlags,
                        path: wasmtime::component::__internal::String,
                        data_access_timestamp: NewTimestamp,
                        data_modification_timestamp: NewTimestamp,
                    ) -> impl ::core::future::Future<
                        Output = Result<(), super::super::super::_TrappableError0>,
                    > + Send;

                    #[doc = "/ Create a hard link."]
                    #[doc = "/ "]
                    #[doc = "/ Fails with `error-code::no-entry` if the old path does not exist,"]
                    #[doc = "/ with `error-code::exist` if the new path already exists, and"]
                    #[doc = "/ `error-code::not-permitted` if the old path is not a file."]
                    #[doc = "/ "]
                    #[doc = "/ Note: This is similar to `linkat` in POSIX."]
                    fn link_at(
                        accessor: &wasmtime::component::Accessor<T, Self>,
                        self_: wasmtime::component::Resource<Descriptor>,
                        old_path_flags: PathFlags,
                        old_path: wasmtime::component::__internal::String,
                        new_descriptor: wasmtime::component::Resource<Descriptor>,
                        new_path: wasmtime::component::__internal::String,
                    ) -> impl ::core::future::Future<
                        Output = Result<(), super::super::super::_TrappableError0>,
                    > + Send;

                    #[doc = "/ Open a file or directory."]
                    #[doc = "/ "]
                    #[doc = "/ If `flags` contains `descriptor-flags::mutate-directory`, and the base"]
                    #[doc = "/ descriptor doesn\'t have `descriptor-flags::mutate-directory` set,"]
                    #[doc = "/ `open-at` fails with `error-code::read-only`."]
                    #[doc = "/ "]
                    #[doc = "/ If `flags` contains `write` or `mutate-directory`, or `open-flags`"]
                    #[doc = "/ contains `truncate` or `create`, and the base descriptor doesn\'t have"]
                    #[doc = "/ `descriptor-flags::mutate-directory` set, `open-at` fails with"]
                    #[doc = "/ `error-code::read-only`."]
                    #[doc = "/ "]
                    #[doc = "/ Note: This is similar to `openat` in POSIX."]
                    fn open_at(
                        accessor: &wasmtime::component::Accessor<T, Self>,
                        self_: wasmtime::component::Resource<Descriptor>,
                        path_flags: PathFlags,
                        path: wasmtime::component::__internal::String,
                        open_flags: OpenFlags,
                        flags: DescriptorFlags,
                    ) -> impl ::core::future::Future<
                        Output = Result<
                            wasmtime::component::Resource<Descriptor>,
                            super::super::super::_TrappableError0,
                        >,
                    > + Send;

                    #[doc = "/ Read the contents of a symbolic link."]
                    #[doc = "/ "]
                    #[doc = "/ If the contents contain an absolute or rooted path in the underlying"]
                    #[doc = "/ filesystem, this function fails with `error-code::not-permitted`."]
                    #[doc = "/ "]
                    #[doc = "/ Note: This is similar to `readlinkat` in POSIX."]
                    fn readlink_at(
                        accessor: &wasmtime::component::Accessor<T, Self>,
                        self_: wasmtime::component::Resource<Descriptor>,
                        path: wasmtime::component::__internal::String,
                    ) -> impl ::core::future::Future<
                        Output = Result<
                            wasmtime::component::__internal::String,
                            super::super::super::_TrappableError0,
                        >,
                    > + Send;

                    #[doc = "/ Remove a directory."]
                    #[doc = "/ "]
                    #[doc = "/ Return `error-code::not-empty` if the directory is not empty."]
                    #[doc = "/ "]
                    #[doc = "/ Note: This is similar to `unlinkat(fd, path, AT_REMOVEDIR)` in POSIX."]
                    fn remove_directory_at(
                        accessor: &wasmtime::component::Accessor<T, Self>,
                        self_: wasmtime::component::Resource<Descriptor>,
                        path: wasmtime::component::__internal::String,
                    ) -> impl ::core::future::Future<
                        Output = Result<(), super::super::super::_TrappableError0>,
                    > + Send;

                    #[doc = "/ Rename a filesystem object."]
                    #[doc = "/ "]
                    #[doc = "/ Note: This is similar to `renameat` in POSIX."]
                    fn rename_at(
                        accessor: &wasmtime::component::Accessor<T, Self>,
                        self_: wasmtime::component::Resource<Descriptor>,
                        old_path: wasmtime::component::__internal::String,
                        new_descriptor: wasmtime::component::Resource<Descriptor>,
                        new_path: wasmtime::component::__internal::String,
                    ) -> impl ::core::future::Future<
                        Output = Result<(), super::super::super::_TrappableError0>,
                    > + Send;

                    #[doc = "/ Create a symbolic link (also known as a \"symlink\")."]
                    #[doc = "/ "]
                    #[doc = "/ If `old-path` starts with `/`, the function fails with"]
                    #[doc = "/ `error-code::not-permitted`."]
                    #[doc = "/ "]
                    #[doc = "/ Note: This is similar to `symlinkat` in POSIX."]
                    fn symlink_at(
                        accessor: &wasmtime::component::Accessor<T, Self>,
                        self_: wasmtime::component::Resource<Descriptor>,
                        old_path: wasmtime::component::__internal::String,
                        new_path: wasmtime::component::__internal::String,
                    ) -> impl ::core::future::Future<
                        Output = Result<(), super::super::super::_TrappableError0>,
                    > + Send;

                    #[doc = "/ Unlink a filesystem object that is not a directory."]
                    #[doc = "/ "]
                    #[doc = "/ This is similar to `unlinkat(fd, path, 0)` in POSIX."]
                    #[doc = "/ "]
                    #[doc = "/ Error returns are as specified by POSIX."]
                    #[doc = "/ "]
                    #[doc = "/ If the filesystem object is a directory, `error-code::access` or"]
                    #[doc = "/ `error-code::is-directory` may be returned instead of the"]
                    #[doc = "/ POSIX-specified `error-code::not-permitted`."]
                    fn unlink_file_at(
                        accessor: &wasmtime::component::Accessor<T, Self>,
                        self_: wasmtime::component::Resource<Descriptor>,
                        path: wasmtime::component::__internal::String,
                    ) -> impl ::core::future::Future<
                        Output = Result<(), super::super::super::_TrappableError0>,
                    > + Send;

                    #[doc = "/ Test whether two descriptors refer to the same filesystem object."]
                    #[doc = "/ "]
                    #[doc = "/ In POSIX, this corresponds to testing whether the two descriptors have the"]
                    #[doc = "/ same device (`st_dev`) and inode (`st_ino` or `d_ino`) numbers."]
                    #[doc = "/ wasi-filesystem does not expose device and inode numbers, so this function"]
                    #[doc = "/ may be used instead."]
                    fn is_same_object(
                        accessor: &wasmtime::component::Accessor<T, Self>,
                        self_: wasmtime::component::Resource<Descriptor>,
                        other: wasmtime::component::Resource<Descriptor>,
                    ) -> impl ::core::future::Future<Output = wasmtime::Result<bool>> + Send;

                    #[doc = "/ Return a hash of the metadata associated with a filesystem object referred"]
                    #[doc = "/ to by a descriptor."]
                    #[doc = "/ "]
                    #[doc = "/ This returns a hash of the last-modification timestamp and file size, and"]
                    #[doc = "/ may also include the inode number, device number, birth timestamp, and"]
                    #[doc = "/ other metadata fields that may change when the file is modified or"]
                    #[doc = "/ replaced. It may also include a secret value chosen by the"]
                    #[doc = "/ implementation and not otherwise exposed."]
                    #[doc = "/ "]
                    #[doc = "/ Implementations are encouraged to provide the following properties:"]
                    #[doc = "/ "]
                    #[doc = "/  - If the file is not modified or replaced, the computed hash value should"]
                    #[doc = "/    usually not change."]
                    #[doc = "/  - If the object is modified or replaced, the computed hash value should"]
                    #[doc = "/    usually change."]
                    #[doc = "/  - The inputs to the hash should not be easily computable from the"]
                    #[doc = "/    computed hash."]
                    #[doc = "/ "]
                    #[doc = "/ However, none of these is required."]
                    fn metadata_hash(
                        accessor: &wasmtime::component::Accessor<T, Self>,
                        self_: wasmtime::component::Resource<Descriptor>,
                    ) -> impl ::core::future::Future<
                        Output = Result<MetadataHashValue, super::super::super::_TrappableError0>,
                    > + Send;

                    #[doc = "/ Return a hash of the metadata associated with a filesystem object referred"]
                    #[doc = "/ to by a directory descriptor and a relative path."]
                    #[doc = "/ "]
                    #[doc = "/ This performs the same hash computation as `metadata-hash`."]
                    fn metadata_hash_at(
                        accessor: &wasmtime::component::Accessor<T, Self>,
                        self_: wasmtime::component::Resource<Descriptor>,
                        path_flags: PathFlags,
                        path: wasmtime::component::__internal::String,
                    ) -> impl ::core::future::Future<
                        Output = Result<MetadataHashValue, super::super::super::_TrappableError0>,
                    > + Send;
                }
                pub trait HostDescriptor: Send {
                    fn drop(
                        &mut self,
                        rep: wasmtime::component::Resource<Descriptor>,
                    ) -> wasmtime::Result<()>;
                }
                impl<_T: HostDescriptor + ?Sized + Send> HostDescriptor for &mut _T {
                    fn drop(
                        &mut self,
                        rep: wasmtime::component::Resource<Descriptor>,
                    ) -> wasmtime::Result<()> {
                        HostDescriptor::drop(*self, rep)
                    }
                }
                pub trait HostWithStore<T>:
                    wasmtime::component::HasData + HostDescriptorWithStore<T> + Send
                {
                }
                impl<H: ?Sized, T> HostWithStore<T> for H where
                    H: wasmtime::component::HasData + HostDescriptorWithStore<T> + Send
                {
                }
                pub trait Host: HostDescriptor + Send {
                    fn convert_error_code(
                        &mut self,
                        err: super::super::super::_TrappableError0,
                    ) -> wasmtime::Result<ErrorCode>;
                }
                impl<_T: Host + ?Sized + Send> Host for &mut _T {
                    fn convert_error_code(
                        &mut self,
                        err: super::super::super::_TrappableError0,
                    ) -> wasmtime::Result<ErrorCode> {
                        Host::convert_error_code(*self, err)
                    }
                }
                pub fn add_to_linker_instance<T, D>(
                    inst: &mut wasmtime::component::LinkerInstance<'_, T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static + Send,
                {
                    inst.resource(
                        "descriptor",
                        wasmtime::component::ResourceType::host::<Descriptor>(),
                        move |mut store, rep| -> wasmtime::Result<()> {
                            let resource = wasmtime::component::Resource::new_own(rep);
                            wasmtime::ToWasmtimeResult::to_wasmtime_result(HostDescriptor::drop(
                                &mut host_getter(store.data_mut()),
                                resource,
                            ))
                        },
                    )?;
                    inst.func_wrap(
                        "[method]descriptor.read-via-stream",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>,
                              (arg0, arg1): (
                            wasmtime::component::Resource<Descriptor>,
                            Filesize,
                        )| {
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "types",
                                function = "[method]descriptor.read-via-stream",
                            );
                            let _enter = span.enter();
                            tracing::event!(
                                tracing::Level::TRACE,
                                self_ = tracing::field::debug(&arg0),
                                offset = tracing::field::debug(&arg1),
                                "call"
                            );
                            let access_cx = wasmtime::AsContextMut::as_context_mut(&mut caller);
                            let host =
                                wasmtime::component::Access::<T, D>::new(access_cx, host_getter);
                            let r = <D as HostDescriptorWithStore<T>>::read_via_stream(
                                host, arg0, arg1,
                            );
                            tracing::event!(
                                tracing::Level::TRACE,
                                result = tracing::field::debug(&r),
                                "return"
                            );
                            Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                        },
                    )?;
                    inst.func_wrap(
                        "[method]descriptor.write-via-stream",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>,
                              (arg0, arg1, arg2): (
                            wasmtime::component::Resource<Descriptor>,
                            wasmtime::component::StreamReader<u8>,
                            Filesize,
                        )| {
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "types",
                                function = "[method]descriptor.write-via-stream",
                            );
                            let _enter = span.enter();
                            tracing::event!(
                                tracing::Level::TRACE,
                                self_ = tracing::field::debug(&arg0),
                                data = tracing::field::debug(&arg1),
                                offset = tracing::field::debug(&arg2),
                                "call"
                            );
                            let access_cx = wasmtime::AsContextMut::as_context_mut(&mut caller);
                            let host =
                                wasmtime::component::Access::<T, D>::new(access_cx, host_getter);
                            let r = <D as HostDescriptorWithStore<T>>::write_via_stream(
                                host, arg0, arg1, arg2,
                            );
                            tracing::event!(
                                tracing::Level::TRACE,
                                result = tracing::field::debug(&r),
                                "return"
                            );
                            Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                        },
                    )?;
                    inst.func_wrap(
                        "[method]descriptor.append-via-stream",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>,
                              (arg0, arg1): (
                            wasmtime::component::Resource<Descriptor>,
                            wasmtime::component::StreamReader<u8>,
                        )| {
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "types",
                                function = "[method]descriptor.append-via-stream",
                            );
                            let _enter = span.enter();
                            tracing::event!(
                                tracing::Level::TRACE,
                                self_ = tracing::field::debug(&arg0),
                                data = tracing::field::debug(&arg1),
                                "call"
                            );
                            let access_cx = wasmtime::AsContextMut::as_context_mut(&mut caller);
                            let host =
                                wasmtime::component::Access::<T, D>::new(access_cx, host_getter);
                            let r = <D as HostDescriptorWithStore<T>>::append_via_stream(
                                host, arg0, arg1,
                            );
                            tracing::event!(
                                tracing::Level::TRACE,
                                result = tracing::field::debug(&r),
                                "return"
                            );
                            Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                        },
                    )?;
                    inst.func_wrap_concurrent(
                        "[method]descriptor.advise",
                        move |caller: &wasmtime::component::Accessor<T>,
                              (arg0, arg1, arg2, arg3): (
                            wasmtime::component::Resource<Descriptor>,
                            Filesize,
                            Filesize,
                            Advice,
                        )| {
                            use tracing::Instrument;
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "types",
                                function = "[method]descriptor.advise",
                            );
                            wasmtime::component::__internal::Box::pin(
                                async move {
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        self_ = tracing::field::debug(&arg0),
                                        offset = tracing::field::debug(&arg1),
                                        length = tracing::field::debug(&arg2),
                                        advice = tracing::field::debug(&arg3),
                                        "call"
                                    );
                                    let host = &caller.with_getter::<D>(host_getter);
                                    let r = <D as HostDescriptorWithStore<T>>::advise(
                                        host, arg0, arg1, arg2, arg3,
                                    )
                                    .await;
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        result = tracing::field::debug(&r),
                                        "return"
                                    );
                                    Ok((match r {
                                        Ok(a) => Ok(a),
                                        Err(e) => {
                                            Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(
                                                caller.with(|mut host| {
                                                    Host::convert_error_code(
                                                        &mut host_getter(host.get()),
                                                        e,
                                                    )
                                                }),
                                            )?)
                                        }
                                    },))
                                }
                                .instrument(span),
                            )
                        },
                    )?;
                    inst.func_wrap_concurrent("[method]descriptor.sync-data", move|caller: &wasmtime::component::Accessor::<T>, (arg0,): (wasmtime::component::Resource<Descriptor>,)|{
                        use tracing::Instrument;
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]descriptor.sync-data",);
                        wasmtime::component::__internal::Box::pin(async move {
                            tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
                            let host =  &caller.with_getter::<D>(host_getter);
                            let r =  <D as HostDescriptorWithStore<T>>::sync_data(host, arg0,).await;
                            tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                            Ok((match r {
                                Ok(a) => Ok(a),
                                Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(caller.with(|mut host|Host::convert_error_code(&mut host_getter(host.get()), e)))?),
                            },))
                        }.instrument(span))
                    })?;
                    inst.func_wrap_concurrent("[method]descriptor.get-flags", move|caller: &wasmtime::component::Accessor::<T>, (arg0,): (wasmtime::component::Resource<Descriptor>,)|{
                        use tracing::Instrument;
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]descriptor.get-flags",);
                        wasmtime::component::__internal::Box::pin(async move {
                            tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
                            let host =  &caller.with_getter::<D>(host_getter);
                            let r =  <D as HostDescriptorWithStore<T>>::get_flags(host, arg0,).await;
                            tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                            Ok((match r {
                                Ok(a) => Ok(a),
                                Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(caller.with(|mut host|Host::convert_error_code(&mut host_getter(host.get()), e)))?),
                            },))
                        }.instrument(span))
                    })?;
                    inst.func_wrap_concurrent("[method]descriptor.get-type", move|caller: &wasmtime::component::Accessor::<T>, (arg0,): (wasmtime::component::Resource<Descriptor>,)|{
                        use tracing::Instrument;
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]descriptor.get-type",);
                        wasmtime::component::__internal::Box::pin(async move {
                            tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
                            let host =  &caller.with_getter::<D>(host_getter);
                            let r =  <D as HostDescriptorWithStore<T>>::get_type(host, arg0,).await;
                            tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                            Ok((match r {
                                Ok(a) => Ok(a),
                                Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(caller.with(|mut host|Host::convert_error_code(&mut host_getter(host.get()), e)))?),
                            },))
                        }.instrument(span))
                    })?;
                    inst.func_wrap_concurrent(
                        "[method]descriptor.set-size",
                        move |caller: &wasmtime::component::Accessor<T>,
                              (arg0, arg1): (
                            wasmtime::component::Resource<Descriptor>,
                            Filesize,
                        )| {
                            use tracing::Instrument;
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "types",
                                function = "[method]descriptor.set-size",
                            );
                            wasmtime::component::__internal::Box::pin(
                                async move {
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        self_ = tracing::field::debug(&arg0),
                                        size = tracing::field::debug(&arg1),
                                        "call"
                                    );
                                    let host = &caller.with_getter::<D>(host_getter);
                                    let r = <D as HostDescriptorWithStore<T>>::set_size(
                                        host, arg0, arg1,
                                    )
                                    .await;
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        result = tracing::field::debug(&r),
                                        "return"
                                    );
                                    Ok((match r {
                                        Ok(a) => Ok(a),
                                        Err(e) => {
                                            Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(
                                                caller.with(|mut host| {
                                                    Host::convert_error_code(
                                                        &mut host_getter(host.get()),
                                                        e,
                                                    )
                                                }),
                                            )?)
                                        }
                                    },))
                                }
                                .instrument(span),
                            )
                        },
                    )?;
                    inst.func_wrap_concurrent(
                        "[method]descriptor.set-times",
                        move |caller: &wasmtime::component::Accessor<T>,
                              (arg0, arg1, arg2): (
                            wasmtime::component::Resource<Descriptor>,
                            NewTimestamp,
                            NewTimestamp,
                        )| {
                            use tracing::Instrument;
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "types",
                                function = "[method]descriptor.set-times",
                            );
                            wasmtime::component::__internal::Box::pin(
                                async move {
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        self_ = tracing::field::debug(&arg0),
                                        data_access_timestamp = tracing::field::debug(&arg1),
                                        data_modification_timestamp = tracing::field::debug(&arg2),
                                        "call"
                                    );
                                    let host = &caller.with_getter::<D>(host_getter);
                                    let r = <D as HostDescriptorWithStore<T>>::set_times(
                                        host, arg0, arg1, arg2,
                                    )
                                    .await;
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        result = tracing::field::debug(&r),
                                        "return"
                                    );
                                    Ok((match r {
                                        Ok(a) => Ok(a),
                                        Err(e) => {
                                            Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(
                                                caller.with(|mut host| {
                                                    Host::convert_error_code(
                                                        &mut host_getter(host.get()),
                                                        e,
                                                    )
                                                }),
                                            )?)
                                        }
                                    },))
                                }
                                .instrument(span),
                            )
                        },
                    )?;
                    inst.func_wrap("[method]descriptor.read-directory", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (wasmtime::component::Resource<Descriptor>,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]descriptor.read-directory",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
                        let access_cx = wasmtime::AsContextMut::as_context_mut(&mut caller);
                        let host = wasmtime::component::Access::<T, D>::new(access_cx, host_getter);
                        let r =  <D as HostDescriptorWithStore<T>>::read_directory(host, arg0,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                    })?;
                    inst.func_wrap_concurrent("[method]descriptor.sync", move|caller: &wasmtime::component::Accessor::<T>, (arg0,): (wasmtime::component::Resource<Descriptor>,)|{
                        use tracing::Instrument;
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]descriptor.sync",);
                        wasmtime::component::__internal::Box::pin(async move {
                            tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
                            let host =  &caller.with_getter::<D>(host_getter);
                            let r =  <D as HostDescriptorWithStore<T>>::sync(host, arg0,).await;
                            tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                            Ok((match r {
                                Ok(a) => Ok(a),
                                Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(caller.with(|mut host|Host::convert_error_code(&mut host_getter(host.get()), e)))?),
                            },))
                        }.instrument(span))
                    })?;
                    inst.func_wrap_concurrent(
                        "[method]descriptor.create-directory-at",
                        move |caller: &wasmtime::component::Accessor<T>,
                              (arg0, arg1): (
                            wasmtime::component::Resource<Descriptor>,
                            wasmtime::component::__internal::String,
                        )| {
                            use tracing::Instrument;
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "types",
                                function = "[method]descriptor.create-directory-at",
                            );
                            wasmtime::component::__internal::Box::pin(
                                async move {
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        self_ = tracing::field::debug(&arg0),
                                        path = tracing::field::debug(&arg1),
                                        "call"
                                    );
                                    let host = &caller.with_getter::<D>(host_getter);
                                    let r = <D as HostDescriptorWithStore<T>>::create_directory_at(
                                        host, arg0, arg1,
                                    )
                                    .await;
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        result = tracing::field::debug(&r),
                                        "return"
                                    );
                                    Ok((match r {
                                        Ok(a) => Ok(a),
                                        Err(e) => {
                                            Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(
                                                caller.with(|mut host| {
                                                    Host::convert_error_code(
                                                        &mut host_getter(host.get()),
                                                        e,
                                                    )
                                                }),
                                            )?)
                                        }
                                    },))
                                }
                                .instrument(span),
                            )
                        },
                    )?;
                    inst.func_wrap_concurrent("[method]descriptor.stat", move|caller: &wasmtime::component::Accessor::<T>, (arg0,): (wasmtime::component::Resource<Descriptor>,)|{
                        use tracing::Instrument;
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]descriptor.stat",);
                        wasmtime::component::__internal::Box::pin(async move {
                            tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
                            let host =  &caller.with_getter::<D>(host_getter);
                            let r =  <D as HostDescriptorWithStore<T>>::stat(host, arg0,).await;
                            tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                            Ok((match r {
                                Ok(a) => Ok(a),
                                Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(caller.with(|mut host|Host::convert_error_code(&mut host_getter(host.get()), e)))?),
                            },))
                        }.instrument(span))
                    })?;
                    inst.func_wrap_concurrent(
                        "[method]descriptor.stat-at",
                        move |caller: &wasmtime::component::Accessor<T>,
                              (arg0, arg1, arg2): (
                            wasmtime::component::Resource<Descriptor>,
                            PathFlags,
                            wasmtime::component::__internal::String,
                        )| {
                            use tracing::Instrument;
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "types",
                                function = "[method]descriptor.stat-at",
                            );
                            wasmtime::component::__internal::Box::pin(
                                async move {
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        self_ = tracing::field::debug(&arg0),
                                        path_flags = tracing::field::debug(&arg1),
                                        path = tracing::field::debug(&arg2),
                                        "call"
                                    );
                                    let host = &caller.with_getter::<D>(host_getter);
                                    let r = <D as HostDescriptorWithStore<T>>::stat_at(
                                        host, arg0, arg1, arg2,
                                    )
                                    .await;
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        result = tracing::field::debug(&r),
                                        "return"
                                    );
                                    Ok((match r {
                                        Ok(a) => Ok(a),
                                        Err(e) => {
                                            Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(
                                                caller.with(|mut host| {
                                                    Host::convert_error_code(
                                                        &mut host_getter(host.get()),
                                                        e,
                                                    )
                                                }),
                                            )?)
                                        }
                                    },))
                                }
                                .instrument(span),
                            )
                        },
                    )?;
                    inst.func_wrap_concurrent(
                        "[method]descriptor.set-times-at",
                        move |caller: &wasmtime::component::Accessor<T>,
                              (arg0, arg1, arg2, arg3, arg4): (
                            wasmtime::component::Resource<Descriptor>,
                            PathFlags,
                            wasmtime::component::__internal::String,
                            NewTimestamp,
                            NewTimestamp,
                        )| {
                            use tracing::Instrument;
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "types",
                                function = "[method]descriptor.set-times-at",
                            );
                            wasmtime::component::__internal::Box::pin(
                                async move {
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        self_ = tracing::field::debug(&arg0),
                                        path_flags = tracing::field::debug(&arg1),
                                        path = tracing::field::debug(&arg2),
                                        data_access_timestamp = tracing::field::debug(&arg3),
                                        data_modification_timestamp = tracing::field::debug(&arg4),
                                        "call"
                                    );
                                    let host = &caller.with_getter::<D>(host_getter);
                                    let r = <D as HostDescriptorWithStore<T>>::set_times_at(
                                        host, arg0, arg1, arg2, arg3, arg4,
                                    )
                                    .await;
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        result = tracing::field::debug(&r),
                                        "return"
                                    );
                                    Ok((match r {
                                        Ok(a) => Ok(a),
                                        Err(e) => {
                                            Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(
                                                caller.with(|mut host| {
                                                    Host::convert_error_code(
                                                        &mut host_getter(host.get()),
                                                        e,
                                                    )
                                                }),
                                            )?)
                                        }
                                    },))
                                }
                                .instrument(span),
                            )
                        },
                    )?;
                    inst.func_wrap_concurrent(
                        "[method]descriptor.link-at",
                        move |caller: &wasmtime::component::Accessor<T>,
                              (arg0, arg1, arg2, arg3, arg4): (
                            wasmtime::component::Resource<Descriptor>,
                            PathFlags,
                            wasmtime::component::__internal::String,
                            wasmtime::component::Resource<Descriptor>,
                            wasmtime::component::__internal::String,
                        )| {
                            use tracing::Instrument;
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "types",
                                function = "[method]descriptor.link-at",
                            );
                            wasmtime::component::__internal::Box::pin(
                                async move {
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        self_ = tracing::field::debug(&arg0),
                                        old_path_flags = tracing::field::debug(&arg1),
                                        old_path = tracing::field::debug(&arg2),
                                        new_descriptor = tracing::field::debug(&arg3),
                                        new_path = tracing::field::debug(&arg4),
                                        "call"
                                    );
                                    let host = &caller.with_getter::<D>(host_getter);
                                    let r = <D as HostDescriptorWithStore<T>>::link_at(
                                        host, arg0, arg1, arg2, arg3, arg4,
                                    )
                                    .await;
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        result = tracing::field::debug(&r),
                                        "return"
                                    );
                                    Ok((match r {
                                        Ok(a) => Ok(a),
                                        Err(e) => {
                                            Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(
                                                caller.with(|mut host| {
                                                    Host::convert_error_code(
                                                        &mut host_getter(host.get()),
                                                        e,
                                                    )
                                                }),
                                            )?)
                                        }
                                    },))
                                }
                                .instrument(span),
                            )
                        },
                    )?;
                    inst.func_wrap_concurrent(
                        "[method]descriptor.open-at",
                        move |caller: &wasmtime::component::Accessor<T>,
                              (arg0, arg1, arg2, arg3, arg4): (
                            wasmtime::component::Resource<Descriptor>,
                            PathFlags,
                            wasmtime::component::__internal::String,
                            OpenFlags,
                            DescriptorFlags,
                        )| {
                            use tracing::Instrument;
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "types",
                                function = "[method]descriptor.open-at",
                            );
                            wasmtime::component::__internal::Box::pin(
                                async move {
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        self_ = tracing::field::debug(&arg0),
                                        path_flags = tracing::field::debug(&arg1),
                                        path = tracing::field::debug(&arg2),
                                        open_flags = tracing::field::debug(&arg3),
                                        flags = tracing::field::debug(&arg4),
                                        "call"
                                    );
                                    let host = &caller.with_getter::<D>(host_getter);
                                    let r = <D as HostDescriptorWithStore<T>>::open_at(
                                        host, arg0, arg1, arg2, arg3, arg4,
                                    )
                                    .await;
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        result = tracing::field::debug(&r),
                                        "return"
                                    );
                                    Ok((match r {
                                        Ok(a) => Ok(a),
                                        Err(e) => {
                                            Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(
                                                caller.with(|mut host| {
                                                    Host::convert_error_code(
                                                        &mut host_getter(host.get()),
                                                        e,
                                                    )
                                                }),
                                            )?)
                                        }
                                    },))
                                }
                                .instrument(span),
                            )
                        },
                    )?;
                    inst.func_wrap_concurrent(
                        "[method]descriptor.readlink-at",
                        move |caller: &wasmtime::component::Accessor<T>,
                              (arg0, arg1): (
                            wasmtime::component::Resource<Descriptor>,
                            wasmtime::component::__internal::String,
                        )| {
                            use tracing::Instrument;
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "types",
                                function = "[method]descriptor.readlink-at",
                            );
                            wasmtime::component::__internal::Box::pin(
                                async move {
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        self_ = tracing::field::debug(&arg0),
                                        path = tracing::field::debug(&arg1),
                                        "call"
                                    );
                                    let host = &caller.with_getter::<D>(host_getter);
                                    let r = <D as HostDescriptorWithStore<T>>::readlink_at(
                                        host, arg0, arg1,
                                    )
                                    .await;
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        result = tracing::field::debug(&r),
                                        "return"
                                    );
                                    Ok((match r {
                                        Ok(a) => Ok(a),
                                        Err(e) => {
                                            Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(
                                                caller.with(|mut host| {
                                                    Host::convert_error_code(
                                                        &mut host_getter(host.get()),
                                                        e,
                                                    )
                                                }),
                                            )?)
                                        }
                                    },))
                                }
                                .instrument(span),
                            )
                        },
                    )?;
                    inst.func_wrap_concurrent(
                        "[method]descriptor.remove-directory-at",
                        move |caller: &wasmtime::component::Accessor<T>,
                              (arg0, arg1): (
                            wasmtime::component::Resource<Descriptor>,
                            wasmtime::component::__internal::String,
                        )| {
                            use tracing::Instrument;
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "types",
                                function = "[method]descriptor.remove-directory-at",
                            );
                            wasmtime::component::__internal::Box::pin(
                                async move {
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        self_ = tracing::field::debug(&arg0),
                                        path = tracing::field::debug(&arg1),
                                        "call"
                                    );
                                    let host = &caller.with_getter::<D>(host_getter);
                                    let r = <D as HostDescriptorWithStore<T>>::remove_directory_at(
                                        host, arg0, arg1,
                                    )
                                    .await;
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        result = tracing::field::debug(&r),
                                        "return"
                                    );
                                    Ok((match r {
                                        Ok(a) => Ok(a),
                                        Err(e) => {
                                            Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(
                                                caller.with(|mut host| {
                                                    Host::convert_error_code(
                                                        &mut host_getter(host.get()),
                                                        e,
                                                    )
                                                }),
                                            )?)
                                        }
                                    },))
                                }
                                .instrument(span),
                            )
                        },
                    )?;
                    inst.func_wrap_concurrent(
                        "[method]descriptor.rename-at",
                        move |caller: &wasmtime::component::Accessor<T>,
                              (arg0, arg1, arg2, arg3): (
                            wasmtime::component::Resource<Descriptor>,
                            wasmtime::component::__internal::String,
                            wasmtime::component::Resource<Descriptor>,
                            wasmtime::component::__internal::String,
                        )| {
                            use tracing::Instrument;
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "types",
                                function = "[method]descriptor.rename-at",
                            );
                            wasmtime::component::__internal::Box::pin(
                                async move {
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        self_ = tracing::field::debug(&arg0),
                                        old_path = tracing::field::debug(&arg1),
                                        new_descriptor = tracing::field::debug(&arg2),
                                        new_path = tracing::field::debug(&arg3),
                                        "call"
                                    );
                                    let host = &caller.with_getter::<D>(host_getter);
                                    let r = <D as HostDescriptorWithStore<T>>::rename_at(
                                        host, arg0, arg1, arg2, arg3,
                                    )
                                    .await;
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        result = tracing::field::debug(&r),
                                        "return"
                                    );
                                    Ok((match r {
                                        Ok(a) => Ok(a),
                                        Err(e) => {
                                            Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(
                                                caller.with(|mut host| {
                                                    Host::convert_error_code(
                                                        &mut host_getter(host.get()),
                                                        e,
                                                    )
                                                }),
                                            )?)
                                        }
                                    },))
                                }
                                .instrument(span),
                            )
                        },
                    )?;
                    inst.func_wrap_concurrent(
                        "[method]descriptor.symlink-at",
                        move |caller: &wasmtime::component::Accessor<T>,
                              (arg0, arg1, arg2): (
                            wasmtime::component::Resource<Descriptor>,
                            wasmtime::component::__internal::String,
                            wasmtime::component::__internal::String,
                        )| {
                            use tracing::Instrument;
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "types",
                                function = "[method]descriptor.symlink-at",
                            );
                            wasmtime::component::__internal::Box::pin(
                                async move {
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        self_ = tracing::field::debug(&arg0),
                                        old_path = tracing::field::debug(&arg1),
                                        new_path = tracing::field::debug(&arg2),
                                        "call"
                                    );
                                    let host = &caller.with_getter::<D>(host_getter);
                                    let r = <D as HostDescriptorWithStore<T>>::symlink_at(
                                        host, arg0, arg1, arg2,
                                    )
                                    .await;
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        result = tracing::field::debug(&r),
                                        "return"
                                    );
                                    Ok((match r {
                                        Ok(a) => Ok(a),
                                        Err(e) => {
                                            Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(
                                                caller.with(|mut host| {
                                                    Host::convert_error_code(
                                                        &mut host_getter(host.get()),
                                                        e,
                                                    )
                                                }),
                                            )?)
                                        }
                                    },))
                                }
                                .instrument(span),
                            )
                        },
                    )?;
                    inst.func_wrap_concurrent(
                        "[method]descriptor.unlink-file-at",
                        move |caller: &wasmtime::component::Accessor<T>,
                              (arg0, arg1): (
                            wasmtime::component::Resource<Descriptor>,
                            wasmtime::component::__internal::String,
                        )| {
                            use tracing::Instrument;
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "types",
                                function = "[method]descriptor.unlink-file-at",
                            );
                            wasmtime::component::__internal::Box::pin(
                                async move {
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        self_ = tracing::field::debug(&arg0),
                                        path = tracing::field::debug(&arg1),
                                        "call"
                                    );
                                    let host = &caller.with_getter::<D>(host_getter);
                                    let r = <D as HostDescriptorWithStore<T>>::unlink_file_at(
                                        host, arg0, arg1,
                                    )
                                    .await;
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        result = tracing::field::debug(&r),
                                        "return"
                                    );
                                    Ok((match r {
                                        Ok(a) => Ok(a),
                                        Err(e) => {
                                            Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(
                                                caller.with(|mut host| {
                                                    Host::convert_error_code(
                                                        &mut host_getter(host.get()),
                                                        e,
                                                    )
                                                }),
                                            )?)
                                        }
                                    },))
                                }
                                .instrument(span),
                            )
                        },
                    )?;
                    inst.func_wrap_concurrent(
                        "[method]descriptor.is-same-object",
                        move |caller: &wasmtime::component::Accessor<T>,
                              (arg0, arg1): (
                            wasmtime::component::Resource<Descriptor>,
                            wasmtime::component::Resource<Descriptor>,
                        )| {
                            use tracing::Instrument;
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "types",
                                function = "[method]descriptor.is-same-object",
                            );
                            wasmtime::component::__internal::Box::pin(
                                async move {
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        self_ = tracing::field::debug(&arg0),
                                        other = tracing::field::debug(&arg1),
                                        "call"
                                    );
                                    let host = &caller.with_getter::<D>(host_getter);
                                    let r = <D as HostDescriptorWithStore<T>>::is_same_object(
                                        host, arg0, arg1,
                                    )
                                    .await;
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        result = tracing::field::debug(&r),
                                        "return"
                                    );
                                    Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                                }
                                .instrument(span),
                            )
                        },
                    )?;
                    inst.func_wrap_concurrent("[method]descriptor.metadata-hash", move|caller: &wasmtime::component::Accessor::<T>, (arg0,): (wasmtime::component::Resource<Descriptor>,)|{
                        use tracing::Instrument;
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]descriptor.metadata-hash",);
                        wasmtime::component::__internal::Box::pin(async move {
                            tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
                            let host =  &caller.with_getter::<D>(host_getter);
                            let r =  <D as HostDescriptorWithStore<T>>::metadata_hash(host, arg0,).await;
                            tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                            Ok((match r {
                                Ok(a) => Ok(a),
                                Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(caller.with(|mut host|Host::convert_error_code(&mut host_getter(host.get()), e)))?),
                            },))
                        }.instrument(span))
                    })?;
                    inst.func_wrap_concurrent(
                        "[method]descriptor.metadata-hash-at",
                        move |caller: &wasmtime::component::Accessor<T>,
                              (arg0, arg1, arg2): (
                            wasmtime::component::Resource<Descriptor>,
                            PathFlags,
                            wasmtime::component::__internal::String,
                        )| {
                            use tracing::Instrument;
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "types",
                                function = "[method]descriptor.metadata-hash-at",
                            );
                            wasmtime::component::__internal::Box::pin(
                                async move {
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        self_ = tracing::field::debug(&arg0),
                                        path_flags = tracing::field::debug(&arg1),
                                        path = tracing::field::debug(&arg2),
                                        "call"
                                    );
                                    let host = &caller.with_getter::<D>(host_getter);
                                    let r = <D as HostDescriptorWithStore<T>>::metadata_hash_at(
                                        host, arg0, arg1, arg2,
                                    )
                                    .await;
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        result = tracing::field::debug(&r),
                                        "return"
                                    );
                                    Ok((match r {
                                        Ok(a) => Ok(a),
                                        Err(e) => {
                                            Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(
                                                caller.with(|mut host| {
                                                    Host::convert_error_code(
                                                        &mut host_getter(host.get()),
                                                        e,
                                                    )
                                                }),
                                            )?)
                                        }
                                    },))
                                }
                                .instrument(span),
                            )
                        },
                    )?;
                    Ok(())
                }
                pub fn add_to_linker<T, D>(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static + Send,
                {
                    let mut inst = linker.instance("wasi:filesystem/types@0.3.0")?;
                    add_to_linker_instance::<T, D>(&mut inst, host_getter)
                }
            }
            #[allow(clippy::all)]
            pub mod preopens {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::Box;
                pub type Descriptor = super::super::super::wasi::filesystem::types::Descriptor;
                pub trait HostWithStore<T>: wasmtime::component::HasData {}
                impl<H: ?Sized, T> HostWithStore<T> for H where H: wasmtime::component::HasData {}
                pub trait Host {
                    #[doc = "/ Return the set of preopened directories, and their paths."]
                    fn get_directories(
                        &mut self,
                    ) -> wasmtime::Result<
                        wasmtime::component::__internal::Vec<(
                            wasmtime::component::Resource<Descriptor>,
                            wasmtime::component::__internal::String,
                        )>,
                    >;
                }
                impl<_T: Host + ?Sized> Host for &mut _T {
                    #[doc = "/ Return the set of preopened directories, and their paths."]
                    fn get_directories(
                        &mut self,
                    ) -> wasmtime::Result<
                        wasmtime::component::__internal::Vec<(
                            wasmtime::component::Resource<Descriptor>,
                            wasmtime::component::__internal::String,
                        )>,
                    > {
                        Host::get_directories(*self)
                    }
                }
                pub fn add_to_linker_instance<T, D>(
                    inst: &mut wasmtime::component::LinkerInstance<'_, T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    inst.func_wrap(
                        "get-directories",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "preopens",
                                function = "get-directories",
                            );
                            let _enter = span.enter();
                            tracing::event!(tracing::Level::TRACE, "call");
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::get_directories(host);
                            tracing::event!(
                                tracing::Level::TRACE,
                                result = tracing::field::debug("..."),
                                "return"
                            );
                            Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                        },
                    )?;
                    Ok(())
                }
                pub fn add_to_linker<T, D>(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    let mut inst = linker.instance("wasi:filesystem/preopens@0.3.0")?;
                    add_to_linker_instance::<T, D>(&mut inst, host_getter)
                }
            }
        }
        pub mod random {
            #[allow(clippy::all)]
            pub mod random {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::Box;
                pub trait HostWithStore<T>: wasmtime::component::HasData {}
                impl<H: ?Sized, T> HostWithStore<T> for H where H: wasmtime::component::HasData {}
                pub trait Host {
                    #[doc = "/ Return up to `max-len` cryptographically-secure random or pseudo-random"]
                    #[doc = "/ bytes."]
                    #[doc = "/ "]
                    #[doc = "/ This function must produce data at least as cryptographically secure and"]
                    #[doc = "/ fast as an adequately seeded cryptographically-secure pseudo-random"]
                    #[doc = "/ number generator (CSPRNG). It must not block, from the perspective of"]
                    #[doc = "/ the calling program, under any circumstances, including on the first"]
                    #[doc = "/ request and on requests for numbers of bytes. The returned data must"]
                    #[doc = "/ always be unpredictable."]
                    #[doc = "/ "]
                    #[doc = "/ Implementations MAY return fewer bytes than requested (a short read)."]
                    #[doc = "/ Callers that require exactly `max-len` bytes MUST call this function in"]
                    #[doc = "/ a loop until the desired number of bytes has been accumulated."]
                    #[doc = "/ Implementations MUST return at least 1 byte when `max-len` is greater"]
                    #[doc = "/ than zero. When `max-len` is zero, implementations MUST return an empty"]
                    #[doc = "/ list without trapping."]
                    #[doc = "/ "]
                    #[doc = "/ This function must always return fresh data. Deterministic environments"]
                    #[doc = "/ must omit this function, rather than implementing it with deterministic"]
                    #[doc = "/ data."]
                    fn get_random_bytes(
                        &mut self,
                        max_len: u64,
                    ) -> wasmtime::Result<wasmtime::component::__internal::Vec<u8>>;

                    #[doc = "/ Return a cryptographically-secure random or pseudo-random `u64` value."]
                    #[doc = "/ "]
                    #[doc = "/ This function returns the same type of data as `get-random-bytes`,"]
                    #[doc = "/ represented as a `u64`."]
                    fn get_random_u64(&mut self) -> wasmtime::Result<u64>;
                }
                impl<_T: Host + ?Sized> Host for &mut _T {
                    #[doc = "/ Return up to `max-len` cryptographically-secure random or pseudo-random"]
                    #[doc = "/ bytes."]
                    #[doc = "/ "]
                    #[doc = "/ This function must produce data at least as cryptographically secure and"]
                    #[doc = "/ fast as an adequately seeded cryptographically-secure pseudo-random"]
                    #[doc = "/ number generator (CSPRNG). It must not block, from the perspective of"]
                    #[doc = "/ the calling program, under any circumstances, including on the first"]
                    #[doc = "/ request and on requests for numbers of bytes. The returned data must"]
                    #[doc = "/ always be unpredictable."]
                    #[doc = "/ "]
                    #[doc = "/ Implementations MAY return fewer bytes than requested (a short read)."]
                    #[doc = "/ Callers that require exactly `max-len` bytes MUST call this function in"]
                    #[doc = "/ a loop until the desired number of bytes has been accumulated."]
                    #[doc = "/ Implementations MUST return at least 1 byte when `max-len` is greater"]
                    #[doc = "/ than zero. When `max-len` is zero, implementations MUST return an empty"]
                    #[doc = "/ list without trapping."]
                    #[doc = "/ "]
                    #[doc = "/ This function must always return fresh data. Deterministic environments"]
                    #[doc = "/ must omit this function, rather than implementing it with deterministic"]
                    #[doc = "/ data."]
                    fn get_random_bytes(
                        &mut self,
                        max_len: u64,
                    ) -> wasmtime::Result<wasmtime::component::__internal::Vec<u8>>
                    {
                        Host::get_random_bytes(*self, max_len)
                    }
                    #[doc = "/ Return a cryptographically-secure random or pseudo-random `u64` value."]
                    #[doc = "/ "]
                    #[doc = "/ This function returns the same type of data as `get-random-bytes`,"]
                    #[doc = "/ represented as a `u64`."]
                    fn get_random_u64(&mut self) -> wasmtime::Result<u64> {
                        Host::get_random_u64(*self)
                    }
                }
                pub fn add_to_linker_instance<T, D>(
                    inst: &mut wasmtime::component::LinkerInstance<'_, T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    inst.func_wrap(
                        "get-random-bytes",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (u64,)| {
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "random",
                                function = "get-random-bytes",
                            );
                            let _enter = span.enter();
                            tracing::event!(
                                tracing::Level::TRACE,
                                max_len = tracing::field::debug(&arg0),
                                "call"
                            );
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::get_random_bytes(host, arg0);
                            tracing::event!(
                                tracing::Level::TRACE,
                                result = tracing::field::debug("..."),
                                "return"
                            );
                            Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                        },
                    )?;
                    inst.func_wrap(
                        "get-random-u64",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "random",
                                function = "get-random-u64",
                            );
                            let _enter = span.enter();
                            tracing::event!(tracing::Level::TRACE, "call");
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::get_random_u64(host);
                            tracing::event!(
                                tracing::Level::TRACE,
                                result = tracing::field::debug(&r),
                                "return"
                            );
                            Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                        },
                    )?;
                    Ok(())
                }
                pub fn add_to_linker<T, D>(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    let mut inst = linker.instance("wasi:random/random@0.3.0")?;
                    add_to_linker_instance::<T, D>(&mut inst, host_getter)
                }
            }
            #[allow(clippy::all)]
            pub mod insecure {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::Box;
                pub trait HostWithStore<T>: wasmtime::component::HasData {}
                impl<H: ?Sized, T> HostWithStore<T> for H where H: wasmtime::component::HasData {}
                pub trait Host {
                    #[doc = "/ Return up to `max-len` insecure pseudo-random bytes."]
                    #[doc = "/ "]
                    #[doc = "/ This function is not cryptographically secure. Do not use it for"]
                    #[doc = "/ anything related to security."]
                    #[doc = "/ "]
                    #[doc = "/ There are no requirements on the values of the returned bytes, however"]
                    #[doc = "/ implementations are encouraged to return evenly distributed values with"]
                    #[doc = "/ a long period."]
                    #[doc = "/ "]
                    #[doc = "/ Implementations MAY return fewer bytes than requested (a short read)."]
                    #[doc = "/ Callers that require exactly `max-len` bytes MUST call this function in"]
                    #[doc = "/ a loop until the desired number of bytes has been accumulated."]
                    #[doc = "/ Implementations MUST return at least 1 byte when `max-len` is greater"]
                    #[doc = "/ than zero. When `max-len` is zero, implementations MUST return an empty"]
                    #[doc = "/ list without trapping."]
                    fn get_insecure_random_bytes(
                        &mut self,
                        max_len: u64,
                    ) -> wasmtime::Result<wasmtime::component::__internal::Vec<u8>>;

                    #[doc = "/ Return an insecure pseudo-random `u64` value."]
                    #[doc = "/ "]
                    #[doc = "/ This function returns the same type of pseudo-random data as"]
                    #[doc = "/ `get-insecure-random-bytes`, represented as a `u64`."]
                    fn get_insecure_random_u64(&mut self) -> wasmtime::Result<u64>;
                }
                impl<_T: Host + ?Sized> Host for &mut _T {
                    #[doc = "/ Return up to `max-len` insecure pseudo-random bytes."]
                    #[doc = "/ "]
                    #[doc = "/ This function is not cryptographically secure. Do not use it for"]
                    #[doc = "/ anything related to security."]
                    #[doc = "/ "]
                    #[doc = "/ There are no requirements on the values of the returned bytes, however"]
                    #[doc = "/ implementations are encouraged to return evenly distributed values with"]
                    #[doc = "/ a long period."]
                    #[doc = "/ "]
                    #[doc = "/ Implementations MAY return fewer bytes than requested (a short read)."]
                    #[doc = "/ Callers that require exactly `max-len` bytes MUST call this function in"]
                    #[doc = "/ a loop until the desired number of bytes has been accumulated."]
                    #[doc = "/ Implementations MUST return at least 1 byte when `max-len` is greater"]
                    #[doc = "/ than zero. When `max-len` is zero, implementations MUST return an empty"]
                    #[doc = "/ list without trapping."]
                    fn get_insecure_random_bytes(
                        &mut self,
                        max_len: u64,
                    ) -> wasmtime::Result<wasmtime::component::__internal::Vec<u8>>
                    {
                        Host::get_insecure_random_bytes(*self, max_len)
                    }
                    #[doc = "/ Return an insecure pseudo-random `u64` value."]
                    #[doc = "/ "]
                    #[doc = "/ This function returns the same type of pseudo-random data as"]
                    #[doc = "/ `get-insecure-random-bytes`, represented as a `u64`."]
                    fn get_insecure_random_u64(&mut self) -> wasmtime::Result<u64> {
                        Host::get_insecure_random_u64(*self)
                    }
                }
                pub fn add_to_linker_instance<T, D>(
                    inst: &mut wasmtime::component::LinkerInstance<'_, T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    inst.func_wrap(
                        "get-insecure-random-bytes",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (u64,)| {
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "insecure",
                                function = "get-insecure-random-bytes",
                            );
                            let _enter = span.enter();
                            tracing::event!(
                                tracing::Level::TRACE,
                                max_len = tracing::field::debug(&arg0),
                                "call"
                            );
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::get_insecure_random_bytes(host, arg0);
                            tracing::event!(
                                tracing::Level::TRACE,
                                result = tracing::field::debug("..."),
                                "return"
                            );
                            Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                        },
                    )?;
                    inst.func_wrap(
                        "get-insecure-random-u64",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "insecure",
                                function = "get-insecure-random-u64",
                            );
                            let _enter = span.enter();
                            tracing::event!(tracing::Level::TRACE, "call");
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::get_insecure_random_u64(host);
                            tracing::event!(
                                tracing::Level::TRACE,
                                result = tracing::field::debug(&r),
                                "return"
                            );
                            Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                        },
                    )?;
                    Ok(())
                }
                pub fn add_to_linker<T, D>(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    let mut inst = linker.instance("wasi:random/insecure@0.3.0")?;
                    add_to_linker_instance::<T, D>(&mut inst, host_getter)
                }
            }
            #[allow(clippy::all)]
            pub mod insecure_seed {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::Box;
                pub trait HostWithStore<T>: wasmtime::component::HasData {}
                impl<H: ?Sized, T> HostWithStore<T> for H where H: wasmtime::component::HasData {}
                pub trait Host {
                    #[doc = "/ Return a 128-bit value that may contain a pseudo-random value."]
                    #[doc = "/ "]
                    #[doc = "/ The returned value is not required to be computed from a CSPRNG, and may"]
                    #[doc = "/ even be entirely deterministic. Host implementations are encouraged to"]
                    #[doc = "/ provide pseudo-random values to any program exposed to"]
                    #[doc = "/ attacker-controlled content, to enable DoS protection built into many"]
                    #[doc = "/ languages\' hash-map implementations."]
                    #[doc = "/ "]
                    #[doc = "/ This function is intended to only be called once, by a source language"]
                    #[doc = "/ to initialize Denial Of Service (DoS) protection in its hash-map"]
                    #[doc = "/ implementation."]
                    #[doc = "/ "]
                    #[doc = "/ # Expected future evolution"]
                    #[doc = "/ "]
                    #[doc = "/ This will likely be changed to a value import, to prevent it from being"]
                    #[doc = "/ called multiple times and potentially used for purposes other than DoS"]
                    #[doc = "/ protection."]
                    fn get_insecure_seed(&mut self) -> wasmtime::Result<(u64, u64)>;
                }
                impl<_T: Host + ?Sized> Host for &mut _T {
                    #[doc = "/ Return a 128-bit value that may contain a pseudo-random value."]
                    #[doc = "/ "]
                    #[doc = "/ The returned value is not required to be computed from a CSPRNG, and may"]
                    #[doc = "/ even be entirely deterministic. Host implementations are encouraged to"]
                    #[doc = "/ provide pseudo-random values to any program exposed to"]
                    #[doc = "/ attacker-controlled content, to enable DoS protection built into many"]
                    #[doc = "/ languages\' hash-map implementations."]
                    #[doc = "/ "]
                    #[doc = "/ This function is intended to only be called once, by a source language"]
                    #[doc = "/ to initialize Denial Of Service (DoS) protection in its hash-map"]
                    #[doc = "/ implementation."]
                    #[doc = "/ "]
                    #[doc = "/ # Expected future evolution"]
                    #[doc = "/ "]
                    #[doc = "/ This will likely be changed to a value import, to prevent it from being"]
                    #[doc = "/ called multiple times and potentially used for purposes other than DoS"]
                    #[doc = "/ protection."]
                    fn get_insecure_seed(&mut self) -> wasmtime::Result<(u64, u64)> {
                        Host::get_insecure_seed(*self)
                    }
                }
                pub fn add_to_linker_instance<T, D>(
                    inst: &mut wasmtime::component::LinkerInstance<'_, T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    inst.func_wrap(
                        "get-insecure-seed",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "insecure-seed",
                                function = "get-insecure-seed",
                            );
                            let _enter = span.enter();
                            tracing::event!(tracing::Level::TRACE, "call");
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::get_insecure_seed(host);
                            tracing::event!(
                                tracing::Level::TRACE,
                                result = tracing::field::debug(&r),
                                "return"
                            );
                            Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                        },
                    )?;
                    Ok(())
                }
                pub fn add_to_linker<T, D>(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static,
                {
                    let mut inst = linker.instance("wasi:random/insecure-seed@0.3.0")?;
                    add_to_linker_instance::<T, D>(&mut inst, host_getter)
                }
            }
        }
        pub mod sockets {
            #[allow(clippy::all)]
            pub mod types {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::Box;
                pub type Duration = super::super::super::wasi::clocks::types::Duration;
                const _: () = {
                    assert!(8 == <Duration as wasmtime::component::ComponentType>::SIZE32);
                    assert!(8 == <Duration as wasmtime::component::ComponentType>::ALIGN32);
                };
                #[doc = "/ Error codes."]
                #[doc = "/ "]
                #[doc = "/ In theory, every API can return any error code."]
                #[doc = "/ In practice, API\'s typically only return the errors documented per API"]
                #[doc = "/ combined with a couple of errors that are always possible:"]
                #[doc = "/ - `other`"]
                #[doc = "/ - `access-denied`"]
                #[doc = "/ - `not-supported`"]
                #[doc = "/ - `out-of-memory`"]
                #[doc = "/ "]
                #[doc = "/ See each individual API for what the POSIX equivalents are. They sometimes differ per API."]
                #[derive(
                    wasmtime::component::ComponentType,
                    wasmtime::component::Lift,
                    wasmtime::component::Lower,
                )]
                #[component(variant)]
                #[derive(Clone)]
                pub enum ErrorCode {
                    #[doc = "/ Access denied."]
                    #[doc = "/ "]
                    #[doc = "/ POSIX equivalent: EACCES, EPERM"]
                    #[component(name = "access-denied")]
                    AccessDenied,
                    #[doc = "/ The operation is not supported."]
                    #[doc = "/ "]
                    #[doc = "/ POSIX equivalent: EOPNOTSUPP, ENOPROTOOPT, EPFNOSUPPORT, EPROTONOSUPPORT, ESOCKTNOSUPPORT"]
                    #[component(name = "not-supported")]
                    NotSupported,
                    #[doc = "/ One of the arguments is invalid."]
                    #[doc = "/ "]
                    #[doc = "/ POSIX equivalent: EINVAL, EDESTADDRREQ, EAFNOSUPPORT"]
                    #[component(name = "invalid-argument")]
                    InvalidArgument,
                    #[doc = "/ Not enough memory to complete the operation."]
                    #[doc = "/ "]
                    #[doc = "/ POSIX equivalent: ENOMEM, ENOBUFS"]
                    #[component(name = "out-of-memory")]
                    OutOfMemory,
                    #[doc = "/ The operation timed out before it could finish completely."]
                    #[doc = "/ "]
                    #[doc = "/ POSIX equivalent: ETIMEDOUT"]
                    #[component(name = "timeout")]
                    Timeout,
                    #[doc = "/ The operation is not valid in the socket\'s current state."]
                    #[component(name = "invalid-state")]
                    InvalidState,
                    #[doc = "/ The local address is not available."]
                    #[doc = "/ "]
                    #[doc = "/ POSIX equivalent: EADDRNOTAVAIL"]
                    #[component(name = "address-not-bindable")]
                    AddressNotBindable,
                    #[doc = "/ A bind operation failed because the provided address is already in"]
                    #[doc = "/ use or because there are no ephemeral ports available."]
                    #[doc = "/ "]
                    #[doc = "/ POSIX equivalent: EADDRINUSE"]
                    #[component(name = "address-in-use")]
                    AddressInUse,
                    #[doc = "/ The remote address is not reachable."]
                    #[doc = "/ "]
                    #[doc = "/ POSIX equivalent: EHOSTUNREACH, EHOSTDOWN, ENETDOWN, ENETUNREACH, ENONET"]
                    #[component(name = "remote-unreachable")]
                    RemoteUnreachable,
                    #[doc = "/ The connection was forcefully rejected."]
                    #[doc = "/ "]
                    #[doc = "/ POSIX equivalent: ECONNREFUSED"]
                    #[component(name = "connection-refused")]
                    ConnectionRefused,
                    #[doc = "/ A write failed because the connection was broken."]
                    #[doc = "/ "]
                    #[doc = "/ POSIX equivalent: EPIPE"]
                    #[component(name = "connection-broken")]
                    ConnectionBroken,
                    #[doc = "/ The connection was reset."]
                    #[doc = "/ "]
                    #[doc = "/ POSIX equivalent: ECONNRESET"]
                    #[component(name = "connection-reset")]
                    ConnectionReset,
                    #[doc = "/ The connection was aborted."]
                    #[doc = "/ "]
                    #[doc = "/ POSIX equivalent: ECONNABORTED"]
                    #[component(name = "connection-aborted")]
                    ConnectionAborted,
                    #[doc = "/ The size of a datagram sent to a UDP socket exceeded the maximum"]
                    #[doc = "/ supported size."]
                    #[doc = "/ "]
                    #[doc = "/ POSIX equivalent: EMSGSIZE"]
                    #[component(name = "datagram-too-large")]
                    DatagramTooLarge,
                    #[doc = "/ A catch-all for errors not captured by the existing variants."]
                    #[doc = "/ Implementations can use this to extend the error type without"]
                    #[doc = "/ breaking existing code."]
                    #[component(name = "other")]
                    Other(Option<wasmtime::component::__internal::String>),
                }
                impl core::fmt::Debug for ErrorCode {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        match self {
                            ErrorCode::AccessDenied => {
                                f.debug_tuple("ErrorCode::AccessDenied").finish()
                            }
                            ErrorCode::NotSupported => {
                                f.debug_tuple("ErrorCode::NotSupported").finish()
                            }
                            ErrorCode::InvalidArgument => {
                                f.debug_tuple("ErrorCode::InvalidArgument").finish()
                            }
                            ErrorCode::OutOfMemory => {
                                f.debug_tuple("ErrorCode::OutOfMemory").finish()
                            }
                            ErrorCode::Timeout => f.debug_tuple("ErrorCode::Timeout").finish(),
                            ErrorCode::InvalidState => {
                                f.debug_tuple("ErrorCode::InvalidState").finish()
                            }
                            ErrorCode::AddressNotBindable => {
                                f.debug_tuple("ErrorCode::AddressNotBindable").finish()
                            }
                            ErrorCode::AddressInUse => {
                                f.debug_tuple("ErrorCode::AddressInUse").finish()
                            }
                            ErrorCode::RemoteUnreachable => {
                                f.debug_tuple("ErrorCode::RemoteUnreachable").finish()
                            }
                            ErrorCode::ConnectionRefused => {
                                f.debug_tuple("ErrorCode::ConnectionRefused").finish()
                            }
                            ErrorCode::ConnectionBroken => {
                                f.debug_tuple("ErrorCode::ConnectionBroken").finish()
                            }
                            ErrorCode::ConnectionReset => {
                                f.debug_tuple("ErrorCode::ConnectionReset").finish()
                            }
                            ErrorCode::ConnectionAborted => {
                                f.debug_tuple("ErrorCode::ConnectionAborted").finish()
                            }
                            ErrorCode::DatagramTooLarge => {
                                f.debug_tuple("ErrorCode::DatagramTooLarge").finish()
                            }
                            ErrorCode::Other(e) => {
                                f.debug_tuple("ErrorCode::Other").field(e).finish()
                            }
                        }
                    }
                }
                impl core::fmt::Display for ErrorCode {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(f, "{:?}", self)
                    }
                }
                impl core::error::Error for ErrorCode {}
                const _: () = {
                    assert!(16 == <ErrorCode as wasmtime::component::ComponentType>::SIZE32);
                    assert!(4 == <ErrorCode as wasmtime::component::ComponentType>::ALIGN32);
                };
                #[derive(
                    wasmtime::component::ComponentType,
                    wasmtime::component::Lift,
                    wasmtime::component::Lower,
                )]
                #[component(enum)]
                #[derive(Clone, Copy, Eq, PartialEq)]
                #[repr(u8)]
                pub enum IpAddressFamily {
                    #[doc = "/ Similar to `AF_INET` in POSIX."]
                    #[component(name = "ipv4")]
                    Ipv4,
                    #[doc = "/ Similar to `AF_INET6` in POSIX."]
                    #[component(name = "ipv6")]
                    Ipv6,
                }
                impl core::fmt::Debug for IpAddressFamily {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        match self {
                            IpAddressFamily::Ipv4 => {
                                f.debug_tuple("IpAddressFamily::Ipv4").finish()
                            }
                            IpAddressFamily::Ipv6 => {
                                f.debug_tuple("IpAddressFamily::Ipv6").finish()
                            }
                        }
                    }
                }
                const _: () = {
                    assert!(1 == <IpAddressFamily as wasmtime::component::ComponentType>::SIZE32);
                    assert!(1 == <IpAddressFamily as wasmtime::component::ComponentType>::ALIGN32);
                };
                pub type Ipv4Address = (u8, u8, u8, u8);
                const _: () = {
                    assert!(4 == <Ipv4Address as wasmtime::component::ComponentType>::SIZE32);
                    assert!(1 == <Ipv4Address as wasmtime::component::ComponentType>::ALIGN32);
                };
                pub type Ipv6Address = (u16, u16, u16, u16, u16, u16, u16, u16);
                const _: () = {
                    assert!(16 == <Ipv6Address as wasmtime::component::ComponentType>::SIZE32);
                    assert!(2 == <Ipv6Address as wasmtime::component::ComponentType>::ALIGN32);
                };
                #[derive(
                    wasmtime::component::ComponentType,
                    wasmtime::component::Lift,
                    wasmtime::component::Lower,
                )]
                #[component(variant)]
                #[derive(Clone, Copy)]
                pub enum IpAddress {
                    #[component(name = "ipv4")]
                    Ipv4(Ipv4Address),
                    #[component(name = "ipv6")]
                    Ipv6(Ipv6Address),
                }
                impl core::fmt::Debug for IpAddress {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        match self {
                            IpAddress::Ipv4(e) => {
                                f.debug_tuple("IpAddress::Ipv4").field(e).finish()
                            }
                            IpAddress::Ipv6(e) => {
                                f.debug_tuple("IpAddress::Ipv6").field(e).finish()
                            }
                        }
                    }
                }
                const _: () = {
                    assert!(18 == <IpAddress as wasmtime::component::ComponentType>::SIZE32);
                    assert!(2 == <IpAddress as wasmtime::component::ComponentType>::ALIGN32);
                };
                #[derive(
                    wasmtime::component::ComponentType,
                    wasmtime::component::Lift,
                    wasmtime::component::Lower,
                )]
                #[component(record)]
                #[derive(Clone, Copy)]
                pub struct Ipv4SocketAddress {
                    #[doc = "/ sin_port"]
                    #[component(name = "port")]
                    pub port: u16,
                    #[doc = "/ sin_addr"]
                    #[component(name = "address")]
                    pub address: Ipv4Address,
                }
                impl core::fmt::Debug for Ipv4SocketAddress {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.debug_struct("Ipv4SocketAddress")
                            .field("port", &self.port)
                            .field("address", &self.address)
                            .finish()
                    }
                }
                const _: () = {
                    assert!(6 == <Ipv4SocketAddress as wasmtime::component::ComponentType>::SIZE32);
                    assert!(
                        2 == <Ipv4SocketAddress as wasmtime::component::ComponentType>::ALIGN32
                    );
                };
                #[derive(
                    wasmtime::component::ComponentType,
                    wasmtime::component::Lift,
                    wasmtime::component::Lower,
                )]
                #[component(record)]
                #[derive(Clone, Copy)]
                pub struct Ipv6SocketAddress {
                    #[doc = "/ sin6_port"]
                    #[component(name = "port")]
                    pub port: u16,
                    #[doc = "/ sin6_flowinfo"]
                    #[component(name = "flow-info")]
                    pub flow_info: u32,
                    #[doc = "/ sin6_addr"]
                    #[component(name = "address")]
                    pub address: Ipv6Address,
                    #[doc = "/ sin6_scope_id"]
                    #[component(name = "scope-id")]
                    pub scope_id: u32,
                }
                impl core::fmt::Debug for Ipv6SocketAddress {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.debug_struct("Ipv6SocketAddress")
                            .field("port", &self.port)
                            .field("flow-info", &self.flow_info)
                            .field("address", &self.address)
                            .field("scope-id", &self.scope_id)
                            .finish()
                    }
                }
                const _: () = {
                    assert!(
                        28 == <Ipv6SocketAddress as wasmtime::component::ComponentType>::SIZE32
                    );
                    assert!(
                        4 == <Ipv6SocketAddress as wasmtime::component::ComponentType>::ALIGN32
                    );
                };
                #[derive(
                    wasmtime::component::ComponentType,
                    wasmtime::component::Lift,
                    wasmtime::component::Lower,
                )]
                #[component(variant)]
                #[derive(Clone, Copy)]
                pub enum IpSocketAddress {
                    #[component(name = "ipv4")]
                    Ipv4(Ipv4SocketAddress),
                    #[component(name = "ipv6")]
                    Ipv6(Ipv6SocketAddress),
                }
                impl core::fmt::Debug for IpSocketAddress {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        match self {
                            IpSocketAddress::Ipv4(e) => {
                                f.debug_tuple("IpSocketAddress::Ipv4").field(e).finish()
                            }
                            IpSocketAddress::Ipv6(e) => {
                                f.debug_tuple("IpSocketAddress::Ipv6").field(e).finish()
                            }
                        }
                    }
                }
                const _: () = {
                    assert!(32 == <IpSocketAddress as wasmtime::component::ComponentType>::SIZE32);
                    assert!(4 == <IpSocketAddress as wasmtime::component::ComponentType>::ALIGN32);
                };
                #[doc = "/ A TCP socket resource."]
                #[doc = "/ "]
                #[doc = "/ The socket can be in one of the following states:"]
                #[doc = "/ - `unbound`"]
                #[doc = "/ - `bound` (See note below)"]
                #[doc = "/ - `listening`"]
                #[doc = "/ - `connecting`"]
                #[doc = "/ - `connected`"]
                #[doc = "/ - `closed`"]
                #[doc = "/ See <https://github.com/WebAssembly/WASI/blob/main/proposals/sockets/TcpSocketOperationalSemantics-0.3.0.md>"]
                #[doc = "/ for more information."]
                #[doc = "/ "]
                #[doc = "/ Note: Except where explicitly mentioned, whenever this documentation uses"]
                #[doc = "/ the term \"bound\" without backticks it actually means: in the `bound` state *or higher*."]
                #[doc = "/ (i.e. `bound`, `listening`, `connecting` or `connected`)"]
                #[doc = "/ "]
                #[doc = "/ WASI uses shared ownership semantics: the `tcp-socket` handle and all"]
                #[doc = "/ derived `stream` and `future` values reference a single underlying OS"]
                #[doc = "/ socket:"]
                #[doc = "/ - Send/receive streams remain functional after the original `tcp-socket`"]
                #[doc = "/   handle is dropped."]
                #[doc = "/ - The stream returned by `listen` behaves similarly."]
                #[doc = "/ - Client sockets returned by `tcp-socket::listen` are independent and do"]
                #[doc = "/   not keep the listening socket alive."]
                #[doc = "/ "]
                #[doc = "/ The OS socket is closed only after the last handle is dropped. This"]
                #[doc = "/ model has observable effects; for example, it affects when the local"]
                #[doc = "/ port binding is released."]
                #[doc = "/ "]
                #[doc = "/ In addition to the general error codes documented on the"]
                #[doc = "/ `types::error-code` type, TCP socket methods may always return"]
                #[doc = "/ `error(invalid-state)` when in the `closed` state."]
                pub use super::super::super::__with_name3 as TcpSocket;
                pub trait HostTcpSocketWithStore<T>: wasmtime::component::HasData + Send {
                    #[doc = "/ Connect to a remote endpoint."]
                    #[doc = "/ "]
                    #[doc = "/ On success, the socket is transitioned into the `connected` state"]
                    #[doc = "/ and the `remote-address` of the socket is updated."]
                    #[doc = "/ The `local-address` may be updated as well, based on the best network"]
                    #[doc = "/ path to `remote-address`. If the socket was not already explicitly"]
                    #[doc = "/ bound, this function will implicitly bind the socket to a random"]
                    #[doc = "/ free port."]
                    #[doc = "/ "]
                    #[doc = "/ After a failed connection attempt, the socket will be in the `closed`"]
                    #[doc = "/ state and the only valid action left is to `drop` the socket. A single"]
                    #[doc = "/ socket can not be used to connect more than once."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-argument`:          The `remote-address` has the wrong address family. (EAFNOSUPPORT)"]
                    #[doc = "/ - `invalid-argument`:          `remote-address` is not a unicast address. (EINVAL, ENETUNREACH on Linux, EAFNOSUPPORT on MacOS)"]
                    #[doc = "/ - `invalid-argument`:          `remote-address` is an IPv4-mapped IPv6 address. (EINVAL, EADDRNOTAVAIL on Illumos)"]
                    #[doc = "/ - `invalid-argument`:          The IP address in `remote-address` is set to INADDR_ANY (`0.0.0.0` / `::`). (EADDRNOTAVAIL on Windows)"]
                    #[doc = "/ - `invalid-argument`:          The port in `remote-address` is set to 0. (EADDRNOTAVAIL on Windows)"]
                    #[doc = "/ - `invalid-state`:             The socket is already in the `connecting` state. (EALREADY)"]
                    #[doc = "/ - `invalid-state`:             The socket is already in the `connected` state. (EISCONN)"]
                    #[doc = "/ - `invalid-state`:             The socket is already in the `listening` state. (EOPNOTSUPP, EINVAL on Windows)"]
                    #[doc = "/ - `timeout`:                   Connection timed out. (ETIMEDOUT)"]
                    #[doc = "/ - `connection-refused`:        The connection was forcefully rejected. (ECONNREFUSED)"]
                    #[doc = "/ - `connection-reset`:          The connection was reset. (ECONNRESET)"]
                    #[doc = "/ - `connection-aborted`:        The connection was aborted. (ECONNABORTED)"]
                    #[doc = "/ - `remote-unreachable`:        The remote address is not reachable. (EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)"]
                    #[doc = "/ - `address-in-use`:            Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE, EADDRNOTAVAIL on Linux, EAGAIN on BSD)"]
                    #[doc = "/ "]
                    #[doc = "/ # References"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/connect.2.html>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-connect>"]
                    #[doc = "/ - <https://man.freebsd.org/cgi/man.cgi?connect>"]
                    fn connect(
                        accessor: &wasmtime::component::Accessor<T, Self>,
                        self_: wasmtime::component::Resource<TcpSocket>,
                        remote_address: IpSocketAddress,
                    ) -> impl ::core::future::Future<
                        Output = Result<(), super::super::super::_TrappableError1>,
                    > + Send;

                    #[doc = "/ Start listening and return a stream of new inbound connections."]
                    #[doc = "/ "]
                    #[doc = "/ Transitions the socket into the `listening` state. This can be called"]
                    #[doc = "/ at most once per socket."]
                    #[doc = "/ "]
                    #[doc = "/ If the socket is not already explicitly bound, this function will"]
                    #[doc = "/ implicitly bind the socket to a random free port."]
                    #[doc = "/ "]
                    #[doc = "/ Normally, the returned sockets are bound, in the `connected` state"]
                    #[doc = "/ and immediately ready for I/O. Though, depending on exact timing and"]
                    #[doc = "/ circumstances, a newly accepted connection may already be `closed`"]
                    #[doc = "/ by the time the server attempts to perform its first I/O on it. This"]
                    #[doc = "/ is true regardless of whether the WASI implementation uses"]
                    #[doc = "/ \"synthesized\" sockets or not (see Implementors Notes below)."]
                    #[doc = "/ "]
                    #[doc = "/ The following properties are inherited from the listener socket:"]
                    #[doc = "/ - `address-family`"]
                    #[doc = "/ - `keep-alive-enabled`"]
                    #[doc = "/ - `keep-alive-idle-time`"]
                    #[doc = "/ - `keep-alive-interval`"]
                    #[doc = "/ - `keep-alive-count`"]
                    #[doc = "/ - `hop-limit`"]
                    #[doc = "/ - `receive-buffer-size`"]
                    #[doc = "/ - `send-buffer-size`"]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-state`:             The socket is already in the `connected` state. (EISCONN, EINVAL on BSD)"]
                    #[doc = "/ - `invalid-state`:             The socket is already in the `listening` state."]
                    #[doc = "/ - `address-in-use`:            Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE)"]
                    #[doc = "/ "]
                    #[doc = "/ # Implementors note"]
                    #[doc = "/ This method returns a single perpetual stream that should only close"]
                    #[doc = "/ on fatal errors (if any). Yet, the POSIX\' `accept` function may also"]
                    #[doc = "/ return transient errors (e.g. ECONNABORTED). The exact details differ"]
                    #[doc = "/ per operation system. For example, the Linux manual mentions:"]
                    #[doc = "/ "]
                    #[doc = "/ > Linux accept() passes already-pending network errors on the new"]
                    #[doc = "/ > socket as an error code from accept(). This behavior differs from"]
                    #[doc = "/ > other BSD socket implementations. For reliable operation the"]
                    #[doc = "/ > application should detect the network errors defined for the"]
                    #[doc = "/ > protocol after accept() and treat them like EAGAIN by retrying."]
                    #[doc = "/ > In the case of TCP/IP, these are ENETDOWN, EPROTO, ENOPROTOOPT,"]
                    #[doc = "/ > EHOSTDOWN, ENONET, EHOSTUNREACH, EOPNOTSUPP, and ENETUNREACH."]
                    #[doc = "/ Source: https://man7.org/linux/man-pages/man2/accept.2.html"]
                    #[doc = "/ "]
                    #[doc = "/ WASI implementations have two options to handle this:"]
                    #[doc = "/ - Optionally log it and then skip over non-fatal errors returned by"]
                    #[doc = "/   `accept`. Guest code never gets to see these failures. Or:"]
                    #[doc = "/ - Synthesize a `tcp-socket` resource that exposes the error when"]
                    #[doc = "/   attempting to send or receive on it. Guest code then sees these"]
                    #[doc = "/   failures as regular I/O errors."]
                    #[doc = "/ "]
                    #[doc = "/ In either case, the stream returned by this `listen` method remains"]
                    #[doc = "/ operational."]
                    #[doc = "/ "]
                    #[doc = "/ WASI requires `listen` to perform an implicit bind if the socket"]
                    #[doc = "/ has not already been bound. Not all platforms (notably Windows)"]
                    #[doc = "/ exhibit this behavior out of the box. On platforms that require it,"]
                    #[doc = "/ the WASI implementation can emulate this behavior by performing"]
                    #[doc = "/ the bind itself if the guest hasn\'t already done so."]
                    #[doc = "/ "]
                    #[doc = "/ # References"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/listen.html>"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/accept.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/listen.2.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/accept.2.html>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-listen>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-accept>"]
                    #[doc = "/ - <https://man.freebsd.org/cgi/man.cgi?query=listen&sektion=2>"]
                    #[doc = "/ - <https://man.freebsd.org/cgi/man.cgi?query=accept&sektion=2>"]
                    fn listen(
                        host: wasmtime::component::Access<T, Self>,
                        self_: wasmtime::component::Resource<TcpSocket>,
                    ) -> impl ::core::future::Future<
                        Output = Result<
                            wasmtime::component::StreamReader<
                                wasmtime::component::Resource<TcpSocket>,
                            >,
                            super::super::super::_TrappableError1,
                        >,
                    > + Send;

                    #[doc = "/ Transmit data to peer."]
                    #[doc = "/ "]
                    #[doc = "/ The caller should close the stream when it has no more data to send"]
                    #[doc = "/ to the peer. Under normal circumstances this will cause a FIN packet"]
                    #[doc = "/ to be sent out. Closing the stream is equivalent to calling"]
                    #[doc = "/ `shutdown(SHUT_WR)` in POSIX."]
                    #[doc = "/ "]
                    #[doc = "/ This function may be called at most once and returns once the full"]
                    #[doc = "/ contents of the stream are transmitted or an error is encountered."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-state`:             The socket is not in the `connected` state. (ENOTCONN)"]
                    #[doc = "/ - `invalid-state`:             `send` has already been called on this socket."]
                    #[doc = "/ - `connection-broken`:         The connection is not writable anymore. (EPIPE, ECONNABORTED on Windows)"]
                    #[doc = "/ - `connection-reset`:          The connection was reset. (ECONNRESET)"]
                    #[doc = "/ - `remote-unreachable`:        The remote address is not reachable. (EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)"]
                    #[doc = "/ "]
                    #[doc = "/  # References"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/send.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/send.2.html>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-send>"]
                    #[doc = "/ - <https://man.freebsd.org/cgi/man.cgi?query=send&sektion=2>"]
                    fn send(
                        host: wasmtime::component::Access<T, Self>,
                        self_: wasmtime::component::Resource<TcpSocket>,
                        data: wasmtime::component::StreamReader<u8>,
                    ) -> wasmtime::Result<wasmtime::component::FutureReader<Result<(), ErrorCode>>>;

                    #[doc = "/ Read data from peer."]
                    #[doc = "/ "]
                    #[doc = "/ Returns a `stream` of data sent by the peer. The implementation"]
                    #[doc = "/ drops the stream once no more data is available. At that point, the"]
                    #[doc = "/ returned `future` resolves to:"]
                    #[doc = "/ - `ok` after a graceful shutdown from the peer (i.e. a FIN packet), or"]
                    #[doc = "/ - `err` if the socket was closed abnormally."]
                    #[doc = "/ "]
                    #[doc = "/ `receive` may be called only once per socket. Subsequent calls return"]
                    #[doc = "/ a closed stream and a future resolved to `err(invalid-state)`."]
                    #[doc = "/ "]
                    #[doc = "/ If the caller is not expecting to receive any more data from the peer,"]
                    #[doc = "/ they should drop the stream. Any data still in the receive queue"]
                    #[doc = "/ will be discarded. This is equivalent to calling `shutdown(SHUT_RD)`"]
                    #[doc = "/ in POSIX."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-state`:             The socket is not in the `connected` state. (ENOTCONN)"]
                    #[doc = "/ - `invalid-state`:             `receive` has already been called on this socket."]
                    #[doc = "/ - `connection-reset`:          The connection was reset. (ECONNRESET)"]
                    #[doc = "/ - `remote-unreachable`:        The remote address is not reachable. (EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)"]
                    #[doc = "/ "]
                    #[doc = "/ # References"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/recv.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/recv.2.html>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-recv>"]
                    #[doc = "/ - <https://man.freebsd.org/cgi/man.cgi?query=recv&sektion=2>"]
                    fn receive(
                        host: wasmtime::component::Access<T, Self>,
                        self_: wasmtime::component::Resource<TcpSocket>,
                    ) -> wasmtime::Result<(
                        wasmtime::component::StreamReader<u8>,
                        wasmtime::component::FutureReader<Result<(), ErrorCode>>,
                    )>;
                }
                pub trait HostTcpSocket: Send {
                    #[doc = "/ Create a new TCP socket."]
                    #[doc = "/ "]
                    #[doc = "/ Similar to `socket(AF_INET or AF_INET6, SOCK_STREAM, IPPROTO_TCP)`"]
                    #[doc = "/ in POSIX. On IPv6 sockets, IPV6_V6ONLY is enabled by default and"]
                    #[doc = "/ can\'t be configured otherwise."]
                    #[doc = "/ "]
                    #[doc = "/ Unlike POSIX, WASI sockets have no notion of a socket-level"]
                    #[doc = "/ `O_NONBLOCK` flag. Instead they fully rely on the Component Model\'s"]
                    #[doc = "/ async support."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `not-supported`: The `address-family` is not supported. (EAFNOSUPPORT)"]
                    #[doc = "/ "]
                    #[doc = "/ # References"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/socket.2.html>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasocketw>"]
                    #[doc = "/ - <https://man.freebsd.org/cgi/man.cgi?query=socket&sektion=2>"]
                    fn create(
                        &mut self,
                        address_family: IpAddressFamily,
                    ) -> Result<
                        wasmtime::component::Resource<TcpSocket>,
                        super::super::super::_TrappableError1,
                    >;

                    #[doc = "/ Bind the socket to the provided IP address and port."]
                    #[doc = "/ "]
                    #[doc = "/ If the IP address is zero (`0.0.0.0` in IPv4, `::` in IPv6), it is"]
                    #[doc = "/ left to the implementation to decide which network interface(s) to"]
                    #[doc = "/ bind to. If the TCP/UDP port is zero, the socket will be bound to a"]
                    #[doc = "/ random free port."]
                    #[doc = "/ "]
                    #[doc = "/ Bind can be attempted multiple times on the same socket, even with"]
                    #[doc = "/ different arguments on each iteration. But never concurrently and"]
                    #[doc = "/ only as long as the previous bind failed. Once a bind succeeds, the"]
                    #[doc = "/ binding can\'t be changed anymore."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-argument`:          The `local-address` has the wrong address family. (EAFNOSUPPORT, EFAULT on Windows)"]
                    #[doc = "/ - `invalid-argument`:          `local-address` is not a unicast address. (EINVAL)"]
                    #[doc = "/ - `invalid-argument`:          `local-address` is an IPv4-mapped IPv6 address. (EINVAL)"]
                    #[doc = "/ - `invalid-state`:             The socket is already bound. (EINVAL)"]
                    #[doc = "/ - `address-in-use`:            No ephemeral ports available. (EADDRINUSE, ENOBUFS on Windows)"]
                    #[doc = "/ - `address-in-use`:            Address is already in use. (EADDRINUSE)"]
                    #[doc = "/ - `address-not-bindable`:      `local-address` is not an address that can be bound to. (EADDRNOTAVAIL)"]
                    #[doc = "/ "]
                    #[doc = "/ # Implementors note"]
                    #[doc = "/ The bind operation shouldn\'t be affected by the TIME_WAIT state of a"]
                    #[doc = "/ recently closed socket on the same local address. In practice this"]
                    #[doc = "/ means that the SO_REUSEADDR socket option should be set implicitly"]
                    #[doc = "/ on all platforms, except on Windows where this is the default"]
                    #[doc = "/ behavior and SO_REUSEADDR performs something different."]
                    #[doc = "/ "]
                    #[doc = "/ # References"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/bind.2.html>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-bind>"]
                    #[doc = "/ - <https://man.freebsd.org/cgi/man.cgi?query=bind&sektion=2&format=html>"]
                    fn bind(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                        local_address: IpSocketAddress,
                    ) -> impl ::core::future::Future<
                        Output = Result<(), super::super::super::_TrappableError1>,
                    > + Send;

                    #[doc = "/ Get the bound local address."]
                    #[doc = "/ "]
                    #[doc = "/ POSIX mentions:"]
                    #[doc = "/ > If the socket has not been bound to a local name, the value"]
                    #[doc = "/ > stored in the object pointed to by `address` is unspecified."]
                    #[doc = "/ "]
                    #[doc = "/ WASI is stricter and requires `get-local-address` to return"]
                    #[doc = "/ `invalid-state` when the socket hasn\'t been bound yet."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-state`: The socket is not bound to any local address."]
                    #[doc = "/ "]
                    #[doc = "/ # References"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockname.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/getsockname.2.html>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getsockname>"]
                    #[doc = "/ - <https://man.freebsd.org/cgi/man.cgi?getsockname>"]
                    fn get_local_address(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                    ) -> Result<IpSocketAddress, super::super::super::_TrappableError1>;

                    #[doc = "/ Get the remote address."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-state`: The socket is not connected to a remote address. (ENOTCONN)"]
                    #[doc = "/ "]
                    #[doc = "/ # References"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpeername.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/getpeername.2.html>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getpeername>"]
                    #[doc = "/ - <https://man.freebsd.org/cgi/man.cgi?query=getpeername&sektion=2&n=1>"]
                    fn get_remote_address(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                    ) -> Result<IpSocketAddress, super::super::super::_TrappableError1>;

                    #[doc = "/ Whether the socket is in the `listening` state."]
                    #[doc = "/ "]
                    #[doc = "/ Equivalent to the SO_ACCEPTCONN socket option."]
                    fn get_is_listening(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                    ) -> wasmtime::Result<bool>;

                    #[doc = "/ Whether this is a IPv4 or IPv6 socket."]
                    #[doc = "/ "]
                    #[doc = "/ This is the value passed to the constructor."]
                    #[doc = "/ "]
                    #[doc = "/ Equivalent to the SO_DOMAIN socket option."]
                    fn get_address_family(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                    ) -> wasmtime::Result<IpAddressFamily>;

                    #[doc = "/ Hints the desired listen queue size. Implementations are free to"]
                    #[doc = "/ ignore this."]
                    #[doc = "/ "]
                    #[doc = "/ If the provided value is 0, an `invalid-argument` error is returned."]
                    #[doc = "/ Any other value will never cause an error, but it might be silently"]
                    #[doc = "/ clamped and/or rounded."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `not-supported`:        (set) The platform does not support changing the backlog size after the initial listen."]
                    #[doc = "/ - `invalid-argument`:     (set) The provided value was 0."]
                    #[doc = "/ - `invalid-state`:        (set) The socket is in the `connecting` or `connected` state."]
                    fn set_listen_backlog_size(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                        value: u64,
                    ) -> Result<(), super::super::super::_TrappableError1>;

                    #[doc = "/ Enables or disables keepalive."]
                    #[doc = "/ "]
                    #[doc = "/ The keepalive behavior can be adjusted using:"]
                    #[doc = "/ - `keep-alive-idle-time`"]
                    #[doc = "/ - `keep-alive-interval`"]
                    #[doc = "/ - `keep-alive-count`"]
                    #[doc = "/ These properties can be configured while `keep-alive-enabled` is"]
                    #[doc = "/ false, but only come into effect when `keep-alive-enabled` is true."]
                    #[doc = "/ "]
                    #[doc = "/ Equivalent to the SO_KEEPALIVE socket option."]
                    fn get_keep_alive_enabled(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                    ) -> Result<bool, super::super::super::_TrappableError1>;

                    fn set_keep_alive_enabled(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                        value: bool,
                    ) -> Result<(), super::super::super::_TrappableError1>;

                    #[doc = "/ Amount of time the connection has to be idle before TCP starts"]
                    #[doc = "/ sending keepalive packets."]
                    #[doc = "/ "]
                    #[doc = "/ If the provided value is 0, an `invalid-argument` error is returned."]
                    #[doc = "/ All other values are accepted without error, but may be"]
                    #[doc = "/ clamped or rounded. As a result, the value read back from"]
                    #[doc = "/ this setting may differ from the value that was set."]
                    #[doc = "/ "]
                    #[doc = "/ Equivalent to the TCP_KEEPIDLE socket option. (TCP_KEEPALIVE on MacOS)"]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-argument`:     (set) The provided value was 0."]
                    fn get_keep_alive_idle_time(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                    ) -> Result<Duration, super::super::super::_TrappableError1>;

                    fn set_keep_alive_idle_time(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                        value: Duration,
                    ) -> Result<(), super::super::super::_TrappableError1>;

                    #[doc = "/ The time between keepalive packets."]
                    #[doc = "/ "]
                    #[doc = "/ If the provided value is 0, an `invalid-argument` error is returned."]
                    #[doc = "/ All other values are accepted without error, but may be"]
                    #[doc = "/ clamped or rounded. As a result, the value read back from"]
                    #[doc = "/ this setting may differ from the value that was set."]
                    #[doc = "/ "]
                    #[doc = "/ Equivalent to the TCP_KEEPINTVL socket option."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-argument`:     (set) The provided value was 0."]
                    fn get_keep_alive_interval(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                    ) -> Result<Duration, super::super::super::_TrappableError1>;

                    fn set_keep_alive_interval(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                        value: Duration,
                    ) -> Result<(), super::super::super::_TrappableError1>;

                    #[doc = "/ The maximum amount of keepalive packets TCP should send before"]
                    #[doc = "/ aborting the connection."]
                    #[doc = "/ "]
                    #[doc = "/ If the provided value is 0, an `invalid-argument` error is returned."]
                    #[doc = "/ All other values are accepted without error, but may be"]
                    #[doc = "/ clamped or rounded. As a result, the value read back from"]
                    #[doc = "/ this setting may differ from the value that was set."]
                    #[doc = "/ "]
                    #[doc = "/ Equivalent to the TCP_KEEPCNT socket option."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-argument`:     (set) The provided value was 0."]
                    fn get_keep_alive_count(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                    ) -> Result<u32, super::super::super::_TrappableError1>;

                    fn set_keep_alive_count(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                        value: u32,
                    ) -> Result<(), super::super::super::_TrappableError1>;

                    #[doc = "/ Equivalent to the IP_TTL & IPV6_UNICAST_HOPS socket options."]
                    #[doc = "/ "]
                    #[doc = "/ If the provided value is 0, an `invalid-argument` error is returned."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-argument`:     (set) The TTL value must be 1 or higher."]
                    fn get_hop_limit(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                    ) -> Result<u8, super::super::super::_TrappableError1>;

                    fn set_hop_limit(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                        value: u8,
                    ) -> Result<(), super::super::super::_TrappableError1>;

                    #[doc = "/ Kernel buffer space reserved for sending/receiving on this socket."]
                    #[doc = "/ Implementations usually treat this as a cap the buffer can grow to,"]
                    #[doc = "/ rather than allocating the full amount immediately."]
                    #[doc = "/ "]
                    #[doc = "/ If the provided value is 0, an `invalid-argument` error is returned."]
                    #[doc = "/ All other values are accepted without error, but may be"]
                    #[doc = "/ clamped or rounded. As a result, the value read back from"]
                    #[doc = "/ this setting may differ from the value that was set."]
                    #[doc = "/ "]
                    #[doc = "/ This is only a performance hint. The implementation may ignore it or"]
                    #[doc = "/ tweak it based on real traffic patterns."]
                    #[doc = "/ Linux and macOS appear to behave differently depending on whether a"]
                    #[doc = "/ buffer size was explicitly set. When set, they tend to honor it; when"]
                    #[doc = "/ not set, they dynamically adjust the buffer size as the connection"]
                    #[doc = "/ progresses. This is especially noticeable when comparing the values"]
                    #[doc = "/ from before and after connection establishment."]
                    #[doc = "/ "]
                    #[doc = "/ Equivalent to the SO_RCVBUF and SO_SNDBUF socket options."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-argument`:     (set) The provided value was 0."]
                    fn get_receive_buffer_size(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                    ) -> Result<u64, super::super::super::_TrappableError1>;

                    fn set_receive_buffer_size(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                        value: u64,
                    ) -> Result<(), super::super::super::_TrappableError1>;

                    fn get_send_buffer_size(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                    ) -> Result<u64, super::super::super::_TrappableError1>;

                    fn set_send_buffer_size(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                        value: u64,
                    ) -> Result<(), super::super::super::_TrappableError1>;

                    fn drop(
                        &mut self,
                        rep: wasmtime::component::Resource<TcpSocket>,
                    ) -> wasmtime::Result<()>;
                }
                impl<_T: HostTcpSocket + ?Sized + Send> HostTcpSocket for &mut _T {
                    #[doc = "/ Create a new TCP socket."]
                    #[doc = "/ "]
                    #[doc = "/ Similar to `socket(AF_INET or AF_INET6, SOCK_STREAM, IPPROTO_TCP)`"]
                    #[doc = "/ in POSIX. On IPv6 sockets, IPV6_V6ONLY is enabled by default and"]
                    #[doc = "/ can\'t be configured otherwise."]
                    #[doc = "/ "]
                    #[doc = "/ Unlike POSIX, WASI sockets have no notion of a socket-level"]
                    #[doc = "/ `O_NONBLOCK` flag. Instead they fully rely on the Component Model\'s"]
                    #[doc = "/ async support."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `not-supported`: The `address-family` is not supported. (EAFNOSUPPORT)"]
                    #[doc = "/ "]
                    #[doc = "/ # References"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/socket.2.html>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasocketw>"]
                    #[doc = "/ - <https://man.freebsd.org/cgi/man.cgi?query=socket&sektion=2>"]
                    fn create(
                        &mut self,
                        address_family: IpAddressFamily,
                    ) -> Result<
                        wasmtime::component::Resource<TcpSocket>,
                        super::super::super::_TrappableError1,
                    > {
                        HostTcpSocket::create(*self, address_family)
                    }
                    #[doc = "/ Bind the socket to the provided IP address and port."]
                    #[doc = "/ "]
                    #[doc = "/ If the IP address is zero (`0.0.0.0` in IPv4, `::` in IPv6), it is"]
                    #[doc = "/ left to the implementation to decide which network interface(s) to"]
                    #[doc = "/ bind to. If the TCP/UDP port is zero, the socket will be bound to a"]
                    #[doc = "/ random free port."]
                    #[doc = "/ "]
                    #[doc = "/ Bind can be attempted multiple times on the same socket, even with"]
                    #[doc = "/ different arguments on each iteration. But never concurrently and"]
                    #[doc = "/ only as long as the previous bind failed. Once a bind succeeds, the"]
                    #[doc = "/ binding can\'t be changed anymore."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-argument`:          The `local-address` has the wrong address family. (EAFNOSUPPORT, EFAULT on Windows)"]
                    #[doc = "/ - `invalid-argument`:          `local-address` is not a unicast address. (EINVAL)"]
                    #[doc = "/ - `invalid-argument`:          `local-address` is an IPv4-mapped IPv6 address. (EINVAL)"]
                    #[doc = "/ - `invalid-state`:             The socket is already bound. (EINVAL)"]
                    #[doc = "/ - `address-in-use`:            No ephemeral ports available. (EADDRINUSE, ENOBUFS on Windows)"]
                    #[doc = "/ - `address-in-use`:            Address is already in use. (EADDRINUSE)"]
                    #[doc = "/ - `address-not-bindable`:      `local-address` is not an address that can be bound to. (EADDRNOTAVAIL)"]
                    #[doc = "/ "]
                    #[doc = "/ # Implementors note"]
                    #[doc = "/ The bind operation shouldn\'t be affected by the TIME_WAIT state of a"]
                    #[doc = "/ recently closed socket on the same local address. In practice this"]
                    #[doc = "/ means that the SO_REUSEADDR socket option should be set implicitly"]
                    #[doc = "/ on all platforms, except on Windows where this is the default"]
                    #[doc = "/ behavior and SO_REUSEADDR performs something different."]
                    #[doc = "/ "]
                    #[doc = "/ # References"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/bind.2.html>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-bind>"]
                    #[doc = "/ - <https://man.freebsd.org/cgi/man.cgi?query=bind&sektion=2&format=html>"]
                    fn bind(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                        local_address: IpSocketAddress,
                    ) -> impl ::core::future::Future<
                        Output = Result<(), super::super::super::_TrappableError1>,
                    > + Send {
                        async move { HostTcpSocket::bind(*self, self_, local_address).await }
                    }
                    #[doc = "/ Get the bound local address."]
                    #[doc = "/ "]
                    #[doc = "/ POSIX mentions:"]
                    #[doc = "/ > If the socket has not been bound to a local name, the value"]
                    #[doc = "/ > stored in the object pointed to by `address` is unspecified."]
                    #[doc = "/ "]
                    #[doc = "/ WASI is stricter and requires `get-local-address` to return"]
                    #[doc = "/ `invalid-state` when the socket hasn\'t been bound yet."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-state`: The socket is not bound to any local address."]
                    #[doc = "/ "]
                    #[doc = "/ # References"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockname.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/getsockname.2.html>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getsockname>"]
                    #[doc = "/ - <https://man.freebsd.org/cgi/man.cgi?getsockname>"]
                    fn get_local_address(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                    ) -> Result<IpSocketAddress, super::super::super::_TrappableError1>
                    {
                        HostTcpSocket::get_local_address(*self, self_)
                    }
                    #[doc = "/ Get the remote address."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-state`: The socket is not connected to a remote address. (ENOTCONN)"]
                    #[doc = "/ "]
                    #[doc = "/ # References"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpeername.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/getpeername.2.html>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getpeername>"]
                    #[doc = "/ - <https://man.freebsd.org/cgi/man.cgi?query=getpeername&sektion=2&n=1>"]
                    fn get_remote_address(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                    ) -> Result<IpSocketAddress, super::super::super::_TrappableError1>
                    {
                        HostTcpSocket::get_remote_address(*self, self_)
                    }
                    #[doc = "/ Whether the socket is in the `listening` state."]
                    #[doc = "/ "]
                    #[doc = "/ Equivalent to the SO_ACCEPTCONN socket option."]
                    fn get_is_listening(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                    ) -> wasmtime::Result<bool> {
                        HostTcpSocket::get_is_listening(*self, self_)
                    }
                    #[doc = "/ Whether this is a IPv4 or IPv6 socket."]
                    #[doc = "/ "]
                    #[doc = "/ This is the value passed to the constructor."]
                    #[doc = "/ "]
                    #[doc = "/ Equivalent to the SO_DOMAIN socket option."]
                    fn get_address_family(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                    ) -> wasmtime::Result<IpAddressFamily> {
                        HostTcpSocket::get_address_family(*self, self_)
                    }
                    #[doc = "/ Hints the desired listen queue size. Implementations are free to"]
                    #[doc = "/ ignore this."]
                    #[doc = "/ "]
                    #[doc = "/ If the provided value is 0, an `invalid-argument` error is returned."]
                    #[doc = "/ Any other value will never cause an error, but it might be silently"]
                    #[doc = "/ clamped and/or rounded."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `not-supported`:        (set) The platform does not support changing the backlog size after the initial listen."]
                    #[doc = "/ - `invalid-argument`:     (set) The provided value was 0."]
                    #[doc = "/ - `invalid-state`:        (set) The socket is in the `connecting` or `connected` state."]
                    fn set_listen_backlog_size(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                        value: u64,
                    ) -> Result<(), super::super::super::_TrappableError1> {
                        HostTcpSocket::set_listen_backlog_size(*self, self_, value)
                    }
                    #[doc = "/ Enables or disables keepalive."]
                    #[doc = "/ "]
                    #[doc = "/ The keepalive behavior can be adjusted using:"]
                    #[doc = "/ - `keep-alive-idle-time`"]
                    #[doc = "/ - `keep-alive-interval`"]
                    #[doc = "/ - `keep-alive-count`"]
                    #[doc = "/ These properties can be configured while `keep-alive-enabled` is"]
                    #[doc = "/ false, but only come into effect when `keep-alive-enabled` is true."]
                    #[doc = "/ "]
                    #[doc = "/ Equivalent to the SO_KEEPALIVE socket option."]
                    fn get_keep_alive_enabled(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                    ) -> Result<bool, super::super::super::_TrappableError1> {
                        HostTcpSocket::get_keep_alive_enabled(*self, self_)
                    }
                    fn set_keep_alive_enabled(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                        value: bool,
                    ) -> Result<(), super::super::super::_TrappableError1> {
                        HostTcpSocket::set_keep_alive_enabled(*self, self_, value)
                    }
                    #[doc = "/ Amount of time the connection has to be idle before TCP starts"]
                    #[doc = "/ sending keepalive packets."]
                    #[doc = "/ "]
                    #[doc = "/ If the provided value is 0, an `invalid-argument` error is returned."]
                    #[doc = "/ All other values are accepted without error, but may be"]
                    #[doc = "/ clamped or rounded. As a result, the value read back from"]
                    #[doc = "/ this setting may differ from the value that was set."]
                    #[doc = "/ "]
                    #[doc = "/ Equivalent to the TCP_KEEPIDLE socket option. (TCP_KEEPALIVE on MacOS)"]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-argument`:     (set) The provided value was 0."]
                    fn get_keep_alive_idle_time(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                    ) -> Result<Duration, super::super::super::_TrappableError1>
                    {
                        HostTcpSocket::get_keep_alive_idle_time(*self, self_)
                    }
                    fn set_keep_alive_idle_time(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                        value: Duration,
                    ) -> Result<(), super::super::super::_TrappableError1> {
                        HostTcpSocket::set_keep_alive_idle_time(*self, self_, value)
                    }
                    #[doc = "/ The time between keepalive packets."]
                    #[doc = "/ "]
                    #[doc = "/ If the provided value is 0, an `invalid-argument` error is returned."]
                    #[doc = "/ All other values are accepted without error, but may be"]
                    #[doc = "/ clamped or rounded. As a result, the value read back from"]
                    #[doc = "/ this setting may differ from the value that was set."]
                    #[doc = "/ "]
                    #[doc = "/ Equivalent to the TCP_KEEPINTVL socket option."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-argument`:     (set) The provided value was 0."]
                    fn get_keep_alive_interval(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                    ) -> Result<Duration, super::super::super::_TrappableError1>
                    {
                        HostTcpSocket::get_keep_alive_interval(*self, self_)
                    }
                    fn set_keep_alive_interval(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                        value: Duration,
                    ) -> Result<(), super::super::super::_TrappableError1> {
                        HostTcpSocket::set_keep_alive_interval(*self, self_, value)
                    }
                    #[doc = "/ The maximum amount of keepalive packets TCP should send before"]
                    #[doc = "/ aborting the connection."]
                    #[doc = "/ "]
                    #[doc = "/ If the provided value is 0, an `invalid-argument` error is returned."]
                    #[doc = "/ All other values are accepted without error, but may be"]
                    #[doc = "/ clamped or rounded. As a result, the value read back from"]
                    #[doc = "/ this setting may differ from the value that was set."]
                    #[doc = "/ "]
                    #[doc = "/ Equivalent to the TCP_KEEPCNT socket option."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-argument`:     (set) The provided value was 0."]
                    fn get_keep_alive_count(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                    ) -> Result<u32, super::super::super::_TrappableError1> {
                        HostTcpSocket::get_keep_alive_count(*self, self_)
                    }
                    fn set_keep_alive_count(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                        value: u32,
                    ) -> Result<(), super::super::super::_TrappableError1> {
                        HostTcpSocket::set_keep_alive_count(*self, self_, value)
                    }
                    #[doc = "/ Equivalent to the IP_TTL & IPV6_UNICAST_HOPS socket options."]
                    #[doc = "/ "]
                    #[doc = "/ If the provided value is 0, an `invalid-argument` error is returned."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-argument`:     (set) The TTL value must be 1 or higher."]
                    fn get_hop_limit(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                    ) -> Result<u8, super::super::super::_TrappableError1> {
                        HostTcpSocket::get_hop_limit(*self, self_)
                    }
                    fn set_hop_limit(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                        value: u8,
                    ) -> Result<(), super::super::super::_TrappableError1> {
                        HostTcpSocket::set_hop_limit(*self, self_, value)
                    }
                    #[doc = "/ Kernel buffer space reserved for sending/receiving on this socket."]
                    #[doc = "/ Implementations usually treat this as a cap the buffer can grow to,"]
                    #[doc = "/ rather than allocating the full amount immediately."]
                    #[doc = "/ "]
                    #[doc = "/ If the provided value is 0, an `invalid-argument` error is returned."]
                    #[doc = "/ All other values are accepted without error, but may be"]
                    #[doc = "/ clamped or rounded. As a result, the value read back from"]
                    #[doc = "/ this setting may differ from the value that was set."]
                    #[doc = "/ "]
                    #[doc = "/ This is only a performance hint. The implementation may ignore it or"]
                    #[doc = "/ tweak it based on real traffic patterns."]
                    #[doc = "/ Linux and macOS appear to behave differently depending on whether a"]
                    #[doc = "/ buffer size was explicitly set. When set, they tend to honor it; when"]
                    #[doc = "/ not set, they dynamically adjust the buffer size as the connection"]
                    #[doc = "/ progresses. This is especially noticeable when comparing the values"]
                    #[doc = "/ from before and after connection establishment."]
                    #[doc = "/ "]
                    #[doc = "/ Equivalent to the SO_RCVBUF and SO_SNDBUF socket options."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-argument`:     (set) The provided value was 0."]
                    fn get_receive_buffer_size(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                    ) -> Result<u64, super::super::super::_TrappableError1> {
                        HostTcpSocket::get_receive_buffer_size(*self, self_)
                    }
                    fn set_receive_buffer_size(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                        value: u64,
                    ) -> Result<(), super::super::super::_TrappableError1> {
                        HostTcpSocket::set_receive_buffer_size(*self, self_, value)
                    }
                    fn get_send_buffer_size(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                    ) -> Result<u64, super::super::super::_TrappableError1> {
                        HostTcpSocket::get_send_buffer_size(*self, self_)
                    }
                    fn set_send_buffer_size(
                        &mut self,
                        self_: wasmtime::component::Resource<TcpSocket>,
                        value: u64,
                    ) -> Result<(), super::super::super::_TrappableError1> {
                        HostTcpSocket::set_send_buffer_size(*self, self_, value)
                    }
                    fn drop(
                        &mut self,
                        rep: wasmtime::component::Resource<TcpSocket>,
                    ) -> wasmtime::Result<()> {
                        HostTcpSocket::drop(*self, rep)
                    }
                }
                #[doc = "/ A UDP socket handle."]
                pub use super::super::super::__with_name4 as UdpSocket;
                pub trait HostUdpSocketWithStore<T>: wasmtime::component::HasData + Send {
                    #[doc = "/ Send a message on the socket to a particular peer."]
                    #[doc = "/ "]
                    #[doc = "/ If the socket is connected, the peer address may be left empty. In"]
                    #[doc = "/ that case this is equivalent to `send` in POSIX. Otherwise it is"]
                    #[doc = "/ equivalent to `sendto`."]
                    #[doc = "/ "]
                    #[doc = "/ Additionally, if the socket is connected, a `remote-address` argument"]
                    #[doc = "/ _may_ be provided but then it must be identical to the address"]
                    #[doc = "/ passed to `connect`."]
                    #[doc = "/ "]
                    #[doc = "/ If the socket has not been explicitly bound, it will be"]
                    #[doc = "/ implicitly bound to a random free port."]
                    #[doc = "/ "]
                    #[doc = "/ Implementations may trap if the `data` length exceeds 64 KiB."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-argument`:        The `remote-address` has the wrong address family. (EAFNOSUPPORT)"]
                    #[doc = "/ - `invalid-argument`:        The IP address in `remote-address` is set to INADDR_ANY (`0.0.0.0` / `::`). (EDESTADDRREQ, EADDRNOTAVAIL)"]
                    #[doc = "/ - `invalid-argument`:        The port in `remote-address` is set to 0. (EDESTADDRREQ, EADDRNOTAVAIL)"]
                    #[doc = "/ - `invalid-argument`:        The socket is in \"connected\" mode and `remote-address` is `some` value that does not match the address passed to `connect`. (EISCONN)"]
                    #[doc = "/ - `invalid-argument`:        The socket is not \"connected\" and no value for `remote-address` was provided. (EDESTADDRREQ)"]
                    #[doc = "/ - `remote-unreachable`:      The remote address is not reachable. (ECONNRESET, ENETRESET on Windows, EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)"]
                    #[doc = "/ - `connection-refused`:      The connection was refused. (ECONNREFUSED)"]
                    #[doc = "/ - `datagram-too-large`:      The datagram is too large. (EMSGSIZE)"]
                    #[doc = "/ - `address-in-use`:          Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE)"]
                    #[doc = "/ "]
                    #[doc = "/ # Implementors note"]
                    #[doc = "/ WASI requires `send` to perform an implicit bind if the socket"]
                    #[doc = "/ has not been bound. Not all platforms (notably Windows) exhibit"]
                    #[doc = "/ this behavior natively. On such platforms, the WASI implementation"]
                    #[doc = "/ should emulate it by performing the bind if the guest has not"]
                    #[doc = "/ already done so."]
                    #[doc = "/ "]
                    #[doc = "/ # References"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendto.html>"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendmsg.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/send.2.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/sendmmsg.2.html>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-send>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-sendto>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasendmsg>"]
                    #[doc = "/ - <https://man.freebsd.org/cgi/man.cgi?query=send&sektion=2>"]
                    fn send(
                        accessor: &wasmtime::component::Accessor<T, Self>,
                        self_: wasmtime::component::Resource<UdpSocket>,
                        data: wasmtime::component::__internal::Vec<u8>,
                        remote_address: Option<IpSocketAddress>,
                    ) -> impl ::core::future::Future<
                        Output = Result<(), super::super::super::_TrappableError1>,
                    > + Send;

                    #[doc = "/ Receive a message on the socket."]
                    #[doc = "/ "]
                    #[doc = "/ On success, the return value contains a tuple of the received data"]
                    #[doc = "/ and the address of the sender. Theoretical maximum length of the"]
                    #[doc = "/ data is 64 KiB. Though in practice, it will typically be less than"]
                    #[doc = "/ 1500 bytes."]
                    #[doc = "/ "]
                    #[doc = "/ If the socket is connected, the sender address is guaranteed to"]
                    #[doc = "/ match the remote address passed to `connect`."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-state`:        The socket has not been bound yet."]
                    #[doc = "/ - `remote-unreachable`:   The remote address is not reachable. (ECONNRESET, ENETRESET on Windows, EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)"]
                    #[doc = "/ - `connection-refused`:   The connection was refused. (ECONNREFUSED)"]
                    #[doc = "/ "]
                    #[doc = "/ # References"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvfrom.html>"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvmsg.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/recv.2.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/recvmmsg.2.html>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-recvfrom>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/mswsock/nc-mswsock-lpfn_wsarecvmsg>"]
                    #[doc = "/ - <https://man.freebsd.org/cgi/man.cgi?query=recv&sektion=2>"]
                    fn receive(
                        accessor: &wasmtime::component::Accessor<T, Self>,
                        self_: wasmtime::component::Resource<UdpSocket>,
                    ) -> impl ::core::future::Future<
                        Output = Result<
                            (wasmtime::component::__internal::Vec<u8>, IpSocketAddress),
                            super::super::super::_TrappableError1,
                        >,
                    > + Send;
                }
                pub trait HostUdpSocket: Send {
                    #[doc = "/ Create a new UDP socket."]
                    #[doc = "/ "]
                    #[doc = "/ Similar to `socket(AF_INET or AF_INET6, SOCK_DGRAM, IPPROTO_UDP)`"]
                    #[doc = "/ in POSIX. On IPv6 sockets, IPV6_V6ONLY is enabled by default and"]
                    #[doc = "/ can\'t be configured otherwise."]
                    #[doc = "/ "]
                    #[doc = "/ Unlike POSIX, WASI sockets have no notion of a socket-level"]
                    #[doc = "/ `O_NONBLOCK` flag. Instead they fully rely on the Component Model\'s"]
                    #[doc = "/ async support."]
                    #[doc = "/ "]
                    #[doc = "/ # References:"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/socket.2.html>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasocketw>"]
                    #[doc = "/ - <https://man.freebsd.org/cgi/man.cgi?query=socket&sektion=2>"]
                    fn create(
                        &mut self,
                        address_family: IpAddressFamily,
                    ) -> impl ::core::future::Future<
                        Output = Result<
                            wasmtime::component::Resource<UdpSocket>,
                            super::super::super::_TrappableError1,
                        >,
                    > + Send;

                    #[doc = "/ Bind the socket to the provided IP address and port."]
                    #[doc = "/ "]
                    #[doc = "/ If the IP address is zero (`0.0.0.0` in IPv4, `::` in IPv6), it is"]
                    #[doc = "/ left to the implementation to decide which network interface(s) to"]
                    #[doc = "/ bind to. If the port is zero, the socket will be bound to a random"]
                    #[doc = "/ free port."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-argument`:          The `local-address` has the wrong address family. (EAFNOSUPPORT, EFAULT on Windows)"]
                    #[doc = "/ - `invalid-state`:             The socket is already bound. (EINVAL)"]
                    #[doc = "/ - `address-in-use`:            No ephemeral ports available. (EADDRINUSE, ENOBUFS on Windows)"]
                    #[doc = "/ - `address-in-use`:            Address is already in use. (EADDRINUSE)"]
                    #[doc = "/ - `address-not-bindable`:      `local-address` is not an address that can be bound to. (EADDRNOTAVAIL)"]
                    #[doc = "/ "]
                    #[doc = "/ # References"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/bind.2.html>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-bind>"]
                    #[doc = "/ - <https://man.freebsd.org/cgi/man.cgi?query=bind&sektion=2&format=html>"]
                    fn bind(
                        &mut self,
                        self_: wasmtime::component::Resource<UdpSocket>,
                        local_address: IpSocketAddress,
                    ) -> impl ::core::future::Future<
                        Output = Result<(), super::super::super::_TrappableError1>,
                    > + Send;

                    #[doc = "/ Associate this socket with a specific peer address."]
                    #[doc = "/ "]
                    #[doc = "/ On success, the `remote-address` of the socket is updated."]
                    #[doc = "/ The `local-address` may be updated as well, based on the best network"]
                    #[doc = "/ path to `remote-address`. If the socket was not already explicitly"]
                    #[doc = "/ bound, this function will implicitly bind the socket to a random"]
                    #[doc = "/ free port."]
                    #[doc = "/ "]
                    #[doc = "/ When a UDP socket is \"connected\", the `send` and `receive` methods"]
                    #[doc = "/ are limited to communicating with that peer only:"]
                    #[doc = "/ - `send` can only be used to send to this destination."]
                    #[doc = "/ - `receive` will only return datagrams sent from the provided `remote-address`."]
                    #[doc = "/ "]
                    #[doc = "/ The name \"connect\" was kept to align with the existing POSIX"]
                    #[doc = "/ terminology. Other than that, this function only changes the local"]
                    #[doc = "/ socket configuration and does not generate any network traffic."]
                    #[doc = "/ The peer is not aware of this \"connection\"."]
                    #[doc = "/ "]
                    #[doc = "/ This method may be called multiple times on the same socket to change"]
                    #[doc = "/ its association, but only the most recent one will be effective."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-argument`:          The `remote-address` has the wrong address family. (EAFNOSUPPORT)"]
                    #[doc = "/ - `invalid-argument`:          The IP address in `remote-address` is set to INADDR_ANY (`0.0.0.0` / `::`). (EDESTADDRREQ, EADDRNOTAVAIL)"]
                    #[doc = "/ - `invalid-argument`:          The port in `remote-address` is set to 0. (EDESTADDRREQ, EADDRNOTAVAIL)"]
                    #[doc = "/ - `address-in-use`:            Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE, EADDRNOTAVAIL on Linux, EAGAIN on BSD)"]
                    #[doc = "/ "]
                    #[doc = "/ # Implementors note"]
                    #[doc = "/ If the socket is already connected, some platforms (e.g. Linux)"]
                    #[doc = "/ require a disconnect before connecting to a different peer address."]
                    #[doc = "/ "]
                    #[doc = "/ # References"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/connect.2.html>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-connect>"]
                    #[doc = "/ - <https://man.freebsd.org/cgi/man.cgi?connect>"]
                    fn connect(
                        &mut self,
                        self_: wasmtime::component::Resource<UdpSocket>,
                        remote_address: IpSocketAddress,
                    ) -> impl ::core::future::Future<
                        Output = Result<(), super::super::super::_TrappableError1>,
                    > + Send;

                    #[doc = "/ Dissociate this socket from its peer address."]
                    #[doc = "/ "]
                    #[doc = "/ After calling this method, `send` & `receive` are free to communicate"]
                    #[doc = "/ with any remote address again."]
                    #[doc = "/ "]
                    #[doc = "/ The POSIX equivalent of this is calling `connect` with an `AF_UNSPEC` address."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-state`:           The socket is not connected."]
                    #[doc = "/ "]
                    #[doc = "/ # References"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/connect.2.html>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-connect>"]
                    #[doc = "/ - <https://man.freebsd.org/cgi/man.cgi?connect>"]
                    fn disconnect(
                        &mut self,
                        self_: wasmtime::component::Resource<UdpSocket>,
                    ) -> Result<(), super::super::super::_TrappableError1>;

                    #[doc = "/ Get the current bound address."]
                    #[doc = "/ "]
                    #[doc = "/ POSIX mentions:"]
                    #[doc = "/ > If the socket has not been bound to a local name, the value"]
                    #[doc = "/ > stored in the object pointed to by `address` is unspecified."]
                    #[doc = "/ "]
                    #[doc = "/ WASI is stricter and requires `get-local-address` to return"]
                    #[doc = "/ `invalid-state` when the socket hasn\'t been bound yet."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-state`: The socket is not bound to any local address."]
                    #[doc = "/ "]
                    #[doc = "/ # References"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockname.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/getsockname.2.html>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getsockname>"]
                    #[doc = "/ - <https://man.freebsd.org/cgi/man.cgi?getsockname>"]
                    fn get_local_address(
                        &mut self,
                        self_: wasmtime::component::Resource<UdpSocket>,
                    ) -> Result<IpSocketAddress, super::super::super::_TrappableError1>;

                    #[doc = "/ Get the address the socket is currently \"connected\" to."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-state`: The socket is not \"connected\" to a specific remote address. (ENOTCONN)"]
                    #[doc = "/ "]
                    #[doc = "/ # References"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpeername.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/getpeername.2.html>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getpeername>"]
                    #[doc = "/ - <https://man.freebsd.org/cgi/man.cgi?query=getpeername&sektion=2&n=1>"]
                    fn get_remote_address(
                        &mut self,
                        self_: wasmtime::component::Resource<UdpSocket>,
                    ) -> Result<IpSocketAddress, super::super::super::_TrappableError1>;

                    #[doc = "/ Whether this is a IPv4 or IPv6 socket."]
                    #[doc = "/ "]
                    #[doc = "/ This is the value passed to the constructor."]
                    #[doc = "/ "]
                    #[doc = "/ Equivalent to the SO_DOMAIN socket option."]
                    fn get_address_family(
                        &mut self,
                        self_: wasmtime::component::Resource<UdpSocket>,
                    ) -> wasmtime::Result<IpAddressFamily>;

                    #[doc = "/ Equivalent to the IP_TTL & IPV6_UNICAST_HOPS socket options."]
                    #[doc = "/ "]
                    #[doc = "/ If the provided value is 0, an `invalid-argument` error is returned."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-argument`:     (set) The TTL value must be 1 or higher."]
                    fn get_unicast_hop_limit(
                        &mut self,
                        self_: wasmtime::component::Resource<UdpSocket>,
                    ) -> Result<u8, super::super::super::_TrappableError1>;

                    fn set_unicast_hop_limit(
                        &mut self,
                        self_: wasmtime::component::Resource<UdpSocket>,
                        value: u8,
                    ) -> Result<(), super::super::super::_TrappableError1>;

                    #[doc = "/ Kernel buffer space reserved for sending/receiving on this socket."]
                    #[doc = "/ Implementations usually treat this as a cap the buffer can grow to,"]
                    #[doc = "/ rather than allocating the full amount immediately."]
                    #[doc = "/ "]
                    #[doc = "/ If the provided value is 0, an `invalid-argument` error is returned."]
                    #[doc = "/ All other values are accepted without error, but may be"]
                    #[doc = "/ clamped or rounded. As a result, the value read back from"]
                    #[doc = "/ this setting may differ from the value that was set."]
                    #[doc = "/ "]
                    #[doc = "/ Equivalent to the SO_RCVBUF and SO_SNDBUF socket options."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-argument`:     (set) The provided value was 0."]
                    fn get_receive_buffer_size(
                        &mut self,
                        self_: wasmtime::component::Resource<UdpSocket>,
                    ) -> Result<u64, super::super::super::_TrappableError1>;

                    fn set_receive_buffer_size(
                        &mut self,
                        self_: wasmtime::component::Resource<UdpSocket>,
                        value: u64,
                    ) -> Result<(), super::super::super::_TrappableError1>;

                    fn get_send_buffer_size(
                        &mut self,
                        self_: wasmtime::component::Resource<UdpSocket>,
                    ) -> Result<u64, super::super::super::_TrappableError1>;

                    fn set_send_buffer_size(
                        &mut self,
                        self_: wasmtime::component::Resource<UdpSocket>,
                        value: u64,
                    ) -> Result<(), super::super::super::_TrappableError1>;

                    fn drop(
                        &mut self,
                        rep: wasmtime::component::Resource<UdpSocket>,
                    ) -> wasmtime::Result<()>;
                }
                impl<_T: HostUdpSocket + ?Sized + Send> HostUdpSocket for &mut _T {
                    #[doc = "/ Create a new UDP socket."]
                    #[doc = "/ "]
                    #[doc = "/ Similar to `socket(AF_INET or AF_INET6, SOCK_DGRAM, IPPROTO_UDP)`"]
                    #[doc = "/ in POSIX. On IPv6 sockets, IPV6_V6ONLY is enabled by default and"]
                    #[doc = "/ can\'t be configured otherwise."]
                    #[doc = "/ "]
                    #[doc = "/ Unlike POSIX, WASI sockets have no notion of a socket-level"]
                    #[doc = "/ `O_NONBLOCK` flag. Instead they fully rely on the Component Model\'s"]
                    #[doc = "/ async support."]
                    #[doc = "/ "]
                    #[doc = "/ # References:"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/socket.2.html>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasocketw>"]
                    #[doc = "/ - <https://man.freebsd.org/cgi/man.cgi?query=socket&sektion=2>"]
                    fn create(
                        &mut self,
                        address_family: IpAddressFamily,
                    ) -> impl ::core::future::Future<
                        Output = Result<
                            wasmtime::component::Resource<UdpSocket>,
                            super::super::super::_TrappableError1,
                        >,
                    > + Send {
                        async move { HostUdpSocket::create(*self, address_family).await }
                    }
                    #[doc = "/ Bind the socket to the provided IP address and port."]
                    #[doc = "/ "]
                    #[doc = "/ If the IP address is zero (`0.0.0.0` in IPv4, `::` in IPv6), it is"]
                    #[doc = "/ left to the implementation to decide which network interface(s) to"]
                    #[doc = "/ bind to. If the port is zero, the socket will be bound to a random"]
                    #[doc = "/ free port."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-argument`:          The `local-address` has the wrong address family. (EAFNOSUPPORT, EFAULT on Windows)"]
                    #[doc = "/ - `invalid-state`:             The socket is already bound. (EINVAL)"]
                    #[doc = "/ - `address-in-use`:            No ephemeral ports available. (EADDRINUSE, ENOBUFS on Windows)"]
                    #[doc = "/ - `address-in-use`:            Address is already in use. (EADDRINUSE)"]
                    #[doc = "/ - `address-not-bindable`:      `local-address` is not an address that can be bound to. (EADDRNOTAVAIL)"]
                    #[doc = "/ "]
                    #[doc = "/ # References"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/bind.2.html>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-bind>"]
                    #[doc = "/ - <https://man.freebsd.org/cgi/man.cgi?query=bind&sektion=2&format=html>"]
                    fn bind(
                        &mut self,
                        self_: wasmtime::component::Resource<UdpSocket>,
                        local_address: IpSocketAddress,
                    ) -> impl ::core::future::Future<
                        Output = Result<(), super::super::super::_TrappableError1>,
                    > + Send {
                        async move { HostUdpSocket::bind(*self, self_, local_address).await }
                    }
                    #[doc = "/ Associate this socket with a specific peer address."]
                    #[doc = "/ "]
                    #[doc = "/ On success, the `remote-address` of the socket is updated."]
                    #[doc = "/ The `local-address` may be updated as well, based on the best network"]
                    #[doc = "/ path to `remote-address`. If the socket was not already explicitly"]
                    #[doc = "/ bound, this function will implicitly bind the socket to a random"]
                    #[doc = "/ free port."]
                    #[doc = "/ "]
                    #[doc = "/ When a UDP socket is \"connected\", the `send` and `receive` methods"]
                    #[doc = "/ are limited to communicating with that peer only:"]
                    #[doc = "/ - `send` can only be used to send to this destination."]
                    #[doc = "/ - `receive` will only return datagrams sent from the provided `remote-address`."]
                    #[doc = "/ "]
                    #[doc = "/ The name \"connect\" was kept to align with the existing POSIX"]
                    #[doc = "/ terminology. Other than that, this function only changes the local"]
                    #[doc = "/ socket configuration and does not generate any network traffic."]
                    #[doc = "/ The peer is not aware of this \"connection\"."]
                    #[doc = "/ "]
                    #[doc = "/ This method may be called multiple times on the same socket to change"]
                    #[doc = "/ its association, but only the most recent one will be effective."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-argument`:          The `remote-address` has the wrong address family. (EAFNOSUPPORT)"]
                    #[doc = "/ - `invalid-argument`:          The IP address in `remote-address` is set to INADDR_ANY (`0.0.0.0` / `::`). (EDESTADDRREQ, EADDRNOTAVAIL)"]
                    #[doc = "/ - `invalid-argument`:          The port in `remote-address` is set to 0. (EDESTADDRREQ, EADDRNOTAVAIL)"]
                    #[doc = "/ - `address-in-use`:            Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE, EADDRNOTAVAIL on Linux, EAGAIN on BSD)"]
                    #[doc = "/ "]
                    #[doc = "/ # Implementors note"]
                    #[doc = "/ If the socket is already connected, some platforms (e.g. Linux)"]
                    #[doc = "/ require a disconnect before connecting to a different peer address."]
                    #[doc = "/ "]
                    #[doc = "/ # References"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/connect.2.html>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-connect>"]
                    #[doc = "/ - <https://man.freebsd.org/cgi/man.cgi?connect>"]
                    fn connect(
                        &mut self,
                        self_: wasmtime::component::Resource<UdpSocket>,
                        remote_address: IpSocketAddress,
                    ) -> impl ::core::future::Future<
                        Output = Result<(), super::super::super::_TrappableError1>,
                    > + Send {
                        async move { HostUdpSocket::connect(*self, self_, remote_address).await }
                    }
                    #[doc = "/ Dissociate this socket from its peer address."]
                    #[doc = "/ "]
                    #[doc = "/ After calling this method, `send` & `receive` are free to communicate"]
                    #[doc = "/ with any remote address again."]
                    #[doc = "/ "]
                    #[doc = "/ The POSIX equivalent of this is calling `connect` with an `AF_UNSPEC` address."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-state`:           The socket is not connected."]
                    #[doc = "/ "]
                    #[doc = "/ # References"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/connect.2.html>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-connect>"]
                    #[doc = "/ - <https://man.freebsd.org/cgi/man.cgi?connect>"]
                    fn disconnect(
                        &mut self,
                        self_: wasmtime::component::Resource<UdpSocket>,
                    ) -> Result<(), super::super::super::_TrappableError1> {
                        HostUdpSocket::disconnect(*self, self_)
                    }
                    #[doc = "/ Get the current bound address."]
                    #[doc = "/ "]
                    #[doc = "/ POSIX mentions:"]
                    #[doc = "/ > If the socket has not been bound to a local name, the value"]
                    #[doc = "/ > stored in the object pointed to by `address` is unspecified."]
                    #[doc = "/ "]
                    #[doc = "/ WASI is stricter and requires `get-local-address` to return"]
                    #[doc = "/ `invalid-state` when the socket hasn\'t been bound yet."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-state`: The socket is not bound to any local address."]
                    #[doc = "/ "]
                    #[doc = "/ # References"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockname.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/getsockname.2.html>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getsockname>"]
                    #[doc = "/ - <https://man.freebsd.org/cgi/man.cgi?getsockname>"]
                    fn get_local_address(
                        &mut self,
                        self_: wasmtime::component::Resource<UdpSocket>,
                    ) -> Result<IpSocketAddress, super::super::super::_TrappableError1>
                    {
                        HostUdpSocket::get_local_address(*self, self_)
                    }
                    #[doc = "/ Get the address the socket is currently \"connected\" to."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-state`: The socket is not \"connected\" to a specific remote address. (ENOTCONN)"]
                    #[doc = "/ "]
                    #[doc = "/ # References"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpeername.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man2/getpeername.2.html>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getpeername>"]
                    #[doc = "/ - <https://man.freebsd.org/cgi/man.cgi?query=getpeername&sektion=2&n=1>"]
                    fn get_remote_address(
                        &mut self,
                        self_: wasmtime::component::Resource<UdpSocket>,
                    ) -> Result<IpSocketAddress, super::super::super::_TrappableError1>
                    {
                        HostUdpSocket::get_remote_address(*self, self_)
                    }
                    #[doc = "/ Whether this is a IPv4 or IPv6 socket."]
                    #[doc = "/ "]
                    #[doc = "/ This is the value passed to the constructor."]
                    #[doc = "/ "]
                    #[doc = "/ Equivalent to the SO_DOMAIN socket option."]
                    fn get_address_family(
                        &mut self,
                        self_: wasmtime::component::Resource<UdpSocket>,
                    ) -> wasmtime::Result<IpAddressFamily> {
                        HostUdpSocket::get_address_family(*self, self_)
                    }
                    #[doc = "/ Equivalent to the IP_TTL & IPV6_UNICAST_HOPS socket options."]
                    #[doc = "/ "]
                    #[doc = "/ If the provided value is 0, an `invalid-argument` error is returned."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-argument`:     (set) The TTL value must be 1 or higher."]
                    fn get_unicast_hop_limit(
                        &mut self,
                        self_: wasmtime::component::Resource<UdpSocket>,
                    ) -> Result<u8, super::super::super::_TrappableError1> {
                        HostUdpSocket::get_unicast_hop_limit(*self, self_)
                    }
                    fn set_unicast_hop_limit(
                        &mut self,
                        self_: wasmtime::component::Resource<UdpSocket>,
                        value: u8,
                    ) -> Result<(), super::super::super::_TrappableError1> {
                        HostUdpSocket::set_unicast_hop_limit(*self, self_, value)
                    }
                    #[doc = "/ Kernel buffer space reserved for sending/receiving on this socket."]
                    #[doc = "/ Implementations usually treat this as a cap the buffer can grow to,"]
                    #[doc = "/ rather than allocating the full amount immediately."]
                    #[doc = "/ "]
                    #[doc = "/ If the provided value is 0, an `invalid-argument` error is returned."]
                    #[doc = "/ All other values are accepted without error, but may be"]
                    #[doc = "/ clamped or rounded. As a result, the value read back from"]
                    #[doc = "/ this setting may differ from the value that was set."]
                    #[doc = "/ "]
                    #[doc = "/ Equivalent to the SO_RCVBUF and SO_SNDBUF socket options."]
                    #[doc = "/ "]
                    #[doc = "/ # Typical errors"]
                    #[doc = "/ - `invalid-argument`:     (set) The provided value was 0."]
                    fn get_receive_buffer_size(
                        &mut self,
                        self_: wasmtime::component::Resource<UdpSocket>,
                    ) -> Result<u64, super::super::super::_TrappableError1> {
                        HostUdpSocket::get_receive_buffer_size(*self, self_)
                    }
                    fn set_receive_buffer_size(
                        &mut self,
                        self_: wasmtime::component::Resource<UdpSocket>,
                        value: u64,
                    ) -> Result<(), super::super::super::_TrappableError1> {
                        HostUdpSocket::set_receive_buffer_size(*self, self_, value)
                    }
                    fn get_send_buffer_size(
                        &mut self,
                        self_: wasmtime::component::Resource<UdpSocket>,
                    ) -> Result<u64, super::super::super::_TrappableError1> {
                        HostUdpSocket::get_send_buffer_size(*self, self_)
                    }
                    fn set_send_buffer_size(
                        &mut self,
                        self_: wasmtime::component::Resource<UdpSocket>,
                        value: u64,
                    ) -> Result<(), super::super::super::_TrappableError1> {
                        HostUdpSocket::set_send_buffer_size(*self, self_, value)
                    }
                    fn drop(
                        &mut self,
                        rep: wasmtime::component::Resource<UdpSocket>,
                    ) -> wasmtime::Result<()> {
                        HostUdpSocket::drop(*self, rep)
                    }
                }
                pub trait HostWithStore<T>:
                    wasmtime::component::HasData
                    + HostTcpSocketWithStore<T>
                    + HostUdpSocketWithStore<T>
                    + Send
                {
                }
                impl<H: ?Sized, T> HostWithStore<T> for H where
                    H: wasmtime::component::HasData
                        + HostTcpSocketWithStore<T>
                        + HostUdpSocketWithStore<T>
                        + Send
                {
                }
                pub trait Host: HostTcpSocket + HostUdpSocket + Send {
                    fn convert_error_code(
                        &mut self,
                        err: super::super::super::_TrappableError1,
                    ) -> wasmtime::Result<ErrorCode>;
                }
                impl<_T: Host + ?Sized + Send> Host for &mut _T {
                    fn convert_error_code(
                        &mut self,
                        err: super::super::super::_TrappableError1,
                    ) -> wasmtime::Result<ErrorCode> {
                        Host::convert_error_code(*self, err)
                    }
                }
                pub fn add_to_linker_instance<T, D>(
                    inst: &mut wasmtime::component::LinkerInstance<'_, T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static + Send,
                {
                    inst.resource(
                        "tcp-socket",
                        wasmtime::component::ResourceType::host::<TcpSocket>(),
                        move |mut store, rep| -> wasmtime::Result<()> {
                            let resource = wasmtime::component::Resource::new_own(rep);
                            wasmtime::ToWasmtimeResult::to_wasmtime_result(HostTcpSocket::drop(
                                &mut host_getter(store.data_mut()),
                                resource,
                            ))
                        },
                    )?;
                    inst.resource(
                        "udp-socket",
                        wasmtime::component::ResourceType::host::<UdpSocket>(),
                        move |mut store, rep| -> wasmtime::Result<()> {
                            let resource = wasmtime::component::Resource::new_own(rep);
                            wasmtime::ToWasmtimeResult::to_wasmtime_result(HostUdpSocket::drop(
                                &mut host_getter(store.data_mut()),
                                resource,
                            ))
                        },
                    )?;
                    inst.func_wrap(
                        "[static]tcp-socket.create",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>,
                              (arg0,): (IpAddressFamily,)| {
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "types",
                                function = "[static]tcp-socket.create",
                            );
                            let _enter = span.enter();
                            tracing::event!(
                                tracing::Level::TRACE,
                                address_family = tracing::field::debug(&arg0),
                                "call"
                            );
                            let host = &mut host_getter(caller.data_mut());
                            let r = HostTcpSocket::create(host, arg0);
                            tracing::event!(
                                tracing::Level::TRACE,
                                result = tracing::field::debug(&r),
                                "return"
                            );
                            Ok((match r {
                                Ok(a) => Ok(a),
                                Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(
                                    Host::convert_error_code(host, e),
                                )?),
                            },))
                        },
                    )?;
                    inst.func_wrap_async(
                        "[method]tcp-socket.bind",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>,
                              (arg0, arg1): (
                            wasmtime::component::Resource<TcpSocket>,
                            IpSocketAddress,
                        )| {
                            use tracing::Instrument;
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "types",
                                function = "[method]tcp-socket.bind",
                            );
                            wasmtime::component::__internal::Box::new(
                                async move {
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        self_ = tracing::field::debug(&arg0),
                                        local_address = tracing::field::debug(&arg1),
                                        "call"
                                    );
                                    let host = &mut host_getter(caller.data_mut());
                                    let r = HostTcpSocket::bind(host, arg0, arg1).await;
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        result = tracing::field::debug(&r),
                                        "return"
                                    );
                                    Ok((match r {
                                        Ok(a) => Ok(a),
                                        Err(e) => {
                                            Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(
                                                Host::convert_error_code(host, e),
                                            )?)
                                        }
                                    },))
                                }
                                .instrument(span),
                            )
                        },
                    )?;
                    inst.func_wrap_concurrent(
                        "[method]tcp-socket.connect",
                        move |caller: &wasmtime::component::Accessor<T>,
                              (arg0, arg1): (
                            wasmtime::component::Resource<TcpSocket>,
                            IpSocketAddress,
                        )| {
                            use tracing::Instrument;
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "types",
                                function = "[method]tcp-socket.connect",
                            );
                            wasmtime::component::__internal::Box::pin(
                                async move {
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        self_ = tracing::field::debug(&arg0),
                                        remote_address = tracing::field::debug(&arg1),
                                        "call"
                                    );
                                    let host = &caller.with_getter::<D>(host_getter);
                                    let r =
                                        <D as HostTcpSocketWithStore<T>>::connect(host, arg0, arg1)
                                            .await;
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        result = tracing::field::debug(&r),
                                        "return"
                                    );
                                    Ok((match r {
                                        Ok(a) => Ok(a),
                                        Err(e) => {
                                            Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(
                                                caller.with(|mut host| {
                                                    Host::convert_error_code(
                                                        &mut host_getter(host.get()),
                                                        e,
                                                    )
                                                }),
                                            )?)
                                        }
                                    },))
                                }
                                .instrument(span),
                            )
                        },
                    )?;
                    inst.func_wrap_async("[method]tcp-socket.listen", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (wasmtime::component::Resource<TcpSocket>,)|{
                        use tracing::Instrument;
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]tcp-socket.listen",);
                        wasmtime::component::__internal::Box::new(async move {
                            tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
                            let access_cx = wasmtime::AsContextMut::as_context_mut(&mut caller);
                            let host = wasmtime::component::Access::<T, D>::new(access_cx, host_getter);
                            let r =  <D as HostTcpSocketWithStore<T>>::listen(host, arg0,).await;
                            tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                            Ok((match r {
                                Ok(a) => Ok(a),
                                Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(Host::convert_error_code(&mut host_getter(caller.data_mut()), e))?),
                            },))
                        }.instrument(span))
                    })?;
                    inst.func_wrap(
                        "[method]tcp-socket.send",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>,
                              (arg0, arg1): (
                            wasmtime::component::Resource<TcpSocket>,
                            wasmtime::component::StreamReader<u8>,
                        )| {
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "types",
                                function = "[method]tcp-socket.send",
                            );
                            let _enter = span.enter();
                            tracing::event!(
                                tracing::Level::TRACE,
                                self_ = tracing::field::debug(&arg0),
                                data = tracing::field::debug(&arg1),
                                "call"
                            );
                            let access_cx = wasmtime::AsContextMut::as_context_mut(&mut caller);
                            let host =
                                wasmtime::component::Access::<T, D>::new(access_cx, host_getter);
                            let r = <D as HostTcpSocketWithStore<T>>::send(host, arg0, arg1);
                            tracing::event!(
                                tracing::Level::TRACE,
                                result = tracing::field::debug(&r),
                                "return"
                            );
                            Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                        },
                    )?;
                    inst.func_wrap("[method]tcp-socket.receive", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (wasmtime::component::Resource<TcpSocket>,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]tcp-socket.receive",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
                        let access_cx = wasmtime::AsContextMut::as_context_mut(&mut caller);
                        let host = wasmtime::component::Access::<T, D>::new(access_cx, host_getter);
                        let r =  <D as HostTcpSocketWithStore<T>>::receive(host, arg0,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                    })?;
                    inst.func_wrap("[method]tcp-socket.get-local-address", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (wasmtime::component::Resource<TcpSocket>,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]tcp-socket.get-local-address",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
                        let host =  &mut host_getter(caller.data_mut());
                        let r = HostTcpSocket::get_local_address(host, arg0,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((match r {
                            Ok(a) => Ok(a),
                            Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(Host::convert_error_code(host, e))?),
                        },))
                    })?;
                    inst.func_wrap("[method]tcp-socket.get-remote-address", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (wasmtime::component::Resource<TcpSocket>,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]tcp-socket.get-remote-address",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
                        let host =  &mut host_getter(caller.data_mut());
                        let r = HostTcpSocket::get_remote_address(host, arg0,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((match r {
                            Ok(a) => Ok(a),
                            Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(Host::convert_error_code(host, e))?),
                        },))
                    })?;
                    inst.func_wrap("[method]tcp-socket.get-is-listening", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (wasmtime::component::Resource<TcpSocket>,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]tcp-socket.get-is-listening",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
                        let host =  &mut host_getter(caller.data_mut());
                        let r = HostTcpSocket::get_is_listening(host, arg0,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                    })?;
                    inst.func_wrap("[method]tcp-socket.get-address-family", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (wasmtime::component::Resource<TcpSocket>,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]tcp-socket.get-address-family",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
                        let host =  &mut host_getter(caller.data_mut());
                        let r = HostTcpSocket::get_address_family(host, arg0,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                    })?;
                    inst.func_wrap("[method]tcp-socket.set-listen-backlog-size", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0, arg1,): (wasmtime::component::Resource<TcpSocket>, u64,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]tcp-socket.set-listen-backlog-size",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), value = tracing::field::debug(&arg1), "call");
                        let host =  &mut host_getter(caller.data_mut());
                        let r = HostTcpSocket::set_listen_backlog_size(host, arg0, arg1,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((match r {
                            Ok(a) => Ok(a),
                            Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(Host::convert_error_code(host, e))?),
                        },))
                    })?;
                    inst.func_wrap("[method]tcp-socket.get-keep-alive-enabled", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (wasmtime::component::Resource<TcpSocket>,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]tcp-socket.get-keep-alive-enabled",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
                        let host =  &mut host_getter(caller.data_mut());
                        let r = HostTcpSocket::get_keep_alive_enabled(host, arg0,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((match r {
                            Ok(a) => Ok(a),
                            Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(Host::convert_error_code(host, e))?),
                        },))
                    })?;
                    inst.func_wrap("[method]tcp-socket.set-keep-alive-enabled", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0, arg1,): (wasmtime::component::Resource<TcpSocket>, bool,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]tcp-socket.set-keep-alive-enabled",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), value = tracing::field::debug(&arg1), "call");
                        let host =  &mut host_getter(caller.data_mut());
                        let r = HostTcpSocket::set_keep_alive_enabled(host, arg0, arg1,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((match r {
                            Ok(a) => Ok(a),
                            Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(Host::convert_error_code(host, e))?),
                        },))
                    })?;
                    inst.func_wrap("[method]tcp-socket.get-keep-alive-idle-time", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (wasmtime::component::Resource<TcpSocket>,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]tcp-socket.get-keep-alive-idle-time",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
                        let host =  &mut host_getter(caller.data_mut());
                        let r = HostTcpSocket::get_keep_alive_idle_time(host, arg0,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((match r {
                            Ok(a) => Ok(a),
                            Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(Host::convert_error_code(host, e))?),
                        },))
                    })?;
                    inst.func_wrap(
                        "[method]tcp-socket.set-keep-alive-idle-time",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>,
                              (arg0, arg1): (
                            wasmtime::component::Resource<TcpSocket>,
                            Duration,
                        )| {
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "types",
                                function = "[method]tcp-socket.set-keep-alive-idle-time",
                            );
                            let _enter = span.enter();
                            tracing::event!(
                                tracing::Level::TRACE,
                                self_ = tracing::field::debug(&arg0),
                                value = tracing::field::debug(&arg1),
                                "call"
                            );
                            let host = &mut host_getter(caller.data_mut());
                            let r = HostTcpSocket::set_keep_alive_idle_time(host, arg0, arg1);
                            tracing::event!(
                                tracing::Level::TRACE,
                                result = tracing::field::debug(&r),
                                "return"
                            );
                            Ok((match r {
                                Ok(a) => Ok(a),
                                Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(
                                    Host::convert_error_code(host, e),
                                )?),
                            },))
                        },
                    )?;
                    inst.func_wrap("[method]tcp-socket.get-keep-alive-interval", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (wasmtime::component::Resource<TcpSocket>,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]tcp-socket.get-keep-alive-interval",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
                        let host =  &mut host_getter(caller.data_mut());
                        let r = HostTcpSocket::get_keep_alive_interval(host, arg0,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((match r {
                            Ok(a) => Ok(a),
                            Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(Host::convert_error_code(host, e))?),
                        },))
                    })?;
                    inst.func_wrap(
                        "[method]tcp-socket.set-keep-alive-interval",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>,
                              (arg0, arg1): (
                            wasmtime::component::Resource<TcpSocket>,
                            Duration,
                        )| {
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "types",
                                function = "[method]tcp-socket.set-keep-alive-interval",
                            );
                            let _enter = span.enter();
                            tracing::event!(
                                tracing::Level::TRACE,
                                self_ = tracing::field::debug(&arg0),
                                value = tracing::field::debug(&arg1),
                                "call"
                            );
                            let host = &mut host_getter(caller.data_mut());
                            let r = HostTcpSocket::set_keep_alive_interval(host, arg0, arg1);
                            tracing::event!(
                                tracing::Level::TRACE,
                                result = tracing::field::debug(&r),
                                "return"
                            );
                            Ok((match r {
                                Ok(a) => Ok(a),
                                Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(
                                    Host::convert_error_code(host, e),
                                )?),
                            },))
                        },
                    )?;
                    inst.func_wrap("[method]tcp-socket.get-keep-alive-count", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (wasmtime::component::Resource<TcpSocket>,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]tcp-socket.get-keep-alive-count",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
                        let host =  &mut host_getter(caller.data_mut());
                        let r = HostTcpSocket::get_keep_alive_count(host, arg0,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((match r {
                            Ok(a) => Ok(a),
                            Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(Host::convert_error_code(host, e))?),
                        },))
                    })?;
                    inst.func_wrap("[method]tcp-socket.set-keep-alive-count", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0, arg1,): (wasmtime::component::Resource<TcpSocket>, u32,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]tcp-socket.set-keep-alive-count",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), value = tracing::field::debug(&arg1), "call");
                        let host =  &mut host_getter(caller.data_mut());
                        let r = HostTcpSocket::set_keep_alive_count(host, arg0, arg1,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((match r {
                            Ok(a) => Ok(a),
                            Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(Host::convert_error_code(host, e))?),
                        },))
                    })?;
                    inst.func_wrap("[method]tcp-socket.get-hop-limit", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (wasmtime::component::Resource<TcpSocket>,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]tcp-socket.get-hop-limit",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
                        let host =  &mut host_getter(caller.data_mut());
                        let r = HostTcpSocket::get_hop_limit(host, arg0,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((match r {
                            Ok(a) => Ok(a),
                            Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(Host::convert_error_code(host, e))?),
                        },))
                    })?;
                    inst.func_wrap("[method]tcp-socket.set-hop-limit", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0, arg1,): (wasmtime::component::Resource<TcpSocket>, u8,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]tcp-socket.set-hop-limit",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), value = tracing::field::debug(&arg1), "call");
                        let host =  &mut host_getter(caller.data_mut());
                        let r = HostTcpSocket::set_hop_limit(host, arg0, arg1,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((match r {
                            Ok(a) => Ok(a),
                            Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(Host::convert_error_code(host, e))?),
                        },))
                    })?;
                    inst.func_wrap("[method]tcp-socket.get-receive-buffer-size", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (wasmtime::component::Resource<TcpSocket>,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]tcp-socket.get-receive-buffer-size",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
                        let host =  &mut host_getter(caller.data_mut());
                        let r = HostTcpSocket::get_receive_buffer_size(host, arg0,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((match r {
                            Ok(a) => Ok(a),
                            Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(Host::convert_error_code(host, e))?),
                        },))
                    })?;
                    inst.func_wrap("[method]tcp-socket.set-receive-buffer-size", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0, arg1,): (wasmtime::component::Resource<TcpSocket>, u64,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]tcp-socket.set-receive-buffer-size",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), value = tracing::field::debug(&arg1), "call");
                        let host =  &mut host_getter(caller.data_mut());
                        let r = HostTcpSocket::set_receive_buffer_size(host, arg0, arg1,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((match r {
                            Ok(a) => Ok(a),
                            Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(Host::convert_error_code(host, e))?),
                        },))
                    })?;
                    inst.func_wrap("[method]tcp-socket.get-send-buffer-size", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (wasmtime::component::Resource<TcpSocket>,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]tcp-socket.get-send-buffer-size",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
                        let host =  &mut host_getter(caller.data_mut());
                        let r = HostTcpSocket::get_send_buffer_size(host, arg0,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((match r {
                            Ok(a) => Ok(a),
                            Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(Host::convert_error_code(host, e))?),
                        },))
                    })?;
                    inst.func_wrap("[method]tcp-socket.set-send-buffer-size", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0, arg1,): (wasmtime::component::Resource<TcpSocket>, u64,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]tcp-socket.set-send-buffer-size",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), value = tracing::field::debug(&arg1), "call");
                        let host =  &mut host_getter(caller.data_mut());
                        let r = HostTcpSocket::set_send_buffer_size(host, arg0, arg1,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((match r {
                            Ok(a) => Ok(a),
                            Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(Host::convert_error_code(host, e))?),
                        },))
                    })?;
                    inst.func_wrap_async(
                        "[static]udp-socket.create",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>,
                              (arg0,): (IpAddressFamily,)| {
                            use tracing::Instrument;
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "types",
                                function = "[static]udp-socket.create",
                            );
                            wasmtime::component::__internal::Box::new(
                                async move {
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        address_family = tracing::field::debug(&arg0),
                                        "call"
                                    );
                                    let host = &mut host_getter(caller.data_mut());
                                    let r = HostUdpSocket::create(host, arg0).await;
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        result = tracing::field::debug(&r),
                                        "return"
                                    );
                                    Ok((match r {
                                        Ok(a) => Ok(a),
                                        Err(e) => {
                                            Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(
                                                Host::convert_error_code(host, e),
                                            )?)
                                        }
                                    },))
                                }
                                .instrument(span),
                            )
                        },
                    )?;
                    inst.func_wrap_async(
                        "[method]udp-socket.bind",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>,
                              (arg0, arg1): (
                            wasmtime::component::Resource<UdpSocket>,
                            IpSocketAddress,
                        )| {
                            use tracing::Instrument;
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "types",
                                function = "[method]udp-socket.bind",
                            );
                            wasmtime::component::__internal::Box::new(
                                async move {
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        self_ = tracing::field::debug(&arg0),
                                        local_address = tracing::field::debug(&arg1),
                                        "call"
                                    );
                                    let host = &mut host_getter(caller.data_mut());
                                    let r = HostUdpSocket::bind(host, arg0, arg1).await;
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        result = tracing::field::debug(&r),
                                        "return"
                                    );
                                    Ok((match r {
                                        Ok(a) => Ok(a),
                                        Err(e) => {
                                            Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(
                                                Host::convert_error_code(host, e),
                                            )?)
                                        }
                                    },))
                                }
                                .instrument(span),
                            )
                        },
                    )?;
                    inst.func_wrap_async(
                        "[method]udp-socket.connect",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>,
                              (arg0, arg1): (
                            wasmtime::component::Resource<UdpSocket>,
                            IpSocketAddress,
                        )| {
                            use tracing::Instrument;
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "types",
                                function = "[method]udp-socket.connect",
                            );
                            wasmtime::component::__internal::Box::new(
                                async move {
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        self_ = tracing::field::debug(&arg0),
                                        remote_address = tracing::field::debug(&arg1),
                                        "call"
                                    );
                                    let host = &mut host_getter(caller.data_mut());
                                    let r = HostUdpSocket::connect(host, arg0, arg1).await;
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        result = tracing::field::debug(&r),
                                        "return"
                                    );
                                    Ok((match r {
                                        Ok(a) => Ok(a),
                                        Err(e) => {
                                            Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(
                                                Host::convert_error_code(host, e),
                                            )?)
                                        }
                                    },))
                                }
                                .instrument(span),
                            )
                        },
                    )?;
                    inst.func_wrap("[method]udp-socket.disconnect", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (wasmtime::component::Resource<UdpSocket>,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]udp-socket.disconnect",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
                        let host =  &mut host_getter(caller.data_mut());
                        let r = HostUdpSocket::disconnect(host, arg0,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((match r {
                            Ok(a) => Ok(a),
                            Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(Host::convert_error_code(host, e))?),
                        },))
                    })?;
                    inst.func_wrap_concurrent(
                        "[method]udp-socket.send",
                        move |caller: &wasmtime::component::Accessor<T>,
                              (arg0, arg1, arg2): (
                            wasmtime::component::Resource<UdpSocket>,
                            wasmtime::component::__internal::Vec<u8>,
                            Option<IpSocketAddress>,
                        )| {
                            use tracing::Instrument;
                            let span = tracing::span!(
                                tracing::Level::TRACE,
                                "wit-bindgen import",
                                module = "types",
                                function = "[method]udp-socket.send",
                            );
                            wasmtime::component::__internal::Box::pin(
                                async move {
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        self_ = tracing::field::debug(&arg0),
                                        data = tracing::field::debug("..."),
                                        remote_address = tracing::field::debug(&arg2),
                                        "call"
                                    );
                                    let host = &caller.with_getter::<D>(host_getter);
                                    let r = <D as HostUdpSocketWithStore<T>>::send(
                                        host, arg0, arg1, arg2,
                                    )
                                    .await;
                                    tracing::event!(
                                        tracing::Level::TRACE,
                                        result = tracing::field::debug(&r),
                                        "return"
                                    );
                                    Ok((match r {
                                        Ok(a) => Ok(a),
                                        Err(e) => {
                                            Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(
                                                caller.with(|mut host| {
                                                    Host::convert_error_code(
                                                        &mut host_getter(host.get()),
                                                        e,
                                                    )
                                                }),
                                            )?)
                                        }
                                    },))
                                }
                                .instrument(span),
                            )
                        },
                    )?;
                    inst.func_wrap_concurrent("[method]udp-socket.receive", move|caller: &wasmtime::component::Accessor::<T>, (arg0,): (wasmtime::component::Resource<UdpSocket>,)|{
                        use tracing::Instrument;
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]udp-socket.receive",);
                        wasmtime::component::__internal::Box::pin(async move {
                            tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
                            let host =  &caller.with_getter::<D>(host_getter);
                            let r =  <D as HostUdpSocketWithStore<T>>::receive(host, arg0,).await;
                            tracing::event!(tracing::Level::TRACE, result = tracing::field::debug("..."), "return");
                            Ok((match r {
                                Ok(a) => Ok(a),
                                Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(caller.with(|mut host|Host::convert_error_code(&mut host_getter(host.get()), e)))?),
                            },))
                        }.instrument(span))
                    })?;
                    inst.func_wrap("[method]udp-socket.get-local-address", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (wasmtime::component::Resource<UdpSocket>,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]udp-socket.get-local-address",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
                        let host =  &mut host_getter(caller.data_mut());
                        let r = HostUdpSocket::get_local_address(host, arg0,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((match r {
                            Ok(a) => Ok(a),
                            Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(Host::convert_error_code(host, e))?),
                        },))
                    })?;
                    inst.func_wrap("[method]udp-socket.get-remote-address", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (wasmtime::component::Resource<UdpSocket>,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]udp-socket.get-remote-address",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
                        let host =  &mut host_getter(caller.data_mut());
                        let r = HostUdpSocket::get_remote_address(host, arg0,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((match r {
                            Ok(a) => Ok(a),
                            Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(Host::convert_error_code(host, e))?),
                        },))
                    })?;
                    inst.func_wrap("[method]udp-socket.get-address-family", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (wasmtime::component::Resource<UdpSocket>,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]udp-socket.get-address-family",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
                        let host =  &mut host_getter(caller.data_mut());
                        let r = HostUdpSocket::get_address_family(host, arg0,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                    })?;
                    inst.func_wrap("[method]udp-socket.get-unicast-hop-limit", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (wasmtime::component::Resource<UdpSocket>,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]udp-socket.get-unicast-hop-limit",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
                        let host =  &mut host_getter(caller.data_mut());
                        let r = HostUdpSocket::get_unicast_hop_limit(host, arg0,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((match r {
                            Ok(a) => Ok(a),
                            Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(Host::convert_error_code(host, e))?),
                        },))
                    })?;
                    inst.func_wrap("[method]udp-socket.set-unicast-hop-limit", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0, arg1,): (wasmtime::component::Resource<UdpSocket>, u8,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]udp-socket.set-unicast-hop-limit",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), value = tracing::field::debug(&arg1), "call");
                        let host =  &mut host_getter(caller.data_mut());
                        let r = HostUdpSocket::set_unicast_hop_limit(host, arg0, arg1,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((match r {
                            Ok(a) => Ok(a),
                            Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(Host::convert_error_code(host, e))?),
                        },))
                    })?;
                    inst.func_wrap("[method]udp-socket.get-receive-buffer-size", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (wasmtime::component::Resource<UdpSocket>,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]udp-socket.get-receive-buffer-size",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
                        let host =  &mut host_getter(caller.data_mut());
                        let r = HostUdpSocket::get_receive_buffer_size(host, arg0,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((match r {
                            Ok(a) => Ok(a),
                            Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(Host::convert_error_code(host, e))?),
                        },))
                    })?;
                    inst.func_wrap("[method]udp-socket.set-receive-buffer-size", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0, arg1,): (wasmtime::component::Resource<UdpSocket>, u64,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]udp-socket.set-receive-buffer-size",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), value = tracing::field::debug(&arg1), "call");
                        let host =  &mut host_getter(caller.data_mut());
                        let r = HostUdpSocket::set_receive_buffer_size(host, arg0, arg1,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((match r {
                            Ok(a) => Ok(a),
                            Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(Host::convert_error_code(host, e))?),
                        },))
                    })?;
                    inst.func_wrap("[method]udp-socket.get-send-buffer-size", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (wasmtime::component::Resource<UdpSocket>,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]udp-socket.get-send-buffer-size",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
                        let host =  &mut host_getter(caller.data_mut());
                        let r = HostUdpSocket::get_send_buffer_size(host, arg0,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((match r {
                            Ok(a) => Ok(a),
                            Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(Host::convert_error_code(host, e))?),
                        },))
                    })?;
                    inst.func_wrap("[method]udp-socket.set-send-buffer-size", move|mut caller: wasmtime::StoreContextMut<'_, T>, (arg0, arg1,): (wasmtime::component::Resource<UdpSocket>, u64,)|{
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "types", function = "[method]udp-socket.set-send-buffer-size",);
                        let _enter = span.enter();
                        tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), value = tracing::field::debug(&arg1), "call");
                        let host =  &mut host_getter(caller.data_mut());
                        let r = HostUdpSocket::set_send_buffer_size(host, arg0, arg1,);
                        tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");
                        Ok((match r {
                            Ok(a) => Ok(a),
                            Err(e) => Err(wasmtime::ToWasmtimeResult::to_wasmtime_result(Host::convert_error_code(host, e))?),
                        },))
                    })?;
                    Ok(())
                }
                pub fn add_to_linker<T, D>(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static + Send,
                {
                    let mut inst = linker.instance("wasi:sockets/types@0.3.0")?;
                    add_to_linker_instance::<T, D>(&mut inst, host_getter)
                }
            }
            #[allow(clippy::all)]
            pub mod ip_name_lookup {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::Box;
                pub type IpAddress = super::super::super::wasi::sockets::types::IpAddress;
                const _: () = {
                    assert!(18 == <IpAddress as wasmtime::component::ComponentType>::SIZE32);
                    assert!(2 == <IpAddress as wasmtime::component::ComponentType>::ALIGN32);
                };
                #[doc = "/ Lookup error codes."]
                #[derive(
                    wasmtime::component::ComponentType,
                    wasmtime::component::Lift,
                    wasmtime::component::Lower,
                )]
                #[component(variant)]
                #[derive(Clone)]
                pub enum ErrorCode {
                    #[doc = "/ Access denied."]
                    #[doc = "/ "]
                    #[doc = "/ POSIX equivalent: EACCES, EPERM"]
                    #[component(name = "access-denied")]
                    AccessDenied,
                    #[doc = "/ `name` is a syntactically invalid domain name or IP address."]
                    #[doc = "/ "]
                    #[doc = "/ POSIX equivalent: EINVAL"]
                    #[component(name = "invalid-argument")]
                    InvalidArgument,
                    #[doc = "/ Name does not exist or has no suitable associated IP addresses."]
                    #[doc = "/ "]
                    #[doc = "/ POSIX equivalent: EAI_NONAME, EAI_NODATA, EAI_ADDRFAMILY"]
                    #[component(name = "name-unresolvable")]
                    NameUnresolvable,
                    #[doc = "/ A temporary failure in name resolution occurred."]
                    #[doc = "/ "]
                    #[doc = "/ POSIX equivalent: EAI_AGAIN"]
                    #[component(name = "temporary-resolver-failure")]
                    TemporaryResolverFailure,
                    #[doc = "/ A permanent failure in name resolution occurred."]
                    #[doc = "/ "]
                    #[doc = "/ POSIX equivalent: EAI_FAIL"]
                    #[component(name = "permanent-resolver-failure")]
                    PermanentResolverFailure,
                    #[doc = "/ A catch-all for errors not captured by the existing variants."]
                    #[doc = "/ Implementations can use this to extend the error type without"]
                    #[doc = "/ breaking existing code."]
                    #[component(name = "other")]
                    Other(Option<wasmtime::component::__internal::String>),
                }
                impl core::fmt::Debug for ErrorCode {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        match self {
                            ErrorCode::AccessDenied => {
                                f.debug_tuple("ErrorCode::AccessDenied").finish()
                            }
                            ErrorCode::InvalidArgument => {
                                f.debug_tuple("ErrorCode::InvalidArgument").finish()
                            }
                            ErrorCode::NameUnresolvable => {
                                f.debug_tuple("ErrorCode::NameUnresolvable").finish()
                            }
                            ErrorCode::TemporaryResolverFailure => f
                                .debug_tuple("ErrorCode::TemporaryResolverFailure")
                                .finish(),
                            ErrorCode::PermanentResolverFailure => f
                                .debug_tuple("ErrorCode::PermanentResolverFailure")
                                .finish(),
                            ErrorCode::Other(e) => {
                                f.debug_tuple("ErrorCode::Other").field(e).finish()
                            }
                        }
                    }
                }
                impl core::fmt::Display for ErrorCode {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(f, "{:?}", self)
                    }
                }
                impl core::error::Error for ErrorCode {}
                const _: () = {
                    assert!(16 == <ErrorCode as wasmtime::component::ComponentType>::SIZE32);
                    assert!(4 == <ErrorCode as wasmtime::component::ComponentType>::ALIGN32);
                };
                pub trait HostWithStore<T>: wasmtime::component::HasData + Send {
                    #[doc = "/ Resolve an internet host name to a list of IP addresses."]
                    #[doc = "/ "]
                    #[doc = "/ Unicode domain names are automatically converted to ASCII using IDNA"]
                    #[doc = "/ encoding. If the input is an IP address string, the address is parsed"]
                    #[doc = "/ and returned as-is without making any external requests."]
                    #[doc = "/ "]
                    #[doc = "/ See the wasi-socket proposal README.md for a comparison with getaddrinfo."]
                    #[doc = "/ "]
                    #[doc = "/ The results are returned in connection order preference."]
                    #[doc = "/ "]
                    #[doc = "/ This function never succeeds with 0 results. It either fails or succeeds"]
                    #[doc = "/ with at least one address. Additionally, this function never returns"]
                    #[doc = "/ IPv4-mapped IPv6 addresses."]
                    #[doc = "/ "]
                    #[doc = "/ # References:"]
                    #[doc = "/ - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/getaddrinfo.html>"]
                    #[doc = "/ - <https://man7.org/linux/man-pages/man3/getaddrinfo.3.html>"]
                    #[doc = "/ - <https://learn.microsoft.com/en-us/windows/win32/api/ws2tcpip/nf-ws2tcpip-getaddrinfo>"]
                    #[doc = "/ - <https://man.freebsd.org/cgi/man.cgi?query=getaddrinfo&sektion=3>"]
                    fn resolve_addresses(
                        accessor: &wasmtime::component::Accessor<T, Self>,
                        name: wasmtime::component::__internal::String,
                    ) -> impl ::core::future::Future<
                        Output = wasmtime::Result<
                            Result<wasmtime::component::__internal::Vec<IpAddress>, ErrorCode>,
                        >,
                    > + Send;
                }
                pub trait Host: Send {}
                impl<_T: Host + ?Sized + Send> Host for &mut _T {}
                pub fn add_to_linker_instance<T, D>(
                    inst: &mut wasmtime::component::LinkerInstance<'_, T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static + Send,
                {
                    inst.func_wrap_concurrent("resolve-addresses", move|caller: &wasmtime::component::Accessor::<T>, (arg0,): (wasmtime::component::__internal::String,)|{
                        use tracing::Instrument;
                        let span = tracing::span!(tracing::Level::TRACE, "wit-bindgen import", module = "ip-name-lookup", function = "resolve-addresses",);
                        wasmtime::component::__internal::Box::pin(async move {
                            tracing::event!(tracing::Level::TRACE, name = tracing::field::debug(&arg0), "call");
                            let host =  &caller.with_getter::<D>(host_getter);
                            let r =  <D as HostWithStore<T>>::resolve_addresses(host, arg0,).await;
                            tracing::event!(tracing::Level::TRACE, result = tracing::field::debug("..."), "return");
                            Ok((wasmtime::ToWasmtimeResult::to_wasmtime_result(r)?,))
                        }.instrument(span))
                    })?;
                    Ok(())
                }
                pub fn add_to_linker<T, D>(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: HostWithStore<T>,
                    for<'a> D::Data<'a>: Host,
                    T: 'static + Send,
                {
                    let mut inst = linker.instance("wasi:sockets/ip-name-lookup@0.3.0")?;
                    add_to_linker_instance::<T, D>(&mut inst, host_getter)
                }
            }
        }
    }
    pub mod exports {
        pub mod wasi {
            pub mod cli {
                #[allow(clippy::all)]
                pub mod run {
                    #[allow(unused_imports)]
                    use wasmtime::component::__internal::Box;
                    #[derive(Clone)]
                    pub struct Guest {
                        run: wasmtime::component::Func,
                    }
                    #[derive(Clone)]
                    pub struct GuestIndices {
                        run: wasmtime::component::ComponentExportIndex,
                    }
                    impl GuestIndices {
                        #[doc = "/ Constructor for [`GuestIndices`] which takes a"]
                        #[doc = "/ [`Component`](wasmtime::component::Component) as input and can be executed"]
                        #[doc = "/ before instantiation."]
                        #[doc = "/"]
                        #[doc = "/ This constructor can be used to front-load string lookups to find exports"]
                        #[doc = "/ within a component."]
                        pub fn new<_T>(
                            _instance_pre: &wasmtime::component::InstancePre<_T>,
                        ) -> wasmtime::Result<GuestIndices> {
                            let instance = _instance_pre
                                .component()
                                .get_export_index(None, "wasi:cli/run@0.3.0")
                                .ok_or_else(|| {
                                    wasmtime::format_err!(
                                        "no exported instance named `wasi:cli/run@0.3.0`"
                                    )
                                })?;
                            let mut lookup = move |name: &str| {
                                _instance_pre.component().get_export_index(Some(&instance), name).ok_or_else(||{
                                    wasmtime::format_err!("instance export `wasi:cli/run@0.3.0` does \
                                                                                                                                                                                                                                                  not have export `{name}`")
                                })
                            };
                            let _ = &mut lookup;
                            let run = lookup("run")?;
                            Ok(GuestIndices { run })
                        }
                        pub fn load(
                            &self,
                            mut store: impl wasmtime::AsContextMut,
                            instance: &wasmtime::component::Instance,
                        ) -> wasmtime::Result<Guest> {
                            let _instance = instance;
                            let _instance_pre = _instance.instance_pre(&store);
                            let _instance_type = _instance_pre.instance_type();
                            let mut store = store.as_context_mut();
                            let _ = &mut store;
                            let run = *_instance
                                .get_typed_func::<(), (Result<(), ()>,)>(&mut store, &self.run)?
                                .func();
                            Ok(Guest { run })
                        }
                    }
                    impl Guest {
                        pub fn func_run(
                            &self,
                        ) -> wasmtime::component::TypedFunc<(), (Result<(), ()>,)>
                        {
                            unsafe {
                                wasmtime::component::TypedFunc::<(), (Result<(), ()>,)>::new_unchecked(self.run)
                            }
                        }
                        #[doc = "/ Run the program."]
                        pub async fn call_run<_T, _D>(
                            &self,
                            accessor: &wasmtime::component::Accessor<_T, _D>,
                        ) -> wasmtime::Result<Result<(), ()>>
                        where
                            _T: Send,
                            _D: wasmtime::component::HasData,
                        {
                            let callee = self.func_run();
                            let (ret0,) = callee.call_concurrent(accessor, ()).await?;
                            Ok(ret0)
                        }
                    }
                }
            }
        }
    }
    const _: &[u8] = include_bytes!(
        r#"/home/someone/repos/webrogue/external/wasmtime/crates/wasi/src/p3/wit/package.wit"#
    );
    const _: &[u8] = include_bytes!(
        r#"/home/someone/repos/webrogue/external/wasmtime/crates/wasi/src/p3/wit/deps/cli.wit"#
    );
    const _: &[u8] = include_bytes!(
        r#"/home/someone/repos/webrogue/external/wasmtime/crates/wasi/src/p3/wit/deps/clocks.wit"#
    );
    const _: &[u8] = include_bytes!(
        r#"/home/someone/repos/webrogue/external/wasmtime/crates/wasi/src/p3/wit/deps/filesystem.wit"#
    );
    const _: &[u8] = include_bytes!(
        r#"/home/someone/repos/webrogue/external/wasmtime/crates/wasi/src/p3/wit/deps/random.wit"#
    );
    const _: &[u8] = include_bytes!(
        r#"/home/someone/repos/webrogue/external/wasmtime/crates/wasi/src/p3/wit/deps/sockets.wit"#
    );
}
pub use self::generated::LinkOptions;
pub use self::generated::exports;
pub use self::generated::wasi::*;

/// Bindings to execute and run a `wasi:cli/command`.
///
/// This structure is automatically generated by `bindgen!`.
///
/// This can be used for a more "typed" view of executing a command component
/// through the [`Command::wasi_cli_run`] method plus
/// [`Guest::call_run`](exports::wasi::cli::run::Guest::call_run).
///
/// # Examples
///
/// ```no_run
/// use wasmtime::{Engine, Result, Store, Config};
/// use wasmtime::component::{Component, Linker, ResourceTable};
/// use wasmtime_wasi::{WasiCtx, WasiCtxView, WasiView};
/// use wasmtime_wasi::p3::bindings::Command;
///
/// // This example is an example shim of executing a component based on the
/// // command line arguments provided to this program.
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let args = std::env::args().skip(1).collect::<Vec<_>>();
///
///     // Configure and create `Engine`
///     let mut config = Config::new();
///     config.wasm_component_model_async(true);
///     let engine = Engine::new(&config)?;
///
///     // Configure a `Linker` with WASI, compile a component based on
///     // command line arguments, and then pre-instantiate it.
///     let mut linker = Linker::<MyState>::new(&engine);
///     wasmtime_wasi::p3::add_to_linker(&mut linker)?;
///     let component = Component::from_file(&engine, &args[0])?;
///
///
///     // Configure a `WasiCtx` based on this program's environment. Then
///     // build a `Store` to instantiate into.
///     let mut builder = WasiCtx::builder();
///     builder.inherit_stdio().inherit_env().args(&args);
///     let mut store = Store::new(
///         &engine,
///         MyState {
///             ctx: builder.build(),
///             table: ResourceTable::default(),
///         },
///     );
///
///     // Instantiate the component and we're off to the races.
///     let command = Command::instantiate_async(&mut store, &component, &linker).await?;
///     let program_result = store.run_concurrent(async move |store| {
///         command.wasi_cli_run().call_run(store).await
///     }).await??;
///     match program_result {
///         Ok(()) => Ok(()),
///         Err(()) => std::process::exit(1),
///     }
/// }
///
/// struct MyState {
///     ctx: WasiCtx,
///     table: ResourceTable,
/// }
///
/// impl WasiView for MyState {
///     fn ctx(&mut self) -> WasiCtxView<'_> {
///         WasiCtxView{
///             ctx: &mut self.ctx,
///             table: &mut self.table,
///         }
///     }
/// }
/// ```
///
/// ---
pub use self::generated::Command;

/// Pre-instantiated analog of [`Command`]
///
/// This can be used to front-load work such as export lookup before
/// instantiation.
///
/// # Examples
///
/// ```no_run
/// use wasmtime::{Engine, Result, Store, Config};
/// use wasmtime::component::{Linker, Component, ResourceTable};
/// use wasmtime_wasi::{WasiCtx, WasiCtxView, WasiView};
/// use wasmtime_wasi::p3::bindings::CommandPre;
///
/// // This example is an example shim of executing a component based on the
/// // command line arguments provided to this program.
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let args = std::env::args().skip(1).collect::<Vec<_>>();
///
///     // Configure and create `Engine`
///     let mut config = Config::new();
///     config.wasm_component_model_async(true);
///     let engine = Engine::new(&config)?;
///
///     // Configure a `Linker` with WASI, compile a component based on
///     // command line arguments, and then pre-instantiate it.
///     let mut linker = Linker::<MyState>::new(&engine);
///     wasmtime_wasi::p3::add_to_linker(&mut linker)?;
///     let component = Component::from_file(&engine, &args[0])?;
///     let pre = CommandPre::new(linker.instantiate_pre(&component)?)?;
///
///
///     // Configure a `WasiCtx` based on this program's environment. Then
///     // build a `Store` to instantiate into.
///     let mut builder = WasiCtx::builder();
///     builder.inherit_stdio().inherit_env().args(&args);
///     let mut store = Store::new(
///         &engine,
///         MyState {
///             ctx: builder.build(),
///             table: ResourceTable::default(),
///         },
///     );
///
///     // Instantiate the component and we're off to the races.
///     let command = pre.instantiate_async(&mut store).await?;
///     // TODO: Construct an accessor from `store` to call `run`
///     // https://github.com/bytecodealliance/wasmtime/issues/11249
///     //let program_result = command.wasi_cli_run().call_run(&mut store).await?;
///     let program_result = todo!();
///     match program_result {
///         Ok(()) => Ok(()),
///         Err(()) => std::process::exit(1),
///     }
/// }
///
/// struct MyState {
///     ctx: WasiCtx,
///     table: ResourceTable,
/// }
///
/// impl WasiView for MyState {
///     fn ctx(&mut self) -> WasiCtxView<'_> {
///         WasiCtxView{
///             ctx: &mut self.ctx,
///             table: &mut self.table,
///         }
///     }
/// }
/// ```
///
/// ---
// TODO: Make this public, once `CommandPre` can be used for
// calling exports
// https://github.com/bytecodealliance/wasmtime/issues/11249
#[doc(hidden)]
pub use self::generated::CommandPre;

pub use self::generated::CommandIndices;
