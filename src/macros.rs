// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

macro_rules! deserialize {
    (
        { $field:ident, $($rest:tt)* }
        $count:tt
        { $($acc:tt)* }
        $name:ident
        $deserializer:ident
        $T:ty
    ) => (
        deserialize!(
            { $($rest)* }
            (1 + $count)
            { $($acc)* { $field $count } }
            $name
            $deserializer
            $T)
    );
    (
        {}
        $total:tt
        { $({ $field:ident $index:expr })+ }
        $name:ident
        $deserializer:ident
        $T:ty
    ) => ({
        let values = try!(<[$T; $total]>::deserialize($deserializer));
        Ok($name { $($field: values[$index].clone(),)+ _unit: PhantomData })
    })
}

macro_rules! define_matrix {
    (
        $(#[$attr:meta])*
        pub struct $name:ident<T, $($phantom:ident),+> {
            $(pub $field:ident: T,)+
        }
    ) => (
        $(#[$attr])*
        pub struct $name<T, $($phantom),+> {
            $(pub $field: T,)+
            _unit: PhantomData<($($phantom),+)>
        }

        impl<T, $($phantom),+> ::heapsize::HeapSizeOf for $name<T, $($phantom),+>
            where T: ::heapsize::HeapSizeOf
        {
            fn heap_size_of_children(&self) -> usize {
                $(self.$field.heap_size_of_children() +)+ 0
            }
        }

        impl<T, $($phantom),+> ::serde::Deserialize for $name<T, $($phantom),+>
            where T: Clone + ::serde::Deserialize
        {
            fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
                where D: ::serde::Deserializer
            {
                deserialize!({ $($field,)+ } 0 {} $name deserializer T)
            }
        }

        impl<T, $($phantom),+> ::serde::Serialize for $name<T, $($phantom),+>
            where T: ::serde::Serialize
        {
            fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
                where S: ::serde::Serializer
            {
                [$(&self.$field,)+].serialize(serializer)
            }
        }
    )
}
