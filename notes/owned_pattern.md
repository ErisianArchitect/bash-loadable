*[Table of Contents](./_toc_.md)*

# Owned Pattern
The "Owned" pattern is a way to create interfaces for Bash types that may or may not be owned.

The general structure is to have three distinct types:

- `FFIType`  
  The raw FFI type that is the exact representation of the C type. The FFI type is raw, and is only used for struct layout.
  - [Start](#ffitype)
  - [Functions](#ffitype-functions)
  - [Methods](#ffitype-methods)
  - [Traits](#ffitype-traits)
- `OwnedType`  
  The owned type is a wrapper around the FFI type, and is used for types that are owned by you. These types are automatically disposed when they are dropped.
  - [Functions](#ownedtype-functions)
  - [Methods](#ownedtype-methods)
  - [Traits](#ownedtype-traits)
- `TypeRef`  
  TypeRef is used for reference types to the FFIType. These are merely shallow views to the FFIType, can can be freely copied and do not dispose of the underlying type.
  - [Functions](#typeref-functions)
  - [Methods](#typeref-methods)
  - [Traits](#typeref-traits)
- `TypeMut`
  TypeMut is used for mutable reference types to the FFIType.
  - [Functions](#typemut-functions)
  - [Methods](#typemut-methods)
  - [Traits](#typemut-traits)

## FFIType

### `FFIType` Functions

### `FFIType` Methods
The FFIType should have methods for related operations, and the `OwnedType` and `TypeRef` should forward to those methods.
- *dispose* - If appplicable, the FFIType should have a dispose method. This should only be available to `OwnedType`.
- *copy* - If applicable, the FFIType should have a (deep) copy method. This should be available to all types.

### `FFIType` Traits
- *Clone* (shallow)
- *Copy* (shallow)

## OwnedType

### `OwnedType` Functions

### `OwnedType` Methods
- `dispose` - self explanatory
- `forget` - forget about type, do not drop.

### `OwnedType` Traits
- *Clone* (deep)
- *Drop*

## TypeRef

### `TypeRef` Functions

### `TypeRef` Methods
- `unsafe make_static` - Create a new TypeRef with a static lifetime.
- `shorten_lifetime` - Create a new TypeRef with a shorter lifetime.

### `TypeRef` Traits
- *Clone* (shallow)
- *Copy* (shallow)
- *ToOwned* (maybe??)

## TypeMut

### `TypeMut` Functions

### `TypeMut` Methods

### `TypeMut` Traits