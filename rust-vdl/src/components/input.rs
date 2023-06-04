#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use dioxus::prelude::*;

const DefaultInputClass: &str = "inputRow";

#[inline_props]
pub fn InputRow<'a>(cx: Scope,
	value: String, onInput: EventHandler<'a, FormEvent>,
	class: Option<String>, placeholder: Option<String>, title: Option<String>,
) -> Element<'a>
{
	let clazz = match class
	{
		None => String::default(),
		Some(c) => c.into(),
	};
	
	let p = match placeholder
	{
		None => String::default(),
		Some(p) => p.into(),
	};
	
	let t = match title
	{
		None => String::default(),
		Some(t) => t.into(),
	};
	
	return cx.render(rsx!
	{
		div
		{
			class: "{clazz}",
			input
			{
				r#type: "text",
				title: "{t}",
				placeholder: "{p}",
				value: "{value}",
				oninput: move |evt| onInput.call(evt)
			}
		}
	});
}

#[inline_props]
pub fn LabelInputRow<'a>(cx: Scope,
	label: String, name: String, value: String, onInput: EventHandler<'a, FormEvent>,
	class: Option<String>, placeholder: Option<String>, title: Option<String>,
) -> Element<'a>
{
	let c = match class
	{
		None => DefaultInputClass.to_string(),
		Some(c) => c.into(),
	};
	
	let p = match placeholder
	{
		None => String::default(),
		Some(p) => p.into(),
	};
	
	let t = match title
	{
		None => String::default(),
		Some(t) => t.into(),
	};
	
	return cx.render(rsx!
	{
		div
		{
			class: "{c}",
			label { class: "labelInputLabel", r#for: "{name}", "{label}:" }
			input
			{
				r#type: "text",
				id: "{name}",
				name: "{name}",
				title: "{t}",
				placeholder: "{p}",
				value: "{value}",
				oninput: move |evt| onInput.call(evt)
			}
		}
	});
}

#[inline_props]
pub fn ToggleRow<'a>(cx: Scope,
	label: String, name: String, value: bool, onInput: EventHandler<'a, FormEvent>,
	class: Option<String>, title: Option<String>,
) -> Element<'a>
{
	let c = match class
	{
		None => DefaultInputClass.to_string(),
		Some(c) => c.into(),
	};
	
	let t = match title
	{
		None => String::default(),
		Some(t) => t.into(),
	};
	
	return cx.render(rsx!
	{
		div
		{
			class: "{c}",
			div { class: "toggleButtonLabel", "{label}:" }
			input
			{
				r#type: "checkbox",
				id: "{name}", 
				name: "{name}",
				checked: "{value}",
				class: "toggleButtonCheckbox toggleButton",
				value: "some",
				oninput: move |evt| onInput.call(evt)
			}
			label { r#for: "{name}", title: "{t}" }
		}
	});
}
