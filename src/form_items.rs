use std::fmt;

use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use web_sys::Document;
use web_sys::Element;

use crate::constants::UnitId;
use crate::utils::{HtmlAttr, make_tag};

#[wasm_bindgen]
pub fn toggle(document: &Document, id: &str) {
    let detail_palce = document
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("should have #{} on the page", id));
    let html_detail_palce = detail_palce
        .dyn_ref::<web_sys::HtmlElement>()
        .unwrap_or_else(|| panic!("#{} should be an `HtmlElement`", id));

    let current_display = html_detail_palce
        .style()
        .get_property_value("display")
        .unwrap_or_else(|_| panic!("cannot get display for #{}", id));
    let next_value = match current_display.as_str() {
        "none" => "initial",
        "initial" => "none",
        _ => "none",
    };
    html_detail_palce
        .style()
        .set_property("display", next_value)
        .unwrap_or_else(|_| panic!("cannot set display for #{}", id));
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct HtmlId(String);

impl fmt::Display for HtmlId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<String> for HtmlId {
    fn into(self) -> String {
        self.0
    }
}

pub trait HtmlItem {
    fn to_html(&self, document: &Document) -> Result<Vec<Element>, JsValue>;
}

pub struct Forms {
    items: Vec<RentItem>,
}

impl Forms {
    pub fn new(items: Vec<RentItem>) -> Forms {
        Forms { items }
    }
}

impl HtmlItem for Forms {
    fn to_html(&self, document: &Document) -> Result<Vec<Element>, JsValue> {
        let t: Result<Vec<Vec<Element>>, JsValue> = self
            .items
            .iter()
            .map(|item| item.to_html(&document))
            .collect();
        t.map(|r| r.into_iter().flatten().collect())
    }
}

pub struct RentItem {
    label: LabelForInputItem,
    input: InputItem,
    unit_info: (Vec<UnitId>, HtmlId),
    detail: DetailForInputItem,
}

impl HtmlItem for RentItem {
    fn to_html(&self, document: &Document) -> Result<Vec<Element>, JsValue> {
        let input_wrapper = make_tag(&document, "div",
                                     vec![HtmlAttr { name: "class", value: "form-row" }],
                                     None, None,
        )?;

        let input = self.input.to_html(document)?;
        input
            .into_iter()
            .map(|i| input_wrapper.append_child(&i))
            .collect::<Result<Vec<_>, JsValue>>()?;

        // add unit form
        {
            let unit_wrapper = make_tag(&document, "div",
                                        vec![HtmlAttr { name: "class", value: "form-group col-md-6" }],
                                        None, Some(&input_wrapper),
            )?;

            let unit_selector = make_tag(&document, "select",
                                         vec![
                                             HtmlAttr { name: "id", value: &self.unit_info.1.to_string() },
                                             HtmlAttr { name: "class", value: "custom-select" },
                                         ],
                                         None, Some(&unit_wrapper))?;

            self.unit_info.0.iter().map(|e|
                make_tag(&document, "option",
                         vec![HtmlAttr { name: "value", value: &format!("{}", *e as i32) }],
                         Some(e.to_string()), Some(&unit_selector),
                ))
                .collect::<Result<Vec<_>, _>>()?;
        }

        let detail = self.detail.to_html(document)?;
        let label = self.label.to_html(document)?;

        Ok(label
            .into_iter()
            .chain(detail.into_iter())
            .chain(vec![input_wrapper])
            .collect())
    }
}

pub struct InputValidation {
    pub default_value: Option<u32>,
    pub min: Option<u32>,
    pub required: Option<bool>,
}

impl Default for InputValidation {
    fn default() -> Self {
        Self {
            default_value: Some(0),
            min: Some(0),
            required: Some(true),
        }
    }
}

impl RentItem {
    pub fn new(
        item_text: &str,
        unit: Vec<UnitId>,
        unit_id_raw: &str,
        detail_text: &str,
        input_id_raw: &str,
        input_validation: InputValidation,
    ) -> Self {
        let unit_id = HtmlId(unit_id_raw.to_string());
        let input_id = HtmlId(input_id_raw.to_string());
        let detail_id = HtmlId(format!("{}_detail", input_id_raw));
        RentItem {
            label: LabelForInputItem {
                text: item_text.to_string(),
                input_id: input_id.clone(),
                detail_id: detail_id.clone(),
            },
            input: InputItem {
                id: input_id,
                input_validation,
                detail_id: detail_id.clone(),
            },
            unit_info: (unit, unit_id),
            detail: DetailForInputItem {
                id: detail_id,
                text: detail_text.to_string(),
            },
        }
    }
}

struct LabelForInputItem {
    text: String,
    input_id: HtmlId,
    detail_id: HtmlId,
}

impl HtmlItem for LabelForInputItem {
    fn to_html(&self, document: &Document) -> Result<Vec<Element>, JsValue> {
        let label = make_tag(&document, "label",
                             vec![HtmlAttr { name: "for", value: &format!("{}", self.input_id) }],
                             None, None)?;

        make_tag(&document, "span", vec![
            HtmlAttr { name: "class", value: "font-weight-bold" }
        ], Some(&self.text), Some(&label))?;

        let info_wrapper = make_tag(&document, "span", vec![], Some(" "), Some(&label))?;

        {
            let detail_id = format!("{}", self.detail_id);
            let document = document.clone();

            let closure =
                Closure::wrap(
                    Box::new(move |_: web_sys::MouseEvent| toggle(&document, &detail_id))
                        as Box<dyn FnMut(_)>,
                );
            info_wrapper
                .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        make_tag(&document, "i",
                 vec![
                     HtmlAttr { name: "class", value: "fas fa-info-circle" },
                 ],
                 None, Some(&info_wrapper))?;
        make_tag(&document, "span", vec![],
                 Some(": "), Some(&label))?;

        Ok(vec![label])
    }
}

struct InputItem {
    id: HtmlId,
    detail_id: HtmlId,
    input_validation: InputValidation,
}

impl HtmlItem for InputItem {
    fn to_html(&self, document: &Document) -> Result<Vec<Element>, JsValue> {
        let wrapper = make_tag(&document, "div",
                               vec![HtmlAttr { name: "class", value: "form-group col-md-6" }],
                               None, None)?;

        {
            let input = make_tag(&document, "input",
                                 vec![
                                     HtmlAttr { name: "type", value: "number" },
                                     HtmlAttr { name: "name", value: &format!("{}", &self.id) },
                                     HtmlAttr { name: "class", value: "form-control" },
                                     HtmlAttr { name: "aria-describedby", value: &format!("{}", &self.detail_id) },
                                     HtmlAttr { name: "id", value: &format!("{}", &self.id) }
                                 ], None, Some(&wrapper))?;
            let input: web_sys::HtmlInputElement = input.dyn_into::<web_sys::HtmlInputElement>()?;

            if let Some(v) = self.input_validation.default_value {
                input.set_default_value(&format!("{}", v));
            }
            if let Some(v) = self.input_validation.min {
                input.set_attribute("min", &format!("{}", &v))?;
            }
            if self.input_validation.required.is_some() {
                input.set_attribute("required", &"")?;
            }
        }
        Ok(vec![wrapper])
    }
}

struct DetailForInputItem {
    id: HtmlId,
    text: String,
}

impl HtmlItem for DetailForInputItem {
    fn to_html(&self, document: &Document) -> Result<Vec<Element>, JsValue> {
        let small = make_tag(&document, "small",
                             vec![
                                 HtmlAttr { name: "id", value: &format!("{}", &self.id) },
                                 HtmlAttr { name: "style", value: "display: none;" },
                                 HtmlAttr { name: "class", value: "form-text text-muted" }
                             ], Some(&self.text), None)?;

        Ok(vec![small])
    }
}
