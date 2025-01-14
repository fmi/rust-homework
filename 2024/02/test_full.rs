use solution::*;

#[test]
fn test_flat_empty() {
    let imports = &[];
    assert_eq!(format_flat(imports, Order::Original), Vec::<String>::new());
    assert_eq!(format_flat(imports, Order::Sorted), Vec::<String>::new());
}

#[test]
fn test_flat_original() {
    let imports = &[
        Import(&["my_crate", "c", "C1"]),
        Import(&["my_crate", "b"]),
        Import(&["my_crate", "x", "y", "z", "W1"]),
        Import(&["my_crate", "a"]),
        Import(&["my_crate", "c", "C2"]),
    ];

    assert_eq!(
        format_flat(imports, Order::Original),
        vec![
            "my_crate::c::C1",
            "my_crate::b",
            "my_crate::x::y::z::W1",
            "my_crate::a",
            "my_crate::c::C2",
        ]
    );
}

#[test]
fn test_flat_original_duplicates() {
    let imports = &[
        Import(&["std", "string", "String"]),
        Import(&["std", "iter", "once"]),
        Import(&["std", "iter"]),
        Import(&["std", "iter"]),
        Import(&["std", "iter", "repeat"]),
        Import(&["std", "string", "String"]),
    ];

    assert_eq!(
        format_flat(imports, Order::Original),
        vec![
            "std::string::String",
            "std::iter::once",
            "std::iter",
            "std::iter::repeat",
        ]
    );
}

#[test]
fn test_flat_sorted() {
    let imports = &[
        Import(&["my_crate", "c", "C1"]),
        Import(&["my_crate", "b"]),
        Import(&["my_crate", "x", "y", "z", "W1"]),
        Import(&["my_crate", "a"]),
        Import(&["my_crate", "c", "C2"]),
    ];

    assert_eq!(
        format_flat(imports, Order::Sorted),
        vec![
            "my_crate::a",
            "my_crate::b",
            "my_crate::c::C1",
            "my_crate::c::C2",
            "my_crate::x::y::z::W1",
        ]
    );
}

#[test]
fn test_flat_sorted_duplicates() {
    let imports = &[
        Import(&["std", "string", "String"]),
        Import(&["std", "iter", "once"]),
        Import(&["std", "iter"]),
        Import(&["std", "iter"]),
        Import(&["std", "iter", "repeat"]),
        Import(&["std", "string", "String"]),
    ];

    assert_eq!(
        format_flat(imports, Order::Sorted),
        vec![
            "std::iter",
            "std::iter::once",
            "std::iter::repeat",
            "std::string::String",
        ]
    );
}

#[test]
fn test_flat_multi_crate() {
    let imports = &[
        Import(&["std", "string", "String"]),
        Import(&["foo", "string", "String"]),
        Import(&["bar", "string", "String"]),
    ];

    assert_eq!(
        format_flat(imports, Order::Original),
        vec!["std::string::String", "foo::string::String", "bar::string::String",]
    );

    assert_eq!(
        format_flat(imports, Order::Sorted),
        vec!["bar::string::String", "foo::string::String", "std::string::String",]
    );
}

#[test]
fn test_nested_empty() {
    let imports = &[];

    assert_eq!(format_nested(imports, Order::Original), Vec::<String>::new());
    assert_eq!(format_nested(imports, Order::Sorted), Vec::<String>::new());
}

#[test]
fn test_nested_only_crate() {
    let imports = &[
        Import(&["my_crate"]),
    ];

    assert_eq!(format_nested(imports, Order::Original), vec!["my_crate\n"]);
    assert_eq!(format_nested(imports, Order::Sorted), vec!["my_crate\n"]);
}

#[test]
fn test_nested_basic() {
    let imports = &[Import(&["std", "a"])];

    assert_eq!(format_nested(imports, Order::Original), vec!["std::{\n    a,\n}\n"]);
    assert_eq!(format_nested(imports, Order::Sorted), vec!["std::{\n    a,\n}\n"]);
}

#[test]
fn test_nested_deep() {
    let imports = &[Import(&["std", "a", "b", "c", "d"])];

    let expected = vec![concat!(
        "std::{\n",
        "    a::{\n",
        "        b::{\n",
        "            c::{\n",
        "                d,\n",
        "            },\n",
        "        },\n",
        "    },\n",
        "}\n",
    )];

    assert_eq!(format_nested(imports, Order::Original), expected);
    assert_eq!(format_nested(imports, Order::Sorted), expected);
}

#[test]
fn test_nested_original() {
    let imports = &[
        Import(&["my_crate", "c"]),
        Import(&["my_crate", "b", "B2"]),
        Import(&["my_crate", "a"]),
        Import(&["my_crate", "b", "B1"]),
    ];

    assert_eq!(
        format_nested(imports, Order::Original),
        vec![concat!(
            "my_crate::{\n",
            "    c,\n",
            "    b::{\n",
            "        B2,\n",
            "        B1,\n",
            "    },\n",
            "    a,\n",
            "}\n",
        )]
    )
}

#[test]
fn test_nested_original_2() {
    let imports = &[
        Import(&["my_crate", "c"]),
        Import(&["my_crate", "b", "B2"]),
        Import(&["my_crate", "a", "inner", "I1"]),
        Import(&["my_crate", "b", "B1"]),
        Import(&["my_crate", "a", "A1"]),
    ];

    assert_eq!(
        format_nested(imports, Order::Original),
        vec![concat!(
            "my_crate::{\n",
            "    c,\n",
            "    b::{\n",
            "        B2,\n",
            "        B1,\n",
            "    },\n",
            "    a::{\n",
            "        inner::{\n",
            "            I1,\n",
            "        },\n",
            "        A1,\n",
            "    },\n",
            "}\n",
        )]
    )
}

#[test]
fn test_nested_sorted() {
    let imports = &[
        Import(&["my_crate", "c"]),
        Import(&["my_crate", "b", "B2"]),
        Import(&["my_crate", "a"]),
        Import(&["my_crate", "b", "B1"]),
    ];

    assert_eq!(
        format_nested(imports, Order::Sorted),
        vec![concat!(
            "my_crate::{\n",
            "    a,\n",
            "    b::{\n",
            "        B1,\n",
            "        B2,\n",
            "    },\n",
            "    c,\n",
            "}\n",
        )]
    )
}

#[test]
fn test_nested_sorted_2() {
    let imports = &[
        Import(&["my_crate", "c"]),
        Import(&["my_crate", "b", "B2"]),
        Import(&["my_crate", "a", "inner", "I1"]),
        Import(&["my_crate", "b", "B1"]),
        Import(&["my_crate", "a", "A1"]),
    ];

    assert_eq!(
        format_nested(imports, Order::Sorted),
        vec![concat!(
            "my_crate::{\n",
            "    a::{\n",
            "        A1,\n",
            "        inner::{\n",
            "            I1,\n",
            "        },\n",
            "    },\n",
            "    b::{\n",
            "        B1,\n",
            "        B2,\n",
            "    },\n",
            "    c,\n",
            "}\n",
        )]
    )
}

#[test]
fn test_nested_original_duplicates() {
    let imports = &[
        Import(&["my_crate", "c"]),
        Import(&["my_crate", "b", "B2"]),
        Import(&["my_crate", "b", "B2"]),
        Import(&["my_crate", "a"]),
        Import(&["my_crate", "b", "B1"]),
        Import(&["my_crate", "c"]),
    ];

    assert_eq!(
        format_nested(imports, Order::Original),
        vec![concat!(
            "my_crate::{\n",
            "    c,\n",
            "    b::{\n",
            "        B2,\n",
            "        B1,\n",
            "    },\n",
            "    a,\n",
            "}\n",
        )]
    )
}

#[test]
fn test_nested_sorted_duplicates() {
    let imports = &[
        Import(&["my_crate", "c"]),
        Import(&["my_crate", "b", "B2"]),
        Import(&["my_crate", "b", "B2"]),
        Import(&["my_crate", "a"]),
        Import(&["my_crate", "b", "B1"]),
        Import(&["my_crate", "c"]),
    ];

    assert_eq!(
        format_nested(imports, Order::Sorted),
        vec![concat!(
            "my_crate::{\n",
            "    a,\n",
            "    b::{\n",
            "        B1,\n",
            "        B2,\n",
            "    },\n",
            "    c,\n",
            "}\n",
        )]
    )
}

#[test]
fn test_nested_original_self() {
    let imports = &[
        Import(&["my_crate", "c"]),
        Import(&["my_crate", "b", "B2"]),
        Import(&["my_crate", "a"]),
        Import(&["my_crate", "b", "B1"]),
        Import(&["my_crate", "b"]),
    ];

    assert_eq!(
        format_nested(imports, Order::Original),
        vec![concat!(
            "my_crate::{\n",
            "    c,\n",
            "    b::{\n",
            "        self,\n",
            "        B2,\n",
            "        B1,\n",
            "    },\n",
            "    a,\n",
            "}\n",
        )]
    )
}

#[test]
fn test_nested_sorted_self() {
    let imports = &[
        Import(&["my_crate", "c"]),
        Import(&["my_crate", "b", "B2"]),
        Import(&["my_crate", "a"]),
        Import(&["my_crate", "b", "B1"]),
        Import(&["my_crate", "b"]),
    ];

    assert_eq!(
        format_nested(imports, Order::Sorted),
        vec![concat!(
            "my_crate::{\n",
            "    a,\n",
            "    b::{\n",
            "        self,\n",
            "        B1,\n",
            "        B2,\n",
            "    },\n",
            "    c,\n",
            "}\n",
        )]
    )
}

#[test]
#[rustfmt::skip]
fn test_nested_original_multi_crate() {
    let imports = &[
        Import(&["crate", "b"]),
        Import(&["std", "string", "String"]),
        Import(&["crate", "a"]),
    ];

    assert_eq!(
        format_nested(imports, Order::Original),
        vec![
            concat!(
                "crate::{\n",
                "    b,\n",
                "    a,\n",
                "}\n",
            ),
            concat!(
                "std::{\n",
                "    string::{\n",
                "        String,\n",
                "    },\n",
                "}\n",
            ),
        ]
    )
}

#[test]
#[rustfmt::skip]
fn test_nested_sorted_multi_crate() {
    let imports = &[
        Import(&["crate", "b"]),
        Import(&["std", "string", "String"]),
        Import(&["crate", "a"]),
    ];

    assert_eq!(
        format_nested(imports, Order::Sorted),
        vec![
            concat!(
                "crate::{\n",
                "    a,\n",
                "    b,\n",
                "}\n",
            ),
            concat!(
                "std::{\n",
                "    string::{\n",
                "        String,\n",
                "    },\n",
                "}\n",
            ),
        ]
    )
}
