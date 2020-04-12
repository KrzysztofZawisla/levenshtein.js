extern crate levenshtein;
extern crate neon;

use levenshtein::levenshtein;
use neon::prelude::*;

fn filter(mut cx: FunctionContext) -> JsResult<JsArray> {
    let input = cx.argument::<JsString>(0)?.value();
    let distance = cx.argument::<JsNumber>(1)?.value() as usize;
    let collection = cx.argument::<JsArray>(2)?;
    let collection_as_vec: Vec<Handle<JsValue>> = collection.to_vec(&mut cx)?;
    let mut temporary_vec = vec![];
    for value in collection_as_vec.iter() {
        let value = value.downcast::<JsString>().or_throw(&mut cx)?.value();
        let counted_distance = levenshtein(&input, &value);
        if counted_distance <= distance {
            temporary_vec.push(value);
        }
    }
    let return_array = JsArray::new(&mut cx, temporary_vec.len() as u32);
    for (i, value) in temporary_vec.iter().enumerate() {
        let js_value = cx.string(value);
        return_array.set(&mut cx, i as u32, js_value).unwrap();
    }
    Ok(return_array)
}

register_module!(mut cx, { cx.export_function("filter", filter) });
