// use std::any::{Any, TypeId};
// use std::collections::hash_map::{IterMut, Keys};
//
// use rustc_hash::FxHashMap;
//
// use crate::wgpu::pipelines::Pipeline;
//
// #[derive(Default, Debug)]
// pub(crate) struct PipelineStorage {
//     pub(crate) inner: FxHashMap<TypeId, Box<dyn Pipeline + Send>>,
// }
//
// impl PipelineStorage {
//     /// Returns key of data
//     pub fn keys(&self) -> Keys<'_, TypeId, Box<dyn Any + Send>> {
//         self.inner.keys()
//     }
//
//     /// Returns `true` if `Storage` contains a type `T`.
//     pub fn has<T: Pipeline + 'static>(&self) -> bool {
//         self.inner.contains_key(&TypeId::of::<T>())
//     }
//
//     /// Inserts the data `T` in to [`Storage`].
//     pub fn store<T: Pipeline + 'static + Send>(&mut self, pipeline: Box<dyn Pipeline + Send>) {
//         let _ = self.inner.insert(TypeId::of::<T>(), pipeline);
//     }
//
//     /// Returns a reference to the data with type `T` if it exists in [`Storage`].
//     pub fn get<T: Pipeline + 'static>(&self) -> Option<&Box<dyn Pipeline + Send>> {
//         self.inner.get(&TypeId::of::<T>())
//     }
//
//     /// Returns a mutable reference to the data with type `T` if it exists in [`Storage`].
//     pub fn get_mut<T: Pipeline + 'static>(&mut self) -> Option<&mut Box<dyn Pipeline + Send>> {
//         self.inner.get_mut(&TypeId::of::<T>())
//     }
//
//     pub fn iter_mut(&mut self) -> IterMut<'_, TypeId, Box<dyn Any + Send>> {
//         self.inner.iter_mut()
//     }
// }

use std::any::{Any, TypeId};
use std::collections::hash_map::{Iter, IterMut, Keys};
use std::collections::{HashMap, HashSet, LinkedList};
use std::marker::PhantomData;

use rustc_hash::FxHashMap;
use crate::pipelines::Pipeline;

#[derive(Default, Debug)]
pub(crate) struct PipelineStorage {
    pub(crate) inner: FxHashMap<TypeId, Box<dyn Any + Send>>,


}

impl PipelineStorage {
    /// Returns key of data
    pub fn keys(&self) -> Keys<'_, TypeId, Box<dyn Any + Send>> {
        self.inner.keys()
    }

    /// Returns `true` if `Storage` contains a type `T`.
    pub fn has<T: 'static>(&self) -> bool {
        self.inner.contains_key(&TypeId::of::<T>())
    }

    /// Inserts the data `T` in to [`Storage`].
    pub fn store<T: 'static + Send>(&mut self, data: T) {
        let _ = self.inner.insert(TypeId::of::<T>(), Box::new(data));
    }

    /// Returns a reference to the data with type `T` if it exists in [`Storage`].
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.inner.get(&TypeId::of::<T>()).map(|pipeline| {
            pipeline
                .downcast_ref::<T>()
                .expect("Value with this type does not exist in Storage.")
        })
    }

    /// Returns a mutable reference to the data with type `T` if it exists in [`Storage`].
    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.inner.get_mut(&TypeId::of::<T>()).map(|pipeline| {
            pipeline
                .downcast_mut::<T>()
                .expect("Value with this type does not exist in Storage.")
        })
    }
    pub fn get_mut_by_id(&mut self, id : &TypeId) -> Option<&mut Box<dyn Pipeline>> {
        self.inner.get_mut(&id).map(|pipeline| {
            pipeline
                .downcast_mut::<Box<dyn Pipeline>>()
                .expect("Value with this type does not exist in Storage.")
        })
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, TypeId, Box<dyn Any + Send>> {
        self.inner.iter_mut()
    }
}
