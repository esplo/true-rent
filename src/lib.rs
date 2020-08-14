use std::string::ToString;

use thiserror::Error;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::*;

use calculator::*;

use crate::constants::InputId;
use crate::form_items::HtmlItem;
use crate::utils::{HtmlAttr, make_tag};
use crate::utils::fetch_value;

mod calculator;
mod constants;
mod form_info;
mod form_items;
mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RentInfo {
    rent: i32,
}


#[derive(Error, Debug)]
pub enum HtmlError {
    #[error("should have #{0} on the page")]
    CannotGetElement(String),
}

impl From<HtmlError> for JsValue {
    fn from(w: HtmlError) -> JsValue {
        JsValue::from(js_sys::Error::new(&format!("{:?}", w)))
    }
}

#[derive(Error, Debug)]
pub enum InputError {
    #[error("Cannot Parse value #{0}")]
    CannotParse(String),
}


#[wasm_bindgen]
pub fn calc(document: &Document) -> Result<(), JsValue> {
    let rent_elem = RentElements::build(document)?;

    {
        let export_form = document
            .get_element_by_id(&InputId::JsonExportTextArea.to_string())
            .unwrap_or_else(|| panic!("should have #{} on the page", InputId::JsonExportTextArea.to_string()));
        let form_place = export_form
            .dyn_ref::<web_sys::HtmlTextAreaElement>()
            .unwrap_or_else(|| panic!("#{} should be an `HtmlTextareaElement`", InputId::JsonExportTextArea.to_string()));

        form_place.set_inner_html(&rent_elem.to_json_string());
    }

    let calculator = RentCalculator {};
    let result = calculator.calc(rent_elem)?;

    // write result title
    {
        let form_place = document
            .get_element_by_id(&InputId::CalcResultTitle.to_string())
            .expect("should have #HtmlAreaId::CalcResult on the page");
        let html_form_place = form_place
            .dyn_ref::<web_sys::HtmlElement>()
            .expect("#forms should be an `HtmlElement`");
        html_form_place.set_inner_html(&format!(
            "実質家賃: {}円/月",
            result.monthly
        ));
    }
    {
        let form_place = document
            .get_element_by_id(&InputId::CalcResultBody.to_string())
            .expect("should have #HtmlAreaId::CalcResult on the page");
        let html_form_place = form_place
            .dyn_ref::<web_sys::HtmlElement>()
            .expect("#forms should be an `HtmlElement`");
        html_form_place.set_inner_html(&format!(
            "{}か月住むと平均 {}円/月 で、トータル{}円の支払いです。\
            家賃+管理費のみだと{}円/月でトータル{}円の支払いでした。\
            実質家賃との差は{}円/月です",
            result.lease_period, result.monthly, result.total, result.original_monthly, result.original_total,
            result.monthly - result.original_monthly
        ));
    }
    Ok(())
}


#[wasm_bindgen]
pub fn restore(document: &Document) -> Result<(), JsValue> {
    let input_json = fetch_value::<String>(&document, InputId::JsonImportTextArea)?;
    serde_json::from_str(&input_json)
        .map_err(|e| JsValue::from(&format!("{}", e)))
        .map(|_: RentElements| {
            unimplemented!("sorry")
        })
}


fn construct() -> Result<Node, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let form_place = document
        .get_element_by_id("forms")
        .expect("should have #forms on the page");

    let html_form_place = form_place
        .dyn_ref::<HtmlElement>()
        .expect("#forms should be an `HtmlElement`");

    let forms = form_info::create_form();
    let form_html = forms.to_html(&document)?;

    let whole_wrapper = make_tag(&document, "div",
                                 vec![HtmlAttr { name: "name", value: "wrapper" }],
                                 None, Some(&html_form_place))?;

    make_tag(&document, "h1", vec![],
             Some("実質家賃計算機"), Some(&whole_wrapper))?;

    let form_wrapper = make_tag(&document, "div",
                                vec![
                                    HtmlAttr { name: "name", value: "form-wrapper" },
                                    HtmlAttr { name: "class", value: "container" }
                                ], None, Some(&whole_wrapper))?;

    form_html
        .iter()
        .map(|item| form_wrapper.append_child(item))
        .collect::<Result<Vec<_>, JsValue>>()?;

    let calc_button_wrapper = make_tag(&document, "div", vec![
        HtmlAttr { name: "class", value: "calc-button" },
    ], None, Some(&whole_wrapper))?;
    let calc_button = make_tag(&document, "button",
                               vec![
                                   HtmlAttr { name: "type", value: "button" },
                                   HtmlAttr { name: "class", value: "btn btn-primary btn-lg btn-block" },
                               ], Some("計算する"), Some(&calc_button_wrapper))?;

    {
        let document = document.clone();
        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            if let Err(e) = calc(&document) {
                console::error_1(&e);
            }
        }) as Box<dyn FnMut(_)>);
        calc_button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let result_card_wrapper = make_tag(&document, "div",
                                           vec![HtmlAttr { name: "class", value: "card border-primary mb-3" }],
                                           None, Some(&whole_wrapper))?;
        make_tag(&document, "div",
                 vec![HtmlAttr { name: "class", value: "card-header" }],
                 Some("計算結果"), Some(&result_card_wrapper))?;
        let card_body = make_tag(&document, "div",
                                 vec![HtmlAttr { name: "class", value: "card-body text-primary" }],
                                 None, Some(&result_card_wrapper))?;
        make_tag(&document, "h5",
                 vec![
                     HtmlAttr { name: "class", value: "card-title" },
                     HtmlAttr { name: "id", value: &InputId::CalcResultTitle.to_string() },
                 ],
                 Some("ここに計算結果が出ます"), Some(&card_body))?;
        make_tag(&document, "p",
                 vec![
                     HtmlAttr { name: "class", value: "card-text" },
                     HtmlAttr { name: "id", value: &InputId::CalcResultBody.to_string() },
                 ],
                 Some("結果の説明など"), Some(&card_body))?;
    }

    make_tag(&document, "hr", vec![], None, Some(&whole_wrapper))?;

    make_tag(&document, "label",
             vec![HtmlAttr { name: "for", value: &InputId::JsonExportTextArea.to_string() }],
             Some("Export （入力値を保存したい場合はこちらをコピペしてください）"), Some(&whole_wrapper))?;

    make_tag(&document, "textarea",
             vec![
                 HtmlAttr { name: "id", value: &InputId::JsonExportTextArea.to_string() },
                 HtmlAttr { name: "class", value: "form-control" },
                 HtmlAttr { name: "rows", value: "3" },
                 HtmlAttr { name: "wrap", value: "soft" }
             ], None, Some(&whole_wrapper))?;

    make_tag(&document, "label",
             vec![HtmlAttr { name: "for", value: &InputId::JsonImportTextArea.to_string() }],
             Some("Import （入力値を復元したい場合はこちらにペーストして復元ボタンを押してください）"), Some(&whole_wrapper))?;

    make_tag(&document, "textarea",
             vec![
                 HtmlAttr { name: "id", value: &InputId::JsonImportTextArea.to_string() },
                 HtmlAttr { name: "class", value: "form-control" },
                 HtmlAttr { name: "rows", value: "3" },
                 HtmlAttr { name: "wrap", value: "soft" },
                 HtmlAttr { name: "placeholder", value: "開発中です" },
                 HtmlAttr { name: "readonly", value: "" },
             ], None, Some(&whole_wrapper))?;
    let restore_button_wrapper = make_tag(&document, "div", vec![], None, Some(&whole_wrapper))?;
    let restore_button = make_tag(&document, "button",
                                  vec![
                                      HtmlAttr { name: "type", value: "button" },
                                      HtmlAttr { name: "class", value: "btn btn-primary" },
                                      HtmlAttr { name: "disabled", value: "" },
                                  ], Some("復元する"), Some(&restore_button_wrapper))?;

    {
        let document = document.clone();
        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            if let Err(e) = restore(&document) {
                console::error_1(&e);
            }
        }) as Box<dyn FnMut(_)>);
        restore_button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    make_footer(&document, &whole_wrapper)?;

    Ok(form_place.into())
}

fn make_footer(document: &Document, parent: &Element) -> Result<Element, JsValue> {
    make_tag(&document, "hr", vec![], None, Some(&parent))?;
    let footer = make_tag(&document, "footer",
                          vec![
                              HtmlAttr { name: "class", value: "text-muted" },
                          ], None, Some(&parent))?;
    let container = make_tag(&document, "div",
                             vec![
                                 HtmlAttr { name: "class", value: "container-fluid" },
                             ], None, Some(&footer))?;
    let right = make_tag(&document, "p",
                         vec![
                             HtmlAttr { name: "class", value: "float-right" },
                         ], None, Some(&container))?;
    make_tag(&document, "a",
             vec![
                 HtmlAttr { name: "href", value: "#" },
             ], Some("上に戻る"), Some(&right))?;
    let message = make_tag(&document, "p",
                           vec![], None, Some(&container))?;
    make_tag(&document, "span",
             vec![], Some("made with ♥ by esplo: "), Some(&message))?;

    {
        let sns = make_tag(&document, "a",
                           vec![
                               HtmlAttr { name: "href", value: "https://twitter.com/esplo77" },
                           ], None, Some(&message))?;
        make_tag(&document, "i",
                 vec![
                     HtmlAttr { name: "class", value: "fa fa-twitter" },
                 ], None, Some(&sns))?;
    }

    {
        let sns = make_tag(&document, "a",
                           vec![
                               HtmlAttr { name: "href", value: "https://github.com/esplo/true-rent" },
                           ], None, Some(&message))?;
        make_tag(&document, "i",
                 vec![
                     HtmlAttr { name: "class", value: "fa fa-github" },
                 ], None, Some(&sns))?;
    }

    Ok(footer)
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    utils::set_panic_hook();
    construct().map(|_| ())
}
