#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use dioxus::prelude::*;

#[inline_props]
pub fn SimpleInput<'a>(cx: Scope,
	label: String, name: String, value: String,
	onInput: EventHandler<'a, FormEvent>,
	class: Option<String>,
) -> Element<'a>
{
	let clazz = match class
	{
		Some(c) => c.into(),
		None => "row".to_string(),
	};
	
	return cx.render(rsx!
	{
		div
		{
			class: "{clazz}",
			label { r#for: "{name}", "{label}:" }
			input { r#type: "text", name: "{name}", value: "{value}", oninput: move |evt| onInput.call(evt) }
		}
	});
}
