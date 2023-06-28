//! Rust study chapter 3 example code for type layout
//!
//! The alignment of a value specifies what addresses are valid to store the value at.
//! A value of alignment n must only be stored at an address that is a multiple of n.
//! For example, a value with an alignment of 2 must be stored at an even address, while a value with an alignment of 1 can be stored at any address.
//! Alignment is measured in bytes, and must be at least 1, and always a power of 2. The alignment of a value can be checked with the align_of_val function.
//!
//! The size of a value is the offset in bytes between successive elements in an array with that item type including alignment padding.
//! The size of a value is always a multiple of its alignment.
//! Note that some types are zero-sized; 0 is considered a multiple of any alignment (for example, on some platforms, the type [u16; 0] has size 0 and alignment 2).
//! The size of a value can be checked with the size_of_val function.
//!
//!

// 0. primitive data layout

struct A;

struct B {
    first: i64,  //8 bytes
    second: i32, //4 bytes
}

fn find_size1<T: ?Sized>() {}
fn find_size2<T: Sized>() {}

fn primitive_size() {
    println!("{:?}", std::mem::size_of::<bool>());
    println!("{:?}", std::mem::size_of::<u16>());
    println!("{:?}", std::mem::size_of::<usize>());
    println!("{:?}", std::mem::size_of::<char>());
    println!("{:?}", std::mem::size_of::<A>());
    println!("{:?}", std::mem::size_of::<B>());
    println!("{:?}", std::mem::size_of::<&mut B>()); //should have same size as usize
    println!("{:?}", std::mem::size_of::<*mut B>()); //should have same size as usize
    println!("{:?}", std::mem::size_of::<&str>()); //should have double size as pointer of sized type since &str is DST(thus unsized) since it should have two pointers.
                                                   //pointer1 points to the value
                                                   //for slices -> second 8bytes store number of elements of the slice
                                                   //for trait object -> second 8bytes store nunmber of element of the trait object
    println!("{:?}", std::mem::size_of::<[usize; 3]>());
}

//alignemt is always factor of 2
fn primitive_alignment() {
    println!("{:?}", std::mem::align_of::<bool>());
    println!("{:?}", std::mem::align_of::<u16>());
    println!("{:?}", std::mem::size_of::<usize>());
    println!("{:?}", std::mem::align_of::<char>());
    println!("{:?}", std::mem::align_of::<A>());
    println!("{:?}", std::mem::align_of::<B>()); // this should be the largest alignment among the fields
    println!("{:?}", std::mem::align_of::<&mut B>());
    println!("{:?}", std::mem::align_of::<*mut B>());
    println!("{:?}", std::mem::align_of::<&str>()); //should have same size as usize
    println!("{:?}", std::mem::align_of::<[usize; 3]>());
}

// Representations

// All user-defined composite types (structs, enums, and unions) have a representation that specifies what the layout is for the type.
// The possible representations for a type are: Default(rust), C, primitive, transparent

// 1. Default
// The fields are properly aligned.
// The fields do not overlap.
// The alignment of the type is at least the maximum alignment of its fields.

// field declaration order does not matter!
struct DefaultA {
    a: i32,
    b: i64,
}
//this could [4, 8] or [8, 4], [8, 8] all possible
//alignment is min(max(i64, i32)) => could be any value greater than 8

enum DefaultEnum {
    A(i32),
    B(i32),
    C(i32),
    D(i32),
}

enum DefaultEnumSingle {
    A(i32),
}

fn default_representation() {
    println!("{:?}", std::mem::size_of::<DefaultA>());
    println!("{:?}", std::mem::align_of::<DefaultA>());

    println!("{:?}", std::mem::size_of::<DefaultEnum>()); // this is 8 = max size + tag
    println!("{:?}", std::mem::align_of::<DefaultEnum>()); //this is 4

    println!("{:?}", std::mem::size_of::<DefaultEnumSingle>()); //this is 4 = no tag
    println!("{:?}", std::mem::align_of::<DefaultEnumSingle>());
}

//2. C
// The alignment of the struct is the alignment of the most-aligned field in it.
// Start with a current offset of 0 bytes.
// For each field in declaration order in the struct, first determine the size and alignment of the field. If the current offset is not a multiple of the field's alignment,
// then add padding bytes to the current offset until it is a multiple of the field's alignment.
// The offset for the field is what the current offset is now. Then increase the current offset by the size of the field.
// Finally, the size of the struct is the current offset rounded up to the nearest multiple of the struct's alignment.

#[repr(C)]
struct CA {
    a: i32,
    b: i64,
}

#[repr(C)]
enum CEnum {
    A(i32),
    B(i32),
    C(i32),
    D(i32),
}
#[repr(C)]
enum CEnumSingle {
    A(i32),
}

fn c_representation() {
    println!("{:?}", std::mem::size_of::<CA>());
    println!("{:?}", std::mem::align_of::<CA>());

    println!("{:?}", std::mem::size_of::<CEnum>());
    println!("{:?}", std::mem::align_of::<CEnum>());

    println!("{:?}", std::mem::size_of::<CEnumSingle>());
    println!("{:?}", std::mem::align_of::<CEnumSingle>());
}

// // ... this struct.
// #[repr(C)]
// struct CEnumRepr {
//     tag: MyEnumDiscriminant,
//     payload: MyEnumFields,
// }

// // This is the discriminant enum.
// #[repr(C)]
// enum MyEnumDiscriminant { A, B, C, D }


// For field-less enums, the C representation has the size and alignment of the default enum size and alignment for the target platform's C ABI.

// // This is the variant union.
// #[repr(C)]
// union MyEnumFields {
//     A: MyAFields,
//     B: MyBFields,
//     C: MyCFields,
//     D: MyDFields,
// }


// first offset 0
// a: size 4, alignment 4 , offset += 4
// b: size 8, alignment 8, sicne offset is not multiple of 8, add 4 padding bytes, offset += 4
// start from offset = 8, offset += 8
// thus memory layout [8,8] is the only possible answer d

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        primitive_size();
    }

    #[test]
    fn second() {
        primitive_alignment()
    }

    #[test]
    fn third() {
        default_representation();
    }

    #[test]
    fn fourth() {
        c_representation()
    }
}
