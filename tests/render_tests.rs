#[cfg(test)]

mod render_tests {
    use std::{
        collections::BTreeMap,
        fmt::write,
        io::{BufWriter, stdout},
    };

    use handlebars::Handlebars;
    use toml::ser::Buffer;

    #[test]
    fn render() {
        let mut handlebars = Handlebars::new();

        let source = "hello {{world}}, {{banana}}";
        assert!(handlebars.register_template_string("t1", source).is_ok());

        let mut data = BTreeMap::new();
        data.insert("world".to_string(), "世界!".to_string());
        //TODO PROBLEMA GRANDE AQUI
        data.insert("banana".to_string(), "B->C".to_string());
        assert_eq!(
            handlebars.render_to_write("t1", &data).unwrap(),
            "hello 世界!, B->C"
        );
    }
}
