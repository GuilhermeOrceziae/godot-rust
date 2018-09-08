use json::*;
use std::fs::File;
use std::io::Write;

use find_class;

use heck::SnakeCase;

pub fn generate_refreference_ctor(output: &mut File, class: &GodotClass) {
    writeln!(output,
r#"
    // Constructor
    pub fn new() -> Self {{
        unsafe {{
            let gd_api = ::get_api();
            let ctor = {name}MethodTable::get(gd_api).class_constructor.unwrap();
            let obj = ctor();
            object::init_ref_count(obj);

            {name} {{
                this: obj
            }}
        }}
    }}

    /// Creates a new reference to the same reference-counted object.
    pub fn new_ref(&self) -> Self {{
        unsafe {{
            object::add_ref(self.this);

            Self {{
                this: self.this,
            }}
        }}
    }}
"#,
        name = class.name
    ).unwrap();
}

pub fn generate_non_refreference_ctor(output: &mut File, class: &GodotClass) {
    writeln!(output,
r#"
    /// Constructor.
    ///
    /// Because this type is not reference counted, the lifetime of the returned object
    /// is *not* automatically managed.
    /// Immediately after creation, the object is owned by the caller, and can be
    /// passed to the engine (in which case the engine will be responsible for
    /// destroying the object) or destroyed manually using `{name}::free`.
    pub fn new() -> Self {{
        unsafe {{
            let gd_api = ::get_api();
            let ctor = {name}MethodTable::get(gd_api).class_constructor.unwrap();
            let this = ctor();

            {name} {{
                this
            }}
        }}
    }}

    /// Manually deallocate the object.
    #[inline]
    pub unsafe fn free(self) {{
        (get_api().godot_object_destroy)(self.this);
    }}
"#,
        name = class.name
    ).unwrap();
}

pub fn generate_godot_object_impl(output: &mut File, class: &GodotClass) {
    writeln!(output, r#"

unsafe impl GodotObject for {name} {{
    fn class_name() -> &'static str {{
        "{name}"
    }}

    unsafe fn from_sys(obj: *mut sys::godot_object) -> Self {{
        {addref_if_reference}
        Self {{ this: obj, }}
    }}

    unsafe fn to_sys(&self) -> *mut sys::godot_object {{
        self.this
    }}
}}

"#,
        name = class.name,
        addref_if_reference = if class.is_refcounted() {
            "object::add_ref(obj);"
        } else {
            "// Not reference-counted."
        }
    ).unwrap();
}

pub fn generate_singleton_getter(output: &mut File, class: &GodotClass) {
    let s_name = if class.name.starts_with("_") {
        &class.name[1..]
    } else {
        class.name.as_ref()
    };

    writeln!(output, r#"
    #[inline]
    pub fn godot_singleton() -> Self {{
        unsafe {{
            let this = (get_api().godot_global_get_singleton)(b"{s_name}\0".as_ptr() as *mut _);

            {name} {{
                this
            }}
        }}
    }}"#,
        name = class.name,
        s_name = s_name
    ).unwrap();
}

pub fn generate_dynamic_cast(output: &mut File, class: &GodotClass) {
    writeln!(output,
r#"
    /// Generic dynamic cast.
    pub {maybe_unsafe}fn cast<T: GodotObject>(&self) -> Option<T> {{
        object::godot_cast::<T>(self.this)
    }}
"#,
        maybe_unsafe = if class.is_pointer_safe() { "" } else { "unsafe " },
    ).unwrap();
}


pub fn generate_upcast(
    output: &mut File,
    classes: &[GodotClass],
    base_class_name: &str,
    is_pointer_safe: bool,
) {
    if let Some(parent) = find_class(classes, &base_class_name) {
        let snake_name = class_name_to_snake_case(&base_class_name);
        if is_pointer_safe {
            writeln!(output,
r#"    /// Up-cast.
    #[inline]
    pub fn to_{snake_name}(&self) -> {name} {{
        {addref_if_reference}
        {name} {{ this: self.this }}
    }}
"#,
                name = parent.name,
                snake_name = snake_name,
                addref_if_reference = if parent.is_refcounted() {
                    "unsafe {{ object::add_ref(self.this); }}"
                } else {
                    "// Not reference-counted."
                },
            ).unwrap();
        } else {
            writeln!(output,
r#"    /// Up-cast.
    #[inline]
    pub unsafe fn to_{snake_name}(&self) -> {name} {{
        {addref_if_reference}
        {name} {{ this: self.this }}
    }}
"#,
                name = parent.name,
                snake_name = snake_name,
                addref_if_reference = if parent.is_refcounted() {
                    "object::add_ref(self.this);"
                } else {
                    "// Not reference-counted."
                },
            ).unwrap();
        }

        generate_upcast(
            output,
            classes,
            &parent.base_class,
            is_pointer_safe,
        );
    }
}

pub fn generate_drop(output: &mut File, class: &GodotClass) {
    writeln!(output,
r#"
impl Drop for {name} {{
    fn drop(&mut self) {{
        unsafe {{
            if object::unref(self.this) {{
                (::get_api().godot_object_destroy)(self.this);
            }}
        }}
    }}
}}
"#,
        name = class.name
    ).unwrap();
}

pub fn class_name_to_snake_case(name: &str) -> String {
    // TODO: this is a quick-n-dirty band-aid, it'd be better to
    // programmatically do the right conversion, but to_snake_case
    // currently translates "Node2D" into "node2_d".
    match name {
        "SpriteBase3D" => "sprite_base_3d".to_string(),
        "Node2D" => "node_2d".to_string(),
        "CollisionObject2D" => "collision_object_2d".to_string(),
        "PhysicsBody2D" => "physics_body_2d".to_string(),
        "VisibilityNotifier2D" => "visibility_notifier_2d".to_string(),
        "Joint2D" => "joint_2d".to_string(),
        "Shape2D" => "shape_2d".to_string(),
        "Physics2DServer" => "physics_2d_server".to_string(),
        "Physics2DDirectBodyState" => "physics_2d_direct_body_state".to_string(),
        _ => name.to_snake_case(),
    }
}
