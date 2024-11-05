extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::parse::Parser;
use syn::{parse, parse_macro_input, DeriveInput};

/// Example of [function-like procedural macro][1].
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#function-like-procedural-macros
// #[proc_macro]
// pub fn my_macro(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//
//     let tokens = quote! {
//         #input
//
//         struct Hello;
//     };
//
//     tokens.into()
// }

/// Example of user-defined [derive mode macro][1]
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-mode-macros
#[proc_macro_derive(object3d)]
pub fn object_3d_derive(input: TokenStream) -> TokenStream {
    // 解析输入的语法树
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let tokens = quote! {
        impl Object3D for #name {
            fn id(&self) -> u32 {
                self.id
            }
            fn mark_dirty(&mut self) {
                self.is_self_dirty = true;
                let mut p = self.get_parent();
                while p.is_some() {
                    {p.as_mut().unwrap().borrow_mut().mark_child_dirty();}
                    {
                        let t = p.as_mut().unwrap().borrow().get_parent();
                         p = t.clone()
                    }
                }
            }
            fn is_dirty(&self) -> bool {
                self.is_self_dirty
            }
            fn mark_child_dirty(&mut self) {
                self.is_child_dirty = true;
            }
            fn has_child_dirty(&self) -> bool {
                self.is_child_dirty
            }
            fn add_child(&mut self, child : Rc<RefCell<Box<dyn Object3D>>>) -> bool {
                if child.borrow().get_parent().is_some() {
                    return false;
                }
                if self.children.iter().find(|&e| e.borrow().id() == child.borrow().id()).is_some() {
                    return false;
                };
                child.borrow_mut().set_parent(self.this.clone());
                self.children.push(child);
                return true;
            }
            fn remove_child(&mut self, child_id : u32) -> bool {
                let mut r = false;
                self.children.retain(|v| {
                    if v.borrow().id() == child_id {
                        r = true;
                        v.borrow_mut().set_parent(None);
                        false  // not keep
                    } else {

                        true
                    }
                });
                return r;
            }

            fn get_child_by_index(&self, index: usize) -> Rc<RefCell<Box<dyn Object3D>>> {
                let t = self.children.get(index).unwrap().clone();
                t
            }
            fn child_num(&self) -> usize {
                return self.children.len();
            }

            fn set_this(&mut self, this: Rc<RefCell<Box<dyn Object3D>>>) {
                self.this = Some(this);
            }

            fn set_parent(&mut self, parent : Option<Rc<RefCell<Box<dyn Object3D>>>>) {
                self.parent = parent;
            }
            fn get_parent(&self) -> Option<Rc<RefCell<Box<dyn Object3D>>>> {
                self.parent.clone()
            }

            fn to_object(self) -> Rc<RefCell<Box<dyn Object3D>>> {
                let t: Box<dyn Object3D> = Box::new(self);
                let this = Rc::new(RefCell::new(t));
                this.borrow_mut().set_this(this.clone());
                this
            }

            fn to_primitive(&self) -> Option<Rc<RefCell<Box<dyn Primitive>>>> {
                self.primitive.clone()
            }
        }

    };

    tokens.into()
}

/// 属性宏
/// 给 object3d对象添加属性
#[proc_macro_attribute]
pub fn add_object3d_attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    println!("attr: \"{args}\"");
    println!("item: \"{input}\"");

    let _ = parse_macro_input!(args as parse::Nothing);
    let mut item_struct = parse_macro_input!(input as DeriveInput);

    match &mut item_struct.data {
        syn::Data::Struct(ref mut struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields.named.push(syn::Field::parse_named.parse2(quote! { id : u32 }).unwrap());
                    fields
                        .named
                        .push(syn::Field::parse_named.parse2(quote! {is_self_dirty : bool }).unwrap());
                    fields
                        .named
                        .push(syn::Field::parse_named.parse2(quote! {is_child_dirty : bool }).unwrap());
                    fields
                        .named
                        .push(syn::Field::parse_named.parse2(quote! { position : glam::Mat4 }).unwrap());
                    fields.named.push(
                        syn::Field::parse_named
                            .parse2(quote! { parent: Option<Rc<RefCell<Box<dyn Object3D>>>> })
                            .unwrap(),
                    );
                    fields.named.push(
                        syn::Field::parse_named
                            .parse2(quote! { children: Vec<Rc<RefCell<Box<dyn Object3D>>>> })
                            .unwrap(),
                    );
                    fields.named.push(
                        syn::Field::parse_named
                            .parse2(quote! { this: Option<Rc<RefCell<Box<dyn Object3D>>>> })
                            .unwrap(),
                    );
                    fields.named.push(
                        syn::Field::parse_named
                            .parse2(quote! { primitive: Option<Rc<RefCell<Box<dyn Primitive>>>> })
                            .unwrap(),
                    );
                }
                _ => (),
            }

            return quote! {
                #item_struct
            }
            .into();
        }
        _ => panic!("`add_object_attribute` has to be used with structs "),
    }
}
