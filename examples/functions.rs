#[macro_use]
extern crate dyon;

use dyon::Vec4;

fn main() {
    use dyon::{error, Runtime};

    let mut dyon_runtime = Runtime::new();
    let dyon_module = load_module().unwrap();
    if error(dyon_runtime.run(&dyon_module)) {
        return
    }
}

fn load_module() -> Option<dyon::Module> {
    use std::sync::Arc;
    use dyon::*;

    let mut module = Module::new();
    module.add(Arc::new("say_hello".into()), say_hello, Dfn {
        lts: vec![],
        tys: vec![],
        ret: Type::Void
    });
    module.add(Arc::new("homer".into()), homer, Dfn {
        lts: vec![],
        tys: vec![],
        ret: Type::Any
    });
    module.add(Arc::new("age".into()), age, Dfn {
        lts: vec![Lt::Default],
        tys: vec![Type::Any],
        ret: Type::Any
    });
    module.add(Arc::new("mr".into()), mr, Dfn {
        lts: vec![Lt::Default; 2],
        tys: vec![Type::Text; 2],
        ret: Type::Text
    });
    module.add(Arc::new("origo".into()), origo, Dfn {
        lts: vec![],
        tys: vec![],
        ret: Type::Object,
    });
    if error(load("source/functions/loader.dyon", &mut module)) {
        None
    } else {
        Some(module)
    }
}

dyon_fn!{fn say_hello() {
    println!("hi!");
}}

dyon_fn!{fn homer() -> Person {
    Person {
        first_name: "Homer".into(),
        last_name: "Simpson".into(),
        age: 48
    }
}}

dyon_fn!{fn age(person: Person) -> Person {
    Person { age: person.age + 1, ..person }
}}

dyon_fn!{fn mr(first_name: String, last_name: String) -> String {
    format!("Mr {} {}", first_name, last_name)
}}

pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub age: u32,
}

dyon_obj!{Person { first_name, last_name, age }}

pub struct PhysicalState {
    pub pos: Vec4,
    pub vel: Vec4
}

dyon_obj!{PhysicalState { pos, vel }}

dyon_fn!{fn origo() -> PhysicalState {
    PhysicalState {
        pos: [0.0, 1.0, 2.0].into(),
        vel: [3.0, 4.0, 5.0].into()
    }
}}
