use crate::documents::BuildXML;
use crate::xml_builder::*;

use super::run_property_default::*;

pub struct DocDefaults {
    run_property_default: RunPropertyDefault,
}

impl DocDefaults {
    pub fn new() -> DocDefaults {
        let run_property_default = RunPropertyDefault::new();
        DocDefaults {
            run_property_default,
        }
    }
}

impl BuildXML for DocDefaults {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let run_property_default = self.run_property_default.build();
        b.open_doc_defaults()
            .add_child_buffer(&run_property_default)
            .close()
            .build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::str;

    #[test]
    fn test_build() {
        let c = DocDefaults::new();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:docDefaults><w:rPrDefault /></w:docDefaults>"#
        );
    }
}
