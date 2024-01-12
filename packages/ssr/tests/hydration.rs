use dioxus::prelude::*;

#[test]
fn root_ids() {
    fn app(_: ()) -> Element {
        render! { div { width: "100px" } }
    }

    let mut dom = VirtualDom::new(app);
    _ = dom.rebuild(&mut dioxus_core::NoOpMutations);

    assert_eq!(
        dioxus_ssr::pre_render(&dom),
        r#"<div style="width:100px;" data-node-hydration="0"></div>"#
    );
}

#[test]
fn dynamic_attributes() {
    fn app(_: ()) -> Element {
        let dynamic = 123;
        render! {
            div { width: "100px", div { width: "{dynamic}px" } }
        }
    }

    let mut dom = VirtualDom::new(app);
    _ = dom.rebuild(&mut dioxus_core::NoOpMutations);

    assert_eq!(
        dioxus_ssr::pre_render(&dom),
        r#"<div style="width:100px;" data-node-hydration="0"><div style="width:123px;" data-node-hydration="1"></div></div>"#
    );
}

#[test]
fn listeners() {
    fn app(_: ()) -> Element {
        render! {
            div { width: "100px", div { onclick: |_| {} } }
        }
    }

    let mut dom = VirtualDom::new(app);
    _ = dom.rebuild(&mut dioxus_core::NoOpMutations);

    assert_eq!(
        dioxus_ssr::pre_render(&dom),
        r#"<div style="width:100px;" data-node-hydration="0"><div data-node-hydration="1,click:1"></div></div>"#
    );

    fn app2(_: ()) -> Element {
        let dynamic = 123;
        render! {
            div { width: "100px", div { width: "{dynamic}px", onclick: |_| {} } }
        }
    }

    let mut dom = VirtualDom::new(app2);
    _ = dom.rebuild(&mut dioxus_core::NoOpMutations);

    assert_eq!(
        dioxus_ssr::pre_render(&dom),
        r#"<div style="width:100px;" data-node-hydration="0"><div style="width:123px;" data-node-hydration="1,click:1"></div></div>"#
    );
}

#[test]
fn text_nodes() {
    fn app(_: ()) -> Element {
        let dynamic_text = "hello";
        render! {
            div { {dynamic_text} }
        }
    }

    let mut dom = VirtualDom::new(app);
    _ = dom.rebuild(&mut dioxus_core::NoOpMutations);

    assert_eq!(
        dioxus_ssr::pre_render(&dom),
        r#"<div data-node-hydration="0"><!--node-id1-->hello<!--#--></div>"#
    );

    fn app2(_: ()) -> Element {
        let dynamic = 123;
        render! {
            div { "{dynamic}", "{1234}" }
        }
    }

    let mut dom = VirtualDom::new(app2);
    _ = dom.rebuild(&mut dioxus_core::NoOpMutations);

    assert_eq!(
        dioxus_ssr::pre_render(&dom),
        r#"<div data-node-hydration="0"><!--node-id1-->123<!--#--><!--node-id2-->1234<!--#--></div>"#
    );
}

#[allow(non_snake_case)]
#[test]
fn components_hydrate() {
    fn app(_: ()) -> Element {
        render! { Child {} }
    }

    fn Child(_: ()) -> Element {
        render! { div { "hello" } }
    }

    let mut dom = VirtualDom::new(app);
    _ = dom.rebuild(&mut dioxus_core::NoOpMutations);

    assert_eq!(
        dioxus_ssr::pre_render(&dom),
        r#"<div data-node-hydration="0">hello</div>"#
    );

    fn app2(_: ()) -> Element {
        render! { Child2 {} }
    }

    fn Child2(_: ()) -> Element {
        let dyn_text = "hello";
        render! {
            div { {dyn_text} }
        }
    }

    let mut dom = VirtualDom::new(app2);
    _ = dom.rebuild(&mut dioxus_core::NoOpMutations);

    assert_eq!(
        dioxus_ssr::pre_render(&dom),
        r#"<div data-node-hydration="0"><!--node-id1-->hello<!--#--></div>"#
    );

    fn app3(_: ()) -> Element {
        render! { Child3 {} }
    }

    fn Child3(_: ()) -> Element {
        render! { div { width: "{1}" } }
    }

    let mut dom = VirtualDom::new(app3);
    _ = dom.rebuild(&mut dioxus_core::NoOpMutations);

    assert_eq!(
        dioxus_ssr::pre_render(&dom),
        r#"<div style="width:1;" data-node-hydration="0"></div>"#
    );

    fn app4(_: ()) -> Element {
        render! { Child4 {} }
    }

    fn Child4(_: ()) -> Element {
        render! {
            for _ in 0..2 {
                {render! { "{1}" }}
            }
        }
    }

    let mut dom = VirtualDom::new(app4);
    _ = dom.rebuild(&mut dioxus_core::NoOpMutations);

    assert_eq!(
        dioxus_ssr::pre_render(&dom),
        r#"<!--node-id0-->1<!--#--><!--node-id1-->1<!--#-->"#
    );
}

#[test]
fn hello_world_hydrates() {
    use dioxus_signals::use_signal;

    fn app(_: ()) -> Element {
        let mut count = use_signal(|| 0);

        render! {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }

    let mut dom = VirtualDom::new(app);
    _ = dom.rebuild(&mut dioxus_core::NoOpMutations);

    assert_eq!(
        dioxus_ssr::pre_render(&dom),
        r#"<h1 data-node-hydration="0"><!--node-id1-->High-Five counter: 0<!--#--></h1><button data-node-hydration="2,click:1">Up high!</button><button data-node-hydration="3,click:1">Down low!</button>"#
    );
}
