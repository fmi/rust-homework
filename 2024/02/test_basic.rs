use solution::*;

#[test]
fn test_basic() {
    let imports = &[
        Import(&["my_crate", "a"]),
        Import(&["my_crate", "b", "B1"]),
        Import(&["my_crate", "b", "B2"]),
        Import(&["my_crate", "c"]),
    ];

    assert_eq!(
        format_flat(imports, Order::Sorted),
        &[
            "my_crate::a",
            "my_crate::b::B1",
            "my_crate::b::B2",
            "my_crate::c",
        ]
    );

    assert_eq!(
        format_nested(imports, Order::Sorted),
        &[
            concat!(
                "my_crate::{\n",
                "    a,\n",
                "    b::{\n",
                "        B1,\n",
                "        B2,\n",
                "    },\n",
                "    c,\n",
                "}\n",
            )
        ]
    );
}
