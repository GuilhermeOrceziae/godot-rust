use gdnative::prelude::*;

pub(crate) fn run_tests() -> bool {
    let mut status = true;

    status &= test_register_property();

    status
}

pub(crate) fn register(handle: InitHandle) {
    handle.add_class::<RegisterSignal>();
    handle.add_class::<RegisterProperty>();
}

#[derive(Copy, Clone, Debug, Default)]
struct RegisterSignal;

impl NativeClass for RegisterSignal {
    type Base = Reference;
    type UserData = user_data::Aether<RegisterSignal>;
    fn class_name() -> &'static str {
        "RegisterSignal"
    }
    fn init(_owner: TRef<Reference>) -> RegisterSignal {
        RegisterSignal
    }
    fn register_properties(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "progress",
            args: &[SignalArgument {
                name: "amount",
                default: Variant::new(),
                export_info: ExportInfo::new(VariantType::I64),
                usage: PropertyUsage::DEFAULT,
            }],
        });
    }
}

#[methods]
impl RegisterSignal {}

struct RegisterProperty {
    value: i64,
}

impl NativeClass for RegisterProperty {
    type Base = Reference;
    type UserData = user_data::MutexData<RegisterProperty>;
    fn class_name() -> &'static str {
        "RegisterProperty"
    }
    fn init(_owner: TRef<Reference>) -> RegisterProperty {
        RegisterProperty { value: 42 }
    }
    fn register_properties(builder: &ClassBuilder<Self>) {
        builder
            .add_property("value")
            .with_default(42)
            .with_setter(RegisterProperty::set_value)
            .with_getter(RegisterProperty::get_value)
            .done();
    }
}

#[methods]
impl RegisterProperty {
    #[export]
    fn set_value(&mut self, _owner: TRef<Reference>, value: i64) {
        self.value = value;
    }

    #[export]
    fn get_value(&self, _owner: TRef<Reference>) -> i64 {
        self.value
    }
}

fn test_register_property() -> bool {
    println!(" -- test_register_property");

    let ok = std::panic::catch_unwind(|| {
        let obj = RegisterProperty::new_instance();

        let base = obj.into_base();

        assert_eq!(Some(42), unsafe {
            base.call("get_value", &[]).try_to_i64()
        });

        base.set("value", 54.to_variant());

        assert_eq!(Some(54), unsafe {
            base.call("get_value", &[]).try_to_i64()
        });

        unsafe { base.call("set_value", &[4242.to_variant()]) };

        assert_eq!(Some(4242), unsafe {
            base.call("get_value", &[]).try_to_i64()
        });
    })
    .is_ok();

    if !ok {
        gdnative::godot_error!("   !! Test test_register_property failed");
    }

    ok
}
