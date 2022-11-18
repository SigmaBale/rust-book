//Implementing an Unsafe Trait
/*
A trait is unsafe when at least one of its methods has some invariant that the compiler can’t verify.
We can declare that a trait is unsafe by adding the unsafe keyword before trait and marking the implementation of the trait as unsafe too.
By using unsafe impl, we’re promising that we’ll uphold the invariants that the compiler can’t verify.
If we implement a type that contains a type that is not Send or Sync, such as raw pointers, and we want to mark that type as Send or Sync, we must use unsafe.
*/
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}


//Accessing Fields of a Union
/*
The final action that works only with unsafe is accessing fields of a union. 
A union is similar to a struct, but only one declared field is used in a particular instance at one time. 
Unions are primarily used to interface with unions in C code. 
Accessing union fields is unsafe because Rust can’t guarantee the type of the data currently being stored in the union instance.

A union declaration uses the same syntax as a struct declaration, except with union in place of struct.
The key property of unions is that all fields of a union share common storage. 
As a result, writes to one field of a union can overwrite its other fields, and size of a union is determined by the size of its largest field.
*/
#[repr(C)]
union _MyUnion {
    f1: u32,
    f2: f32,
}
/*
A value of a union type can be created using the same syntax that is used for struct types, except that it must specify exactly one field:

    let u = MyUnion { f1: 1 };


The expression above creates a value of type MyUnion and initializes the storage using field f1. 
The union can be accessed using the same syntax as struct fields:

    let f = unsafe { u.f1 };


Consequently, all reads of union fields have to be placed in unsafe blocks:

    unsafe {
        let f = u.f1;
    }

Writes to Copy or ManuallyDrop union fields do not require reads for running destructors, so these writes don't have to be placed in unsafe blocks.

    u.f1 = 2;
    u.f2 = ManuallyDrop::new(String::from("example"));

Commonly, code using unions will provide safe wrappers around unsafe union field accesses.
*/

