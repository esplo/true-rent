use wasm_bindgen::JsValue;
use web_sys::{Document, Element};
use web_sys::HtmlElement;
use web_sys::HtmlInputElement;
use web_sys::HtmlSelectElement;
use web_sys::HtmlTextAreaElement;
use crate::{HtmlError, InputError};
use crate::constants::InputId;
use wasm_bindgen::JsCast;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();
}

pub struct HtmlAttr<'a> {
    pub name: &'a str,
    pub value: &'a str,
}

pub fn make_tag(document: &Document, tag_name: &str,
                attr: Vec<HtmlAttr>, inner: Option<&str>,
                parent: Option<&Element>) -> Result<Element, JsValue> {
    let elem = document.create_element(tag_name)?;
    attr.into_iter().map(|a|
        elem.set_attribute(&a.name, &a.value)
    ).collect::<Result<_, _>>()?;
    if let Some(i) = inner {
        elem.set_inner_html(i);
    }
    if let Some(p) = parent {
        p.append_child(&elem)?;
    }
    Ok(elem)
}

pub fn fetch_value<T>(document: &Document, id_key: InputId) -> Result<T, JsValue>
    where T: std::str::FromStr
{
    let id: &str = &id_key.to_string();
    let form_place_raw = document
        .get_element_by_id(id)
        .ok_or_else(|| HtmlError::CannotGetElement(id.to_string()))?;
    let form_place = form_place_raw.dyn_ref::<HtmlElement>()
        .unwrap_or_else(|| panic!("{} is not a HtmlElement", id));

    fn convert_specialized_element(form_place: &HtmlElement) -> Option<(String, bool)> {
        form_place.dyn_ref::<HtmlInputElement>().map(|e| (e.value(), e.report_validity()))
            .or_else(|| form_place.dyn_ref::<HtmlSelectElement>().map(|e| (e.value(), e.report_validity())))
            .or_else(|| form_place.dyn_ref::<HtmlTextAreaElement>().map(|e| (e.value(), e.report_validity())))
    }
    let html_form =
        convert_specialized_element(&form_place)
            .unwrap_or_else(|| panic!("{} is neither HtmlInputElement or HtmlSelectElement", id));

    let result = if html_form.1 {
        html_form.0
            .parse::<T>()
            .map_err(|_e| InputError::CannotParse(id.to_string()))
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
    } else {
        Err(InputError::CannotParse(id.to_string()))
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
    };
    result.map_err(|err| JsValue::from(js_sys::Error::new(&format!("{:?}", err))))
}