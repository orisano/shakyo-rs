fn main() {
    trait Foo {
        fn foo() -> i32;
    }

    struct Bar;

    impl Bar {
        fn foo() -> i32 {
            20
        }
    }

    impl Foo for Bar {
        fn foo() -> i32 {
            10
        }
    }

    assert_eq!(10, <Bar as Foo>::foo());
    assert_eq!(20, Bar::foo());
}
