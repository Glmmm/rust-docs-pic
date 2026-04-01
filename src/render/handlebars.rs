use handlebars::Handlebars;
use std::collections::BTreeMap;

fn render() {
    let mut handlebars = Handlebars::new();

    let source = "hello {{world}}";
    assert!(handlebars.register_template_string("t1", source).is_ok());

    // The data type should implements `serde::Serialize`
    let mut data = BTreeMap::new();
    data.insert("world".to_string(), "世界!".to_string());
    assert_eq!(handlebars.render("t1", &data).unwrap(), "hello 世界!");
}
