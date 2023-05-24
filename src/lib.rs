use serde_json::Value;

pub fn get_last_segment(blocks: &Value) -> Option<&Value> {
    let prompt_left_blocks = blocks
        .as_array()?
        .iter()
        .filter(|block| block["alignment"] == "left" || block["type"] == "prompt")
        .map(|block| &block["segments"]);
    prompt_left_blocks.last()?.as_array()?.last()
}

pub fn alter_omp_json(json: &mut Value) {
    let last_segment = get_last_segment(&json["blocks"])
        .expect("Cannot find last segment for given ")
        .clone();
    json.as_object_mut()
        .unwrap()
        .insert("transient_prompt".to_owned(), last_segment);
}
